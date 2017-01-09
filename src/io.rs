//! I/O

use core::fmt;

use syscall;

/// Byte-oriented sink
pub trait Write {
    /// Error type
    type Error;

    /// Writes `bytes` into the sink, returning how bytes were written
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error>;

    /// Writes the entire `buffer` into the sink
    fn write_all(&mut self, mut buffer: &[u8]) -> Result<(), Self::Error> {
        loop {
            let n = self.write(buffer)?;

            if n < buffer.len() {
                buffer = &buffer[n..];
            } else {
                return Ok(())
            }
        }
    }
}

/// Standard error
pub struct Stderr {
    _0: (),
}

impl Write for Stderr {
    type Error = ();

    fn write(&mut self, bytes: &[u8]) -> Result<usize, ()> {
        match unsafe { syscall::write(2, bytes.as_ptr(), bytes.len()) } {
            -1 => Err(()),
            i => Ok(i as usize),
        }
    }
}

impl fmt::Write for Stderr {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

/// Standard output
pub struct Stdout {
    _0: (),
}

impl Write for Stdout {
    type Error = ();

    fn write(&mut self, bytes: &[u8]) -> Result<usize, ()> {
        match unsafe { syscall::write(1, bytes.as_ptr(), bytes.len()) } {
            -1 => Err(()),
            i => Ok(i as usize),
        }
    }
}

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

/// Returns a handle to the standard error
pub fn stderr() -> Stderr {
    Stderr { _0: () }
}

/// Returns a handle to the standard output
pub fn stdout() -> Stdout {
    Stdout { _0: () }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    if stdout().write_fmt(args).is_err() {
        panic!("failed printing to stdout")
    }
}
