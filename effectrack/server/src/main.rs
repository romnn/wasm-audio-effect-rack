#[cfg(all(not(feature = "record"), any(feature = "portaudio", feature = "jack")))]
compile_error!("feature \"jack\" and feature \"portaudio\" cannot be enabled when feature \"record\" is disabled");

mod analyzer;
mod cli;
mod controller;
mod session;
mod viewer;
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
use clap::Clap;
use cli::{Commands, Config, Opts};
use nanoid::nanoid;
use proto::grpc::remote_controller_server::RemoteControllerServer;
use proto::grpc::remote_viewer_server::RemoteViewerServer;
#[cfg(feature = "portaudio")]
use recorder::portaudio::PortaudioAudioInput;

use futures::future;
use futures::future::{Either, TryFutureExt};
use http::{version::Version, Response};
use hyper::{service::make_service_fn, Body, Request as HyperRequest, Server};
#[cfg(feature = "record")]
use recorder::{cpal::CpalAudioBackend, cpal::CpalAudioInput, AudioInput, AudioInputConfig};
use session::Session;
use std::collections::HashMap;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::marker::Send;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::signal;
use tokio::sync::{watch, RwLock};
use tonic::{transport::Server as TonicServer, Code, Request, Status};
use tower::{make::Shared, service_fn, Service, ServiceBuilder};
// use tower_http::add_extension::AddExtensionLayer;
use warp::Filter;

