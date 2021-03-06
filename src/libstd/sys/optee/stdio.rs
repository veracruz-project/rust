use crate::io;
use crate::sys::unsupported;

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub fn new() -> io::Result<Stdin> {
        Ok(Stdin)
    }
}

impl io::Read for Stdin {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }
}


impl Stdout {
    pub fn new() -> io::Result<Stdout> {
        Ok(Stdout)
    }
}

impl io::Write for Stdout {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        unsupported()
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Stderr {
    pub fn new() -> io::Result<Stderr> {
        Ok(Stderr)
    }
}

impl io::Write for Stderr {
    fn write(&mut self, _data: &[u8]) -> io::Result<usize> {
        unsupported()
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}

pub fn panic_output() -> Option<impl io::Write> {
    Stderr::new().ok()
}
