mod command;
mod errors;
mod merger;
mod splitter;
mod util;

pub use command::Command;
pub use errors::ParseArgError;
use merger::Merger;
use splitter::Splitter;

pub struct FileSizeManager {}

impl FileSizeManager {
    pub fn run() {
        match Command::get_sub_command() {
            Ok(command) => FileSizeManager::run_sub_command(command),
            Err(err) => (),
        }
    }

    pub fn run_sub_command(command: Command) {
        match command {
            Command::Split { entries, options } => {
                Splitter::run(entries, options).unwrap();
            }
            Command::Merge { entries, options } => {
                Merger::run(entries, options).unwrap();
            }
        }
    }
}
