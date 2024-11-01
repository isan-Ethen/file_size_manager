use crate::file_size_manager::ParseArgError;
use std::collections::HashMap;
use std::sync::LazyLock;

static OPT_DICT: LazyLock<HashMap<String, u8>> = LazyLock::new(|| {
    let map = HashMap::new();
    map
});

pub fn is_option(arg: &String) -> bool {
    arg.starts_with("-")
}

pub fn get_option(arg: &String) -> Result<u8, ParseArgError> {
    if let Some(opt) = OPT_DICT.get(arg) {
        Ok(*opt)
    } else {
        Err(ParseArgError::UnknownOption(format!(
            "unknown option provided: {}",
            arg
        )))
    }
}
