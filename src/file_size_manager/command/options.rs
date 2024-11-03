use crate::file_size_manager::ParseArgError;
use std::env::Args;

#[allow(unused)]
#[derive(Hash, Eq, PartialEq)]
pub enum Key {
    Size,
    Output,
    Follow,
}

#[allow(unused)]
pub enum Opt {
    Size(u64),
    Output(String),
    Follow,
}

impl Opt {
    pub fn is_option(arg: &String) -> bool {
        arg.starts_with("-")
    }

    pub fn get_option(arg: &String, args: &mut Args) -> Result<(Key, Opt), ParseArgError> {
        match arg.as_ref() {
            "-s" => match args.next() {
                Some(next) => {
                    let size: Opt = Opt::get_size(next)?;
                    Ok((Key::Size, size))
                }
                None => Err(ParseArgError::MissingArgument("-s".to_string())),
            },
            "-o" => match args.next() {
                Some(next) => Ok((Key::Output, Opt::Output(next))),
                None => Err(ParseArgError::MissingArgument("-o".to_string())),
            },
            "-f" => Ok((Key::Follow, Opt::Follow)),
            _ => Err(ParseArgError::InvalidOption(format!(
                "Unknown option: {}",
                arg
            ))),
        }
    }

    fn get_size(arg: String) -> Result<Opt, ParseArgError> {
        match arg.parse::<u64>() {
            Ok(num) => Ok(Opt::Size(num)),
            Err(err) => Err(ParseArgError::InvalidOption(format!(
                "Invalid number for -s: {}",
                err
            ))),
        }
    }
}
