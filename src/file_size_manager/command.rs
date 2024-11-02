mod options;

use crate::file_size_manager::ParseArgError;
use options::{get_option, is_option, Opt};
use std::env::{self, Args};

pub enum Command {
    Split {
        entries: Vec<String>,
        options: Vec<Opt>,
    },
    Merge {
        entries: Vec<String>,
        options: Vec<Opt>,
    },
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
        let (entries, options): (Vec<String>, Vec<Opt>) = Command::parse_args(args)?;
        Ok(Self::Split { entries, options })
    }

    fn new_merge(args: Args) -> Result<Self, ParseArgError> {
        let (entries, options): (Vec<String>, Vec<Opt>) = Command::parse_args(args)?;
        Ok(Self::Merge { entries, options })
    }

    fn parse_args(mut args: Args) -> Result<(Vec<String>, Vec<Opt>), ParseArgError> {
        let mut entries: Vec<String> = Vec::new();
        let mut options: Vec<Opt> = Vec::new();
        while let Some(arg) = args.next() {
            if is_option(&arg) {
                options.push(get_option(&arg, &mut args)?);
            }
            entries.push(arg);
        }
        Ok((entries, options))
    }
}
