use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct LockPipe {
  path: PathBuf,
}

impl LockPipe {
  pub fn new<P: Into<PathBuf>>(path: P) -> Self {
    Self { path: path.into() }
  }

  pub fn read(&self) -> io::Result<()> {
    match fs::read(&self.path) {
      Ok(_) => Ok(()),
      Err(error) => Err(error),
    }
  }
}