const INSTANCE_ID_KEY: &str = "instance-id";
const SESSION_TOKEN_KEY: &str = "session-token";

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
                        Session::new(self.config.clone()),
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
    fn new_with_shutdown(config: Config, shutdown_rx: watch::Receiver<bool>) -> Self {
        Self {
            config,
            shutdown_rx,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

// async fn handler(request: Request<Body>) -> Result<Response<Body>, Error> {
//     // web.call(req)
//     //     .map_ok(|res| res.map(EitherBody::Left))
//     //     .map_err(Error::from)
// }

impl DiscoServer<ViewerUpdateMsg, ControllerUpdateMsg> {
    async fn serve(&self) -> Result<()> {
        let addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let addr = SocketAddr::new(addr, self.config.run.port);

        println!("listening on {}", addr);

        // warp::serve(route).run(([127, 0, 0, 1], 3030)).await;

        Server::bind(&addr)
            .serve(make_service_fn(move |_| {
                let remote_controller_grpc_server = RemoteControllerServer::new(self.clone());
                let remote_controller_grpc_server = tonic_web::config()
                    // .allow_origins(vec!["localhost", "127.0.0.1"])
                    .enable(remote_controller_grpc_server);

                let remote_viewer_grpc_server = RemoteViewerServer::new(self.clone());
                let remote_viewer_grpc_server = tonic_web::config()
                    // .allow_origins(vec!["localhost", "127.0.0.1"])
                    .enable(remote_viewer_grpc_server);

                let mut grpc_server = TonicServer::builder()
                    .accept_http1(true)
                    .max_concurrent_streams(128)
                    .add_service(remote_controller_grpc_server)
                    .add_service(remote_viewer_grpc_server)
                    .into_service();

                // let mut grpc_web_server = TonicServer::builder()
                //     .accept_http1(true)
                //     .max_concurrent_streams(128)
                //     .add_service(remote_controller_grpc_web_server)
                //     .add_service(remote_viewer_grpc_web_server)
                //     .into_service();

                let mut web = warp::service(warp::path("/").and(warp::fs::dir("www/build")));
                // let mut web = warp::path("").and(warp::fs::dir("www/build"));
                // let routes = warp::service(warp::get().and(web.or(warp::service(grpc_web_server))));
                // let mut web = warp::service(warp::path("hello").map(|| {
                //     "Test"
                //     // grpc_web_server
                //     //     .call(req)
                //     //     .map_ok(|res| res.map(EitherBody::Left))
                //     //     .map_err(Error::from);
                // }));

                // let service = ServiceBuilder::new()
                //     .layer(AddExtensionLayer::new(Arc::new(web)))
                //     .layer(AddExtensionLayer::new(Arc::new(grpc_web_server)))
                //     .service_fn(handler);
                // let mut grpc_web_server = grpc_server.accept_http1(true).into_service();
                // let mut grpc_server = grpc_server.into_service();

                // Ok::<_, Infallible>(service_fn(move |req: hyper::Request<hyper::Body>| {
                future::ok::<_, Infallible>(service_fn(move |req: hyper::Request<hyper::Body>| {
                    // future::ok::<_, Infallible>(service_fn(move |req| {
                    match req.version() {
                        // Version::HTTP_11 | Version::HTTP_10 => Either::Left((|| {
                        Version::HTTP_11 | Version::HTTP_10 => {
                            // if req.uri().path().starts_with("grpc") {
                            //     return MyEither::Left(
                            //         web.call(req)
                            //             .map_ok(|res| res.map(EitherBody::Left))
                            //             .map_err(Error::from),
                            //     );
                            // }
                            return Either::Left(
                                web.call(req)
                                    .map_ok(|res| res.map(EitherBody::Left))
                                    .map_err(Error::from),
                            );
                        }
                        Version::HTTP_2 => Either::Right(
                            grpc_server
                                .call(req)
                                .map_ok(|res| res.map(EitherBody::Right))
                                .map_err(Error::from),
                        ),
                        _ => unimplemented!(),
                    }

                    // return Either::Left(
                    //     web.call(req)
                    //         .map_ok(|res| res.map(EitherBody::Left))
                    //         .map_err(Error::from),
                    // );
                    //     .map_ok(|res| {
                    //         let s: String = res;
                    //     })
                    //     .map_err(Error::from);
                    // Pin<Box<(dyn futures::Future<Output = std::result::Result<http::Response<BoxBody<hyper::body::Bytes, Status>>
                    // , Never>> + std::marker::Send + 'static)>>, fn(Never) -> Box<(dyn std::error::Error + Sync + std::marker::Send + 'static)>>
                    //     // .map_ok(|res| res.map(EitherBody::Right))
                    //     .map_err(Error::from).into();
                    // return Either::Right(
                    //     grpc_web_server
                    //         .call(req)
                    //         // .map_ok(|res| res.map(|res| Test::new(res)))
                    //         .map_ok(|res| res.map(EitherBody::Right))
                    //         .map_err(Error::from),
                    // );
                    // Either::Right(
                    //     grpc_server
                    //         .call(req)
                    //         .map_ok(|res| res.map(EitherBody::Right))
                    //         .map_err(Error::from),
                    // );
                    // .map_ok(|res| res.map(|res| Test::new(res)))
                    // .map_err(Error::from)
                }))
                // async {
                //     Ok::<_, Error>(service_fn(move |req| {
                //         // todo:
                //     }))
                // }
                // future::ok::<_, Infallible>(tower::service_fn(
                //     move |req: hyper::Request<hyper::Body>| {
                //         // web
                //         //     .call(req)
                //         //     // .map_ok(|res| res.map(EitherBody::Left))
                //         //     // .map_ok(|res| res.map(EitherBody::Left))
                //         //     .map_err(Error::from)
                //         match req.version() {
                //             // Version::HTTP_11 | Version::HTTP_10 => Either::Left((|| {
                //             Version::HTTP_11 | Version::HTTP_10 => {
                //                 if req.uri().path().starts_with("grpc") {
                //                     return Either::Left(
                //                         web.call(req)
                //                             .map_ok(|res| res.map(EitherBody::Third))
                //                             .map_err(Error::from),
                //                     );
                //                 }
                //                 return Either::Left(
                //                     grpc_web_server
                //                         .call(req)
                //                         .map_ok(|res| res.map(EitherBody::Left))
                //                         .map_err(Error::from),
                //                 );
                //             }
                //             Version::HTTP_2 => Either::Right({
                //                 grpc_server
                //                     .call(req)
                //                     .map_ok(|res| res.map(EitherBody::Right))
                //                     .map_err(Error::from)
                //             }),
                //             _ => unimplemented!(),
                //         }
                //     },
                // ))
            }))
            .await?;

        // .serve_with_shutdown(addr, async {
        // self.shutdown_rx
        // .clone()
        // .changed()
        // .await
        // .expect("failed to shutdown");
        // })
        // .await?;
        Ok(())
    }
}

pub enum MyEither<A, B, C> {
    AA(A),
    BB(B),
    CC(C),
}

impl<A, B, C> MyEither<A, B, C> {
    fn project(self: Pin<&mut Self>) -> MyEither<Pin<&mut A>, Pin<&mut B>, Pin<&mut C>> {
        unsafe {
            match self.get_unchecked_mut() {
                Self::AA(a) => MyEither::AA(Pin::new_unchecked(a)),
                Self::BB(b) => MyEither::BB(Pin::new_unchecked(b)),
                Self::CC(c) => MyEither::CC(Pin::new_unchecked(c)),
            }
        }
    }
}

impl<A, B, C> futures::Future for MyEither<A, B, C>
where
    A: futures::Future,
    B: futures::Future<Output = A::Output>,
    C: futures::Future<Output = A::Output>,
{
    type Output = A::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            MyEither::AA(x) => x.poll(cx),
            MyEither::BB(x) => x.poll(cx),
            MyEither::CC(x) => x.poll(cx),
        }
    }
}

// impl http_body::Body for EitherBody<A, B, C>
struct Test<R> {
    // struct Test {
    // lol: R,
    phantom: PhantomData<R>,
}

