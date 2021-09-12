#[cfg(all(not(feature = "record"), any(feature = "portaudio", feature = "jack")))]
compile_error!("feature \"jack\" and feature \"portaudio\" cannot be enabled when feature \"record\" is disabled");

pub mod analyzer;
pub mod cli;
pub mod controller;
pub mod session;
pub mod viewer;
pub extern crate common;
pub extern crate hardware;
pub extern crate proto;

#[cfg(feature = "p2p")]
mod p2p;
#[cfg(feature = "analyze")]
pub extern crate analysis;
#[cfg(feature = "record")]
pub extern crate recorder;

use anyhow::Result;
use cli::Config;
use nanoid::nanoid;
use proto::grpc::remote_controller_server::RemoteControllerServer;
use proto::grpc::remote_viewer_server::RemoteViewerServer;
#[cfg(feature = "portaudio")]
use recorder::portaudio::PortaudioAudioInput;

#[cfg(feature = "record")]
use session::Session;
use std::collections::HashMap;
use std::marker::Send;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::{watch, RwLock};
use tonic::{transport::Server as TonicServer, Code, Request, Status};
use warp::Filter;

const INSTANCE_ID_KEY: &str = "instance-id";
const SESSION_TOKEN_KEY: &str = "session-token";

pub type Sample = f32;
pub type ViewerUpdateMsg = proto::grpc::ViewerUpdate;
pub type ControllerUpdateMsg = proto::grpc::ControllerUpdate;

#[derive(Clone)]
pub struct DiscoServer<VU, CU>
where
    VU: Clone,
    CU: Clone,
{
    pub config: Config,
    pub sessions: Arc<RwLock<HashMap<proto::grpc::SessionToken, Session<VU, CU>>>>,
    pub shutdown_rx: watch::Receiver<bool>,
}

impl<VU, CU> DiscoServer<VU, CU>
where
    VU: Clone,
    CU: Clone,
{
    async fn new_session(&self) -> Result<proto::grpc::SessionToken, Status> {
        let alphabet: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let mut sessions = self.sessions.write().await;
        let session_count = sessions.len();
        println!("there are {} sessions", session_count);
        if let Some(max_sessions) = self.config.run.max_sessions {
            if session_count >= max_sessions {
                return Err(Status::unavailable(format!(
                    "maximum number of sessions ({}) exceeded",
                    max_sessions
                )));
            }
        }
        let session_token = (|| {
            for _ in 0..100 {
                let token = proto::grpc::SessionToken {
                    token: nanoid!(4, &alphabet),
                };
                if sessions.get(&token).is_none() {
                    // insert
                    sessions.insert(
                        token.clone(),
                        // RwLock::new(Session::new(self.config.clone())),
                        Session::new(token.clone(), self.config.clone()),
                    );
                    return Ok(token);
                }
            }
            Err(Status::internal("failed to generate a new session token"))
        })();
        session_token
    }

    async fn extract_metadata<T>(request: &Request<T>, key: &str) -> Result<String, Status> {
        request
            .metadata()
            .get(key)
            .ok_or(Status::new(
                Code::InvalidArgument,
                format!("missing \"{}\" metadata", key),
            ))
            .and_then(|token| {
                token.to_str().map_err(|_| {
                    Status::new(
                        Code::InvalidArgument,
                        format!("failed to decode \"{}\" metadata", key),
                    )
                })
            })
            .map(|token| token.to_string())
    }

    async fn extract_session_token<T>(
        request: &Request<T>,
    ) -> Result<proto::grpc::SessionToken, Status> {
        Self::extract_metadata(request, SESSION_TOKEN_KEY)
            .await
            .map(|token| proto::grpc::SessionToken { token })
    }

    async fn extract_instance_id<T>(
        request: &Request<T>,
    ) -> Result<proto::grpc::InstanceId, Status> {
        Self::extract_metadata(request, INSTANCE_ID_KEY)
            .await
            .map(|id| proto::grpc::InstanceId { id })
    }
    async fn extract_session_instance<T>(
        request: &Request<T>,
    ) -> Result<(proto::grpc::SessionToken, proto::grpc::InstanceId), Status> {
        let session_token = Self::extract_session_token(request).await?;
        let instance_id = Self::extract_instance_id(request).await?;
        Ok((session_token, instance_id))
    }
}

impl<VU, CU> DiscoServer<VU, CU>
where
    VU: Clone + Send,
    CU: Clone + Send,
{
    pub fn new_with_shutdown(config: Config, shutdown_rx: watch::Receiver<bool>) -> Self {
        Self {
            config,
            shutdown_rx,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl DiscoServer<ViewerUpdateMsg, ControllerUpdateMsg> {
    pub async fn serve(&self) -> Result<()> {
        let host = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let grpc_addr = SocketAddr::new(host, self.config.run.port);
        let static_addr = SocketAddr::new(host, 8888);

        println!("grpc listening at {}", grpc_addr);
        println!("frontend served at {}", static_addr);

        let remote_controller_grpc_server = RemoteControllerServer::new(self.clone());
        let remote_controller_grpc_server = tonic_web::config()
            // .allow_origins(vec!["localhost", "127.0.0.1"])
            .enable(remote_controller_grpc_server);

        let remote_viewer_grpc_server = RemoteViewerServer::new(self.clone());
        let remote_viewer_grpc_server = tonic_web::config()
            // .allow_origins(vec!["localhost", "127.0.0.1"])
            .enable(remote_viewer_grpc_server);

        let grpc_server = TonicServer::builder()
            .accept_http1(true)
            .max_concurrent_streams(128)
            .add_service(remote_controller_grpc_server)
            .add_service(remote_viewer_grpc_server);

        tokio::task::spawn(async move {
            let web = warp::get().and(warp::fs::dir("../../www/build"));
            warp::serve(web).run(static_addr).await;
        });

        grpc_server
            .serve_with_shutdown(grpc_addr, async {
                self.shutdown_rx
                    .clone()
                    .changed()
                    .await
                    .expect("failed to shutdown");
            })
            .await?;
        Ok(())
    }
}
