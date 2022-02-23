use disco::cli::{Config, Opts, StartOpts};
use disco::DiscoServer;
use pyo3::exceptions;
use pyo3::prelude::*;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::watch;

// TODO: query audio backend
// TODO: all the other possible GRPC methods

#[pyclass(subclass)]
struct Parameterizer {}

#[pymethods]
impl Parameterizer {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {})
    }
}

#[pyclass(subclass)]
struct Analyzer {}

#[pymethods]
impl Analyzer {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {})
    }
}

#[pyclass]
struct Server {
    server: Arc<DiscoServer<disco::ViewerUpdateMsg, disco::ControllerUpdateMsg>>,
    runtime: Runtime,
    shutdown_tx: watch::Sender<bool>,
}

#[pymethods]
impl Server {
    #[new]
    fn new() -> PyResult<Self> {
        // Err(exceptions::PyTypeError::new_err("Error message"))
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let config = Config {
            run: StartOpts {
                port: 9000,
                play_file: None,
                max_sessions: None,
                max_viewers: None,
                max_controllers: None,
                max_keepalive_sec: 30,
                no_sound: false,
            },
            default: Opts {
                input_device: None,
                output_device: None,
                latency: 5,
                #[cfg(use_jack)]
                use_jack: false,
                #[cfg(feature = "portaudio")]
                use_portaudio: false,
                commands: None,
            },
        };

        let runtime = Runtime::new().unwrap();
        let server = Arc::new(DiscoServer::new_with_shutdown(config, shutdown_rx));
        Ok(Server {
            server,
            runtime,
            shutdown_tx,
        })
    }

    fn start(self_: PyRef<Self>) -> PyResult<()> {
        let server = self_.server.clone();
        // create a new runtime for the server
        println!("starting server");
        runtime.spawn(async move {
            println!("starting server");
            server.serve().await.expect("failed to run disco");
        });
        // tokio::task::spawn(async move {
        // });
        Ok(())
    }

    fn stop(self_: PyRef<Self>) -> PyResult<()> {
        self_.shutdown_tx.send(true).expect("send shutdown signal");
        // todo: await for the tokio task here to make sure it really quits
        Ok(())
    }
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn disco(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    // TODO: expose a class for the server here
    // TODO: expose a class for visualization, parameterizer etc.
    m.add_class::<Server>()?;
    m.add_class::<Parameterizer>()?;
    m.add_class::<Analyzer>()?;
    Ok(())
}
