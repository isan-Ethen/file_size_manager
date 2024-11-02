use crate::file_size_manager::ParseArgError;
use std::env::Args;

pub enum Opt {
    Size(usize),
    Output(String),
    Follow,
}

pub fn is_option(arg: &String) -> bool {
    arg.starts_with("-")
}

pub fn get_option(arg: &String, args: &mut Args) -> Result<Opt, ParseArgError> {
    match arg.as_ref() {
        "-s" => match args.next() {
            Some(next) => match next.parse::<usize>() {
                Ok(num) => Ok(Opt::Size(num)),
                Err(err) => Err(ParseArgError::InvalidOption(format!(
                    "Invalid number for -s: {}",
                    err
                ))),
            },
            None => Err(ParseArgError::MissingArgument("-s".to_string())),
        },
        "-o" => match args.next() {
            Some(next) => Ok(Opt::Output(next)),
            None => Err(ParseArgError::MissingArgument("-o".to_string())),
        },
        "-f" => Ok(Opt::Follow),
        _ => Err(ParseArgError::InvalidOption(format!(
            "Unknown option: {}",
            arg
        ))),
    }
}
