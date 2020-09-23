use lockpipes::{logging, Command};
use structopt::StructOpt;

fn main() {
  logging::init();
  let status = Command::from_args().execute();
  std::process::exit(status);
}
