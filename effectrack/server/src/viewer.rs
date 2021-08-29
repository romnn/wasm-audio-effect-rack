use crate::cli::Config;
use crate::session::Session;
use crate::{EffectRack, ViewerUpdateMsg, INSTANCE_ID_KEY, SESSION_TOKEN_KEY};
#[cfg(feature = "analyze")]
use anyhow::Result;
use futures::{Stream};
use std::marker::Send;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tokio::time;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct Viewer<VU> {
    connection: Arc<RwLock<mpsc::Sender<Result<VU, Status>>>>,
    pub tx: mpsc::Sender<Result<VU, String>>,
    pub alive: Arc<RwLock<bool>>,

    pub config: Config,
    pub session_token: proto::grpc::SessionToken,
    pub instance_id: proto::grpc::InstanceId,
}

impl<VU> Viewer<VU> {
    pub async fn send(&self, message: Result<VU, String>) {
        if *self.alive.read().await {
            if let Err(err) = self.tx.send(message).await {
                eprintln!(
                    "failed to send to instance {} in session {}: {}",
                    self.instance_id, self.session_token, err
                );
            }
        }
    }
    pub fn start(&self) {}
}

impl<CU> EffectRack<ViewerUpdateMsg, CU>
where
    CU: Clone + Send + 'static,
{
    async fn new_viewer_instance(
        &self,
        session_token: proto::grpc::SessionToken,
    ) -> Result<proto::grpc::InstanceId, Status> {
        println!("[viewer] new instance id for session: {}", session_token);
        let mut sessions = self.sessions.write().await;
        let viewers = sessions
            .entry(session_token.clone())
            .or_insert(Session::new(self.config.clone()))
            .viewers
            .write()
            .await;
        let viewer_count = viewers.len();
        if let Some(max_viewers) = self.config.run.max_viewers {
            if viewer_count >= max_viewers {
                return Err(Status::unavailable(format!(
                    "maximum number of viewers ({}) exceeded",
                    max_viewers
                )));
            }
        }
        println!("session {} has {} viewers", session_token, viewer_count);
        let instance_id = (move || {
            for candidate in 1..viewer_count + 2 {
                let id = proto::grpc::InstanceId {
                    id: candidate.to_string(),
                };
                if !viewers.contains_key(&id) {
                    // insert here
                    return Ok(id);
                }
            }
            Err(Status::internal("failed to generate instance id"))
        })();
        instance_id
    }
}

