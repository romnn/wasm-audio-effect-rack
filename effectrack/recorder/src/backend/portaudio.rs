use anyhow::Result;

#[cfg(feature = "portaudio")]
use crate::portaudio::PortAudioRecorder;
use crate::{AudioBackend, AudioBackendConfig};

#[cfg(feature = "portaudio")]
#[derive()]
pub struct PortaudioAudioBackend {}

#[cfg(feature = "portaudio")]
impl AudioBackend for PortaudioBackend {
    type Rec = PortaudioRecorder;

    fn new(config: AudioBackendConfig) -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn new_recorder(&self) -> Result<Self::Rec> {
        Ok(Self::Rec {})
    }

    fn query(&self) -> Result<()> {
        Ok(())
    }
}
