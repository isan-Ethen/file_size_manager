mod options;

use crate::file_size_manager::ParseArgError;
use options::{get_option, is_option};
use std::env::{self, Args};

pub enum Command {
    Split { entries: Vec<String>, options: u8 },
    Merge { entries: Vec<String>, options: u8 },
}

impl Command {
    pub fn get_sub_command() -> Result<Self, ParseArgError> {
        let mut args: Args = env::args().into_iter();

        args.next();

        if let Some(fname) = args.next() {
            match fname.as_str() {
                "split" => Ok(Command::new_split(args)?),
                "merge" => Ok(Command::new_merge(args)?),
                _ => Err(ParseArgError::InvalidSubFunction(format!(
                    "unknown function provided: {}",
                    fname
                ))),
            }
        } else {
            Err(ParseArgError::InvalidSubFunction(
                "no function provided".into(),
            ))
        }
    }

    fn new_split(args: Args) -> Result<Self, ParseArgError> {
        let (entries, options): (Vec<String>, u8) = Command::parse_args(args)?;
        Ok(Self::Split { entries, options })
    }

    fn new_merge(args: Args) -> Result<Self, ParseArgError> {
        let (entries, options): (Vec<String>, u8) = Command::parse_args(args)?;
        Ok(Self::Merge { entries, options })
    }

    fn parse_args(args: Args) -> Result<(Vec<String>, u8), ParseArgError> {
        let mut entries: Vec<String> = Vec::new();
        let mut options: u8 = 0;
        for arg in args {
            if is_option(&arg) {
                options += get_option(&arg)?;
            }
            entries.push(arg);
        }
        Ok((entries, options))
    }
}
