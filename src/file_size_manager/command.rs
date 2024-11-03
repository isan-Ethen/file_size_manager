mod options;

use crate::file_size_manager::ParseArgError;
pub use options::{Key, Opt};
use std::collections::HashMap;
use std::env::{self, Args};

pub enum Command {
    Split {
        entries: Vec<String>,
        options: HashMap<Key, Opt>,
    },
    Merge {
        entries: Vec<String>,
        options: HashMap<Key, Opt>,
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
        let (entries, options): (Vec<String>, HashMap<Key, Opt>) = Command::parse_args(args)?;
        Ok(Self::Split { entries, options })
    }

    fn new_merge(args: Args) -> Result<Self, ParseArgError> {
        let (entries, options): (Vec<String>, HashMap<Key, Opt>) = Command::parse_args(args)?;
        Ok(Self::Merge { entries, options })
    }

    fn parse_args(mut args: Args) -> Result<(Vec<String>, HashMap<Key, Opt>), ParseArgError> {
        let mut entries: Vec<String> = Vec::new();
        let mut options: HashMap<Key, Opt> = HashMap::new();
        while let Some(arg) = args.next() {
            if Opt::is_option(&arg) {
                let (key, value) = Opt::get_option(&arg, &mut args)?;
                options.insert(key, value);
                continue;
            }
            entries.push(arg);
        }
        Ok((entries, options))
    }
}
