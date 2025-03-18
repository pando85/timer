use crate::Result;

use std::io::{BufReader, Cursor};

use rodio::{Decoder, OutputStream, OutputStreamHandle, source::Source};

pub struct Sound {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    cursor: Cursor<&'static [u8; 5943]>,
}

impl Sound {
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let contents = include_bytes!("beep.ogg");
        Ok(Sound {
            _stream: stream,
            stream_handle,
            cursor: Cursor::new(contents),
        })
    }

    /// Play sound (it has a delay of ~100ms)
    pub fn play(&self) -> Result<()> {
        let buffer = BufReader::new(self.cursor.clone());
        let decoded = Decoder::new_vorbis(buffer).unwrap();
        let source = decoded.convert_samples();
        self.stream_handle.play_raw(source)?;
        Ok(())
    }
}
