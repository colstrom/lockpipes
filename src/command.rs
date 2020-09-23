use crate::{LockPipe, Program};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Command {
  #[structopt(
    long,
    short,
    env = "LOCKPIPE_PATH",
    default_value = "/run/forever",
    help = "sets the path for the pipe"
  )]
  path: PathBuf,
  #[structopt(subcommand)]
  action: Action,
}

impl Command {
  pub fn execute(&self) -> i32 {
    let lockpipe = LockPipe::new(&self.path);
    let program = Program::new(lockpipe);

    self.action.execute(&program)
  }
}

#[derive(Debug, StructOpt)]
pub enum Action {
  #[structopt(alias = "c", about = "Creates a new LockPipe")]
  Create,
  #[structopt(alias = "d", about = "Deletes an existing LockPipe")]
  Delete,
  #[structopt(alias = "e", about = "Checks if a LockPipe exists")]
  Exists,
  #[structopt(alias = "r", about = "Reads from an existing LockPipe")]
  Read,
  #[structopt(alias = "w", about = "Writes to an existing LockPipe")]
  Write,
}

impl Action {
  pub fn execute(&self, program: &Program) -> i32 {
    match self {
      Self::Create => program.create(),
      Self::Delete => program.delete(),
      Self::Exists => program.exists(),
      Self::Read => program.read(),
      Self::Write => program.write(),
    }
  }
}