#[tonic::async_trait]
impl<CU> proto::grpc::remote_viewer_server::RemoteViewer for EffectRack<ViewerUpdateMsg, CU>
where
    CU: Send + Clone + 'static,
{
    type ConnectStream = Pin<
        Box<dyn Stream<Item = Result<proto::grpc::ViewerUpdate, Status>> + Send + Sync + 'static>,
    >;

    async fn update_subscription(
        &self,
        request: Request<proto::grpc::UpdateSubscriptionRequest>,
    ) -> Result<Response<proto::grpc::Empty>, Status> {
        let (session_token, instance_id) = Self::extract_session_instance(&request).await?;
        // todo:
        println!("update subscription: {} {}", session_token, instance_id);
        Ok(Response::new(proto::grpc::Empty {}))
    }

    async fn disconnect(
        &self,
        request: Request<proto::grpc::ViewerDisconnectRequest>,
    ) -> Result<Response<proto::grpc::Empty>, Status> {
        let (session_token, instance_id) = Self::extract_session_instance(&request).await?;
        self.sessions
            .read()
            .await
            .get(&session_token)
            .ok_or(Status::not_found(
                format!("session {} does not exist", session_token),
            ))?
            .viewers
            .write()
            .await
            .remove(&instance_id)
            .ok_or(Status::not_found(
                format!("viewer instance {} does not exist", instance_id),
            ))?;
        println!("[viewer] disconnect: {} {}", session_token, instance_id);
        Ok(Response::new(proto::grpc::Empty {}))
    }

    async fn connect(
        &self,
        request: Request<proto::grpc::ViewerConnectRequest>,
    ) -> Result<Response<Self::ConnectStream>, Status> {
        let session_token = match Self::extract_session_token(&request).await {
            Ok(token) => Ok(token),
            Err(_) => self.new_session().await,
        }?;

        let instance_id = Self::extract_instance_id(&request)
            .await
            .or(self.new_viewer_instance(session_token.clone()).await)?;

        println!("[viewer] connect: {} {}", session_token, instance_id);
        let (stream_tx, stream_rx) = mpsc::channel(1);
        let pinned_stream = Box::pin(ReceiverStream::new(stream_rx));
        let mut response: Response<Self::ConnectStream> = Response::new(pinned_stream);
        let metadata = response.metadata_mut();
        metadata.insert(
            SESSION_TOKEN_KEY,
            session_token.clone().token.parse().unwrap(),
        );
        metadata.insert(INSTANCE_ID_KEY, instance_id.clone().id.parse().unwrap());
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .entry(session_token.clone())
            .or_insert(Session::new(self.config.clone()));
        let (tx, mut rx) = mpsc::channel(1);
        if let Some(existing) = session.viewers.read().await.get(&instance_id) {
            println!(
                "instance {} in session {} reconnected",
                instance_id, session_token
            );
            let existing = existing.read().await;
            let mut old_connection = existing.connection.write().await;
            *old_connection = stream_tx.clone();
            println!("exiting");
            return Ok(response);
        }

        println!(
            "instance {} in session {} connected",
            instance_id, session_token
        );
        let viewer = Viewer {
            connection: Arc::new(RwLock::new(stream_tx.clone())),
            tx,
            alive: Arc::new(RwLock::new(true)),
            config: self.config.clone(),
            session_token: session_token.clone(),
            instance_id: instance_id.clone(),
        };
        let stream_tx = viewer.connection.clone();
        let alive_state = viewer.alive.clone();
        session
            .viewers
            .write()
            .await
            .insert(instance_id.clone(), RwLock::new(viewer));
        let viewers = session.viewers.clone();
        let mut shutdown_rx = self.shutdown_rx.clone();
        let config = self.config.clone();
        tokio::spawn(async move {
            // assign the instance
            let assignment = proto::grpc::ViewerUpdate {
                update: Some(proto::grpc::viewer_update::Update::Assignment(
                    proto::grpc::Assignment {
                        session_token: Some(session_token.clone()),
                        instance_id: Some(instance_id.clone()),
                    },
                )),
            };

            let _ = stream_tx.read().await.send(Ok(assignment)).await;

            // wait for either shutdown, heartbeat or an update to send out
            let mut seq = 0u64;
            // todo: counter for failed send events
            let heartbeat_interval = time::sleep(Duration::from_millis(5 * 1000));
            let mut alive = true;
            let mut last_heartbeat = tokio::time::Instant::now();
            let mut heartbeat_timer = Box::pin(heartbeat_interval);
            let mut shutdown_signal = Box::pin(shutdown_rx.changed());
            loop {
                let was_alive = alive;
                tokio::select! {
                    _ = &mut shutdown_signal => {
                        println!("shutdown from open connection");
                        return;
                    }
                    received = rx.recv() => {
                        if let Some(received) = received {
                        match received {
                            Ok(update) => {
                                alive = stream_tx.read().await.send(Ok(update)).await.is_ok();

                            }
                            Err(err) => {
                                alive = stream_tx.read().await.send(Err(Status::internal(err))).await.is_ok();
                            }
                        }
                        }
                    }
                    _ = &mut heartbeat_timer => {
                        // reset the heartbeat
                        let heartbeat_interval = time::sleep(Duration::from_millis(5 * 1000));
                        heartbeat_timer = Box::pin(heartbeat_interval);

                        let heartbeat = proto::grpc::ViewerUpdate {
                            update: Some(proto::grpc::viewer_update::Update::Heartbeat(proto::grpc::Heartbeat { seq })),
                        };
                        match stream_tx.read().await.send(Ok(heartbeat)).await {
                            Err(_) => {
                                alive = false;
                                if last_heartbeat.elapsed().as_secs() > config.run.max_keepalive_sec {
                                    // remove the instance
                                    eprintln!("can no longer send to {} {}", session_token, instance_id);
                                    if viewers.write().await.remove(&instance_id).is_none() {
                                        eprintln!("failed to remove {} {}", session_token, instance_id);
                                    }
                                    eprintln!("removed {} {}", session_token, instance_id);
                                    return;
                                }
                            }
                            Ok(_) => {
                                alive = true;
                                last_heartbeat =  tokio::time::Instant::now();
                            }
                        };
                        seq = seq + 1;
                    }
                }
                if was_alive != alive {
                    let mut new_liveness = alive_state.write().await;
                    *new_liveness = alive;
                }
            }
        });

        println!("exiting");
        Ok(response)
    }
}
