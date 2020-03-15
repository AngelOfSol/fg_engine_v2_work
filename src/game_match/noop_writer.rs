use std::io::Write;

pub struct NoopWriter;

impl From<()> for NoopWriter {
    fn from(_: ()) -> Self {
        NoopWriter
    }
}

impl Write for NoopWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
