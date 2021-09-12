use crate::analyzer::{AudioAnalyzerNode, AudioInputNode, AudioOutputNode};
use crate::cli::Config;
use crate::controller::Controller;
use crate::viewer::Viewer;
use crate::Sample;
use futures::stream::{self, StreamExt};
use prost_types::Timestamp;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Session<VU, CU> {
    /// session token
    pub token: proto::grpc::SessionToken,

    /// session configuration
    pub config: Config,

    /// session start time
    pub started: Timestamp,

    /// connected controllers
    pub controllers: Arc<RwLock<HashMap<proto::grpc::InstanceId, RwLock<Controller<CU>>>>>,

    /// connected viewers
    pub viewers: Arc<RwLock<HashMap<proto::grpc::InstanceId, RwLock<Viewer<VU>>>>>,

    /// all running analyzers of this session
    pub analyzers: Arc<
        RwLock<HashMap<proto::grpc::AudioAnalyzerDescriptor, RwLock<AudioAnalyzerNode<Sample>>>>,
    >,

    /// all running input streams of this session
    pub input_streams:
        Arc<RwLock<HashMap<proto::grpc::AudioInputDescriptor, RwLock<AudioInputNode<Sample>>>>>,

    /// all running output streams of this session
    pub output_streams:
        Arc<RwLock<HashMap<proto::grpc::AudioOutputDescriptor, RwLock<AudioOutputNode<Sample>>>>>,
}

impl<VU, CU> Session<VU, CU> {
    pub fn new(token: proto::grpc::SessionToken, config: Config) -> Self {
        Self {
            config,
            token,
            started: Timestamp::from(SystemTime::now()),
            viewers: Arc::new(RwLock::new(HashMap::new())),
            controllers: Arc::new(RwLock::new(HashMap::new())),
            analyzers: Arc::new(RwLock::new(HashMap::new())),
            input_streams: Arc::new(RwLock::new(HashMap::new())),
            output_streams: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn info(&self) -> proto::grpc::SessionInfo {
        let controllers = stream::iter(self.controllers.read().await.deref())
            .then(|(_, controller)| async move { controller.read().await.deref().info().await })
            .collect()
            .await;
        let viewers = stream::iter(self.viewers.read().await.deref())
            .then(|(_, viewer)| async move { viewer.read().await.deref().info().await })
            .collect()
            .await;
        proto::grpc::SessionInfo {
            token: Some(self.token.clone()),
            started: Some(self.started.clone()),
            controllers,
            viewers,
        }
    }
}
