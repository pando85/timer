use crate::Result;

use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::convert::TryFrom;
use std::io::Cursor;

pub struct Sound {
    _stream: OutputStream,
    cursor: Cursor<&'static [u8; 5943]>,
}

impl Sound {
    pub fn new() -> Result<Self> {
        let stream = OutputStreamBuilder::open_default_stream()?;
        let contents = include_bytes!("beep.ogg");
        Ok(Sound {
            _stream: stream,
            cursor: Cursor::new(contents),
        })
    }

    /// Play sound (it has a delay of ~100ms)
    pub fn play(&self) -> Result<()> {
        let stream = OutputStreamBuilder::open_default_stream()?;
        let mixer = stream.mixer();
        let cursor = self.cursor.clone();
        let decoder = Decoder::try_from(cursor)?;
        let sink = Sink::connect_new(mixer);
        sink.append(decoder);
        sink.sleep_until_end();
        Ok(())
    }
}
