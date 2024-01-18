use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
  /// Write task to journal file
  Add {
    #[structopt()]
    task: String
  },
  /// Remove task by position
  Done {
    #[structopt()]
    position: usize
  },
  /// List all tasks
  List,
}

#[derive(Debug, StructOpt)]
#[structopt(
  name = "Rusty Journal",
  about = "A command line tool"
)]
pub struct CommandLineArgs {
  #[structopt(subcommand)]
  pub action: Action,
  #[structopt(parse(from_os_str), short, long)]
  pub journal_file: Option<PathBuf>,
}