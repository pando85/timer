use crate::Result;

use std::io::Cursor;

use rodio::mixer::Mixer;
use rodio::stream::{OutputStream, OutputStreamBuilder};

pub struct Sound {
    _stream: OutputStream,
    mixer: Mixer,
    cursor: Cursor<&'static [u8; 5943]>,
}

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
