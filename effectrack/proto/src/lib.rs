pub mod audio {
    use std::fmt;

    pub mod analysis {
        tonic::include_proto!("proto.audio.analysis");
    }
}

pub mod grpc {
    use std::fmt;

    tonic::include_proto!("proto.grpc");

    impl fmt::Display for SessionToken {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "\"{}\"", self.token)
        }
    }

    impl fmt::Display for InstanceId {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "\"{}\"", self.id)
        }
    }

    impl fmt::Display for AudioInputDescriptor {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[INPUT,{},{},{}]", self.backend, self.host, self.device)
        }
    }

    impl fmt::Display for AudioOutputDescriptor {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "({:?} -> [OUTPUT,{},{},{}])",
                self.input, self.backend, self.host, self.device
            )
        }
    }

    impl fmt::Display for AudioAnalyzerDescriptor {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({:?} -> [ANALYZER,{}])", self.input, self.name)
        }
    }
}
