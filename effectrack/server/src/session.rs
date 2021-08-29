use crate::analyzer::{AudioAnalyzerNode, AudioInputNode, AudioOutputNode};
use crate::cli::Config;
use crate::controller::Controller;
use crate::viewer::Viewer;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock};

#[derive(Clone)]
pub struct Session<VU, CU> {
    /// session configuration
    pub config: Config,

    /// connected controllers
    pub controllers: Arc<RwLock<HashMap<String, RwLock<Controller<CU>>>>>,

    /// connected viewers
    pub viewers: Arc<RwLock<HashMap<proto::grpc::InstanceId, RwLock<Viewer<VU>>>>>,

    /// all running analyzers of this session
    pub analyzers: Arc<
        RwLock<
            HashMap<proto::grpc::AudioAnalyzerDescriptor, RwLock<AudioAnalyzerNode<crate::Sample>>>,
        >,
    >,

    /// all running input streams of this session
    pub input_streams: Arc<
        RwLock<HashMap<proto::grpc::AudioInputDescriptor, RwLock<AudioInputNode<crate::Sample>>>>,
    >,

    /// all running output streams of this session
    pub output_streams: Arc<
        RwLock<HashMap<proto::grpc::AudioOutputDescriptor, RwLock<AudioOutputNode<crate::Sample>>>>,
    >,
}

impl<VU, CU> Session<VU, CU> {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            viewers: Arc::new(RwLock::new(HashMap::new())),
            controllers: Arc::new(RwLock::new(HashMap::new())),
            analyzers: Arc::new(RwLock::new(HashMap::new())),
            input_streams: Arc::new(RwLock::new(HashMap::new())),
            output_streams: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
