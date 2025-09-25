#[cfg(not(feature = "test-beep"))]
use crate::Result;
#[cfg(not(feature = "test-beep"))]
use rodio::mixer::Mixer;
#[cfg(not(feature = "test-beep"))]
use rodio::stream::{OutputStream, OutputStreamBuilder};
#[cfg(not(feature = "test-beep"))]
use std::io::Cursor;

#[cfg(not(feature = "test-beep"))]
pub struct Sound {
    _stream: OutputStream,
    mixer: Mixer,
    cursor: Cursor<&'static [u8; 5943]>,
}

#[cfg(not(feature = "test-beep"))]
impl Sound {
    pub fn new() -> Result<Self> {
        let stream = OutputStreamBuilder::open_default_stream()?;
        let mixer = stream.mixer().clone();
        let contents = include_bytes!("beep.ogg");
        Ok(Sound {
            _stream: stream,
            mixer,
            cursor: Cursor::new(contents),
        })
    }

    /// Play sound (it has a delay of ~100ms)
    pub fn play(&self) -> Result<()> {
        let cursor = self.cursor.clone();
        let decoder = rodio::Decoder::new(cursor)?;
        let sink = rodio::Sink::connect_new(&self.mixer);
        sink.append(decoder);
        sink.sleep_until_end();
        Ok(())
    }
}
