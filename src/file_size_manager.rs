mod command;
mod errors;
mod merger;
mod splitter;

pub use command::Command;
pub use errors::{ParseArgError, RunCommandError};
use merger::Merger;
use splitter::Splitter;

pub struct FileSizeManager {}

impl FileSizeManager {
    pub fn run() {
        match Command::get_sub_command() {
            Ok(command) => FileSizeManager::run_sub_command(command),
            Err(err) => FileSizeManager::print_errors(err),
        }
    }

    fn run_sub_command(command: Command) {
        match command {
            Command::Split { entries, options } => Splitter::run(entries.into_iter(), options),
            Command::Merge { entries, options } => Merger::run(entries.into_iter(), options),
        }
    }

    fn print_errors(err: ParseArgError) {
        eprintln!("error occured: {}", err)
    }
}
