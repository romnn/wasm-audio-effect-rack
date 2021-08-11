// use std::env;
// use std::path::PathBuf;

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
        // .type_attribute("routeguide.Point", "#[derive(Hash)]")
        .build_server(true)
        .build_client(false)
        .compile(
            &[
                // "../../proto/audio/analysis.proto",
                // "../../proto/audio/analysis/analysis.proto",
                "../../proto/grpc/remote.proto",
            ],
            &["../../"],
        )
        .unwrap();

    // tonic_build::compile_protos("proto/route_guide.proto")
    //     .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // tonic_build::configure()
    //     .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
    //     .compile(&["proto/helloworld/helloworld.proto"], &["proto"])
    //     .unwrap();

    // tonic_build::compile_protos("proto/echo/echo.proto").unwrap();

    // tonic_build::configure()
    //     .server_mod_attribute("attrs", "#[cfg(feature = \"server\")]")
    //     .server_attribute("Echo", "#[derive(PartialEq)]")
    //     .client_mod_attribute("attrs", "#[cfg(feature = \"client\")]")
    //     .client_attribute("Echo", "#[derive(PartialEq)]")
    //     .compile(&["proto/attrs/attrs.proto"], &["proto"])
    //     .unwrap();

    // tonic_build::configure()
    //     .build_server(false)
    //     .compile(
    //         &["proto/googleapis/google/pubsub/v1/pubsub.proto"],
    //         &["proto/googleapis"],
    //     )
    //     .unwrap();
}
