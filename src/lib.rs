pub mod command;
pub mod logging;

pub use command::Command;
pub use nix;

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

/// The low-level interface for lockpipes, providing the core functionality,
/// with minimal error handling.

impl LockPipe {
  pub fn new<P: Into<PathBuf>>(path: P) -> Self {
    Self { path: path.into() }
  }

  /// consumes and discards the data in the pipe. It blocks if there are no
  /// writers. Multiple LockPipe instances may read from the same pipe, all will
  /// be unblocked when anything writes to it.

  pub fn read(&self) -> io::Result<()> {
    match fs::read(&self.path) {
      Ok(_) => Ok(()),
      Err(error) => Err(error),
    }
  }

  /// writes an empty string to the pipe. It blocks if there are no readers.
  /// Multiple LockPipe instances may write to the same pipe, all will be
  /// unblocked when anything reads from it.

  pub fn write(&self) -> io::Result<()> {
    fs::write(&self.path, "")
  }

  /// checks if the pipe exists, sort of. It checks that *something* exists at
  /// that path, not that it's specifically a pipe. It probably should. Patches
  /// to improve this are welcome.

  pub fn exists(&self) -> io::Result<()> {
    match fs::metadata(&self.path) {
      Ok(_) => Ok(()),
      Err(error) => Err(error),
    }
  }

  /// creates a named pipe at the given path.

  pub fn create(&self) -> nix::Result<()> {
    unistd::mkfifo(&self.path, stat::Mode::S_IRWXU)
  }

  /// deletes whatever is at the given path. It makes no attempt to verify that
  /// the thing being deleted is actually a pipe.

  pub fn delete(&self) -> io::Result<()> {
    fs::remove_file(&self.path)
  }
}

#[derive(Debug)]
pub struct Program {
  pipe: LockPipe,
}

/// The high-level interface for lockpipes, wrapping a LockPipe and providing
/// error handling and logging.

impl Program {
  pub fn new(pipe: LockPipe) -> Self {
    Self { pipe }
  }

  /// attempts to create the pipe. If something already exists there, it is
  /// assumed to be the right thing, and not effort is made to confirm that it
  /// actually is a pipe.

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

  /// checks if the pipe exists.

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

  /// internal function that checks if the pipe exists, creates it if not, and
  /// exits the program if it can't do that.

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

  /// reads from the pipe, returning errno if that for any reason.

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

  /// writes to the pipe, returning errno if that fails for any reason.

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

  /// attempts to delete the pipe. If it doesn't exist, it considers this to be
  /// a success, since you asked it to make the path not exist. If it fails for
  /// any other reason, it returns errno.

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
