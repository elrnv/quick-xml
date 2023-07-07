//! Custom write struct that extends io::Write with formatting capabilities.

use std::{io, fmt};

/// Custom write trait.
pub trait Write: fmt::Write {
    /// Write function akin to `std::io::Write::write`.
    fn write(&mut self, b: &[u8]) -> fmt::Result;
}

impl<W: fmt::Write> Write for W {
    fn write(&mut self, b: &[u8]) -> fmt::Result {
        self.write_str(&String::from_utf8_lossy(b))
    }
}

/// Wrapper for IO writers.
pub struct IoWriter<W>(pub W);
 
impl<W: io::Write> fmt::Write for IoWriter<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write(s.as_bytes()).map_err(|_| fmt::Error)?;
        self.0.flush().map_err(|_| fmt::Error)?;
        Ok(())
    }
}