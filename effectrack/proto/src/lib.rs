pub mod audio {
    pub mod analysis {
        tonic::include_proto!("proto.audio.analysis");
    }
}

pub mod grpc {
    tonic::include_proto!("proto.grpc");
}
