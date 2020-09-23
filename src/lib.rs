use nix::sys::stat;
use nix::unistd;
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

  pub fn write(&self) -> io::Result<()> {
    fs::write(&self.path, "")
  }

  pub fn exists(&self) -> io::Result<()> {
    match fs::metadata(&self.path) {
      Ok(_) => Ok(()),
      Err(error) => Err(error),
    }
  }

  pub fn create(&self) -> nix::Result<()> {
    unistd::mkfifo(&self.path, stat::Mode::S_IRWXU)
  }

  pub fn delete(&self) -> io::Result<()> {
    fs::remove_file(&self.path)
  }
}
