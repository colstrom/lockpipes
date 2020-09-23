use nix::errno::errno;
use nix::errno::Errno;
use nix::sys::stat;
use nix::unistd;
use nix::Error as NixError;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

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

#[derive(Debug)]
pub struct Program {
  pipe: LockPipe,
}

impl Program {
  pub fn new(pipe: LockPipe) -> Self {
    Self { pipe }
  }

  pub fn create(&self) -> i32 {
    log::debug!("creating pipe at {:?}", &self.pipe.path);

    match self.pipe.create() {
      Ok(_) => {
        log::info!("created pipe at {:?}", &self.pipe.path);
        0
      }
      Err(error) => match error {
        NixError::Sys(Errno::EEXIST) => {
          log::warn!("pipe already exists at {:?}", &self.pipe.path);
          0
        }
        _ => {
          log::error!(
            "failed to create pipe at {:?}: {:?}",
            &self.pipe.path,
            error
          );
          errno()
        }
      },
    }
  }

  pub fn exists(&self) -> i32 {
    log::debug!("checking if pipe exists at {:?}", &self.pipe.path);

    match self.pipe.exists() {
      Ok(_) => {
        log::info!("pipe exists at {:?}", &self.pipe.path);
        0
      }
      Err(error) => match error.kind() {
        io::ErrorKind::NotFound => {
          log::info!("pipe does not exist at {:?}", &self.pipe.path);
          1
        }
        _ => {
          log::error!("failed checking if pipe exists at {:?}", &self.pipe.path);
          errno()
        }
      },
    }
  }

  fn ensure_exists(&self) {
    log::debug!("ensuring pipe exists at {:?}", &self.pipe.path);

    match self.pipe.exists() {
      Ok(_) => log::info!("pipe exists at {:?}", &self.pipe.path),
      Err(error) => match error.kind() {
        io::ErrorKind::NotFound => {
          log::warn!("pipe does not exist at {:?}", &self.pipe.path);
          self.create();
        }
        _ => {
          log::error!("failed checking if pipe exists at {:?}", &self.pipe.path);
          process::exit(errno());
        }
      },
    }
  }

  pub fn read(&self) -> i32 {
    self.ensure_exists();

    log::debug!("reading from pipe at {:?}", &self.pipe.path);

    match self.pipe.read() {
      Ok(_) => {
        log::info!("read from pipe at {:?}", &self.pipe.path);
        0
      }
      Err(error) => {
        log::error!(
          "failed reading from pipe at {:?}: {:?}",
          &self.pipe.path,
          error
        );
        errno()
      }
    }
  }

  pub fn write(&self) -> i32 {
    self.ensure_exists();

    log::debug!("writing to pipe at {:?}", &self.pipe.path);

    match self.pipe.write() {
      Ok(_) => {
        log::info!("wrote to pipe at {:?}", &self.pipe.path);
        0
      }
      Err(error) => {
        log::error!(
          "failed writing to pipe at {:?}: {:?}",
          &self.pipe.path,
          error
        );
        errno()
      }
    }
  }

  pub fn delete(&self) -> i32 {
    match self.pipe.delete() {
      Ok(_) => {
        log::info!("deleted pipe at {:?}", &self.pipe.path);
        0
      }

      Err(error) => match error.kind() {
        io::ErrorKind::NotFound => {
          log::warn!("pipe does not exist at {:?}", &self.pipe.path);
          0
        }
        _ => {
          log::error!(
            "failed to delete pipe at {:?}: {:?}",
            &self.pipe.path,
            error
          );
          errno()
        }
      },
    }
  }
}
