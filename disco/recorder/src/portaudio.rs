#[cfg(feature = "portaudio")]
#[derive(Clone)]
pub struct PortaudioRecorder {}

#[cfg(feature = "portaudio")]
impl Recorder for PortaudioRecorder {
    fn stream_file(&self, path: PathBuf) -> Result<()> {
        Ok(())
    }

    fn stream_input(&self) -> Result<()> {
        Ok(())
    }
}
