fn main() {
    // trigger rebuilds when these files change
    println!("cargo:rerun-if-changed=../../proto/grpc/remote.proto");
    println!("cargo:rerun-if-changed=../../proto/audio/analysis/analysis.proto");
    println!("cargo:rerun-if-changed=../../proto/audio/analysis/spectral.proto");

    tonic_build::configure()
        .type_attribute("proto.grpc.InstanceId", "#[derive(Hash, Eq)]")
        .type_attribute("proto.grpc.SessionToken", "#[derive(Hash, Eq)]")
        .type_attribute("proto.grpc.AudioInputDescriptor", "#[derive(Hash, Eq)]")
        .type_attribute("proto.grpc.AudioOutputDescriptor", "#[derive(Hash, Eq)]")
        .type_attribute("proto.grpc.AudioAnalyzerDescriptor", "#[derive(Hash, Eq)]")
        .build_server(true)
        .build_client(false)
        .compile(&["../../proto/grpc/remote.proto"], &["../../"])
        .unwrap();
}
