use std::fs::File;
use std::io::{self, Result, Write, IoSlice};

struct Write {
    file: File,
}

impl Trait {
    fn new(file: File) -> Trait {
        Trait { file }
    }
}

impl Write for Trait {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut duplicated_buf = Vec::with_capacity(buf.len() * 2);

        for &byte in buf {
            duplicated_buf.push(byte);
            duplicated_buf.push(byte);
        }

        self.file.write_all(&duplicated_buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.file.flush()
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize> {
        let mut total_bytes_written = 0;
        for buf in bufs {
            total_bytes_written += self.write(buf)?;
        }
        Ok(total_bytes_written)
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.write(buf)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut writer = Trait::new(File::create("a.txt")?);
    writer.write_all(b"abc")?;

    Ok(())
}