use std::path::PathBuf;

#[derive(Debug)]
pub struct LockPipe {
  path: PathBuf,
}

impl LockPipe {
  pub fn new<P: Into<PathBuf>>(path: P) -> Self {
    Self { path: path.into() }
  }
}
