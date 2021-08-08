#[derive(Debug)]
pub struct ConnectedUserState<U, R> {
    connection: Arc<RwLock<mpsc::Sender<U>>>,
    recorder: Option<Arc<R>>,
    config: StartOpts,
    is_analyzing: bool,
}