impl<R> Test<R> {
    fn new(r: R) -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

// impl<R> http_body::Body for Test<R>
impl<R> http_body::Body for Test<R>
// impl<R> http_body::Body for warp::filter::service::FilteredFuture
where
    R: http_body::Body + Send + Unpin,
{
    type Data = R::Data;
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    fn is_end_stream(&self) -> bool {
        // self.lol.is_end_stream()
        self.is_end_stream()
    }

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        Pin::new(self.get_mut())
            .poll_data(cx)
            .map(|e| e.map(|e| e.map_err(Into::into)))
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        Pin::new(self.get_mut())
            .poll_trailers(cx)
            .map_err(Into::into)
    }
}

enum EitherBody<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> http_body::Body for EitherBody<A, B>
where
    A: http_body::Body + Send + Unpin,
    B: http_body::Body<Data = A::Data> + Send + Unpin,
    A::Error: Into<Error>,
    B::Error: Into<Error>,
{
    type Data = A::Data;
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    fn is_end_stream(&self) -> bool {
        match self {
            EitherBody::Left(b) => b.is_end_stream(),
            EitherBody::Right(b) => b.is_end_stream(),
        }
    }

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        match self.get_mut() {
            EitherBody::Left(b) => Pin::new(b)
                .poll_data(cx)
                .map(|e| e.map(|e| e.map_err(Into::into))),
            EitherBody::Right(b) => Pin::new(b)
                .poll_data(cx)
                .map(|e| e.map(|e| e.map_err(Into::into))),
        }
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        match self.get_mut() {
            EitherBody::Left(b) => Pin::new(b).poll_trailers(cx).map_err(Into::into),
            EitherBody::Right(b) => Pin::new(b).poll_trailers(cx).map_err(Into::into),
        }
    }
}

// enum EitherBody<A, B, C> {
//     Left(A),
//     Right(B),
//     Third(C),
// }

// impl<A, B, C> http_body::Body for EitherBody<A, B, C>
// where
//     A: http_body::Body + Send + Unpin,
//     B: http_body::Body<Data = A::Data> + Send + Unpin,
//     C: http_body::Body<Data = A::Data> + Send + Unpin,
//     A::Error: Into<Error>,
//     B::Error: Into<Error>,
//     C::Error: Into<Error>,
// {
//     type Data = A::Data;
//     type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

//     fn is_end_stream(&self) -> bool {
//         match self {
//             EitherBody::Left(b) => b.is_end_stream(),
//             EitherBody::Right(b) => b.is_end_stream(),
//             EitherBody::Third(b) => b.is_end_stream(),
//         }
//     }

//     fn poll_data(
//         self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//     ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
//         match self.get_mut() {
//             EitherBody::Left(b) => Pin::new(b)
//                 .poll_data(cx)
//                 .map(|e| e.map(|e| e.map_err(Into::into))),
//             EitherBody::Right(b) => Pin::new(b)
//                 .poll_data(cx)
//                 .map(|e| e.map(|e| e.map_err(Into::into))),
//             EitherBody::Third(b) => Pin::new(b)
//                 .poll_data(cx)
//                 .map(|e| e.map(|e| e.map_err(Into::into))),
//         }
//     }

//     fn poll_trailers(
//         self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//     ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
//         match self.get_mut() {
//             EitherBody::Left(b) => Pin::new(b).poll_trailers(cx).map_err(Into::into),
//             EitherBody::Right(b) => Pin::new(b).poll_trailers(cx).map_err(Into::into),
//             EitherBody::Third(b) => Pin::new(b).poll_trailers(cx).map_err(Into::into),
//         }
//     }
// }

pub type Sample = f32;
const SPLASH_LOGO: &str = "
   __  ___  __   _   _  
   ) )  )  (_ ` / ` / ) 
  /_/ _(_ .__) (_. (_/  
";

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    println!("{}", SPLASH_LOGO);

    if let Some(ref subcommand) = opts.commands {
        match subcommand {
            #[cfg(feature = "record")]
            Commands::Query(cfg) => {
                let audio_backend =
                    CpalAudioInput::<Sample>::new(AudioInputConfig::from(opts.clone()))?;
                cfg_if::cfg_if! {
                    if #[cfg(feature = "portaudio")] {
                        if opts.use_portaudio{
                            let audio_backend  = PortaudioAudioInput::new(opts.into())?;
                        };
                    }
                };
                match &cfg.device {
                    Some(_device) => {
                        // audio_backend.query_device(device)?;
                    }
                    None => {
                        audio_backend.query()?;
                    }
                }
            }
            Commands::Start(cfg) => {
                let config = Config {
                    run: cfg.clone(),
                    default: opts.clone(),
                };

                let disco = Arc::new(DiscoServer::new_with_shutdown(config, shutdown_rx));

                let _running = tokio::task::spawn(async move {
                    disco.serve().await.expect("failed to run disco");
                });

                signal::ctrl_c().await.ok().map(|_| {
                    println!("received shutdown");
                    shutdown_tx.send(true).expect("send shutdown signal");
                });

                // todo: also wait for other threads and all
                // _running.await.ok();
                println!("exiting");
            }
        }
    }
    Ok(())
}
