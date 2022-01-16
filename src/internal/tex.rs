use std::string::String;
use std::vec::Vec;

pub const PARAMETER_START: char = '[';
pub const PARAMETER_END: char = ']';
pub const ARGUMENT_START: char = '{';
pub const ARGUMENT_END: char = '}';

/// A key-value pair used for
/// holding parameters and arguments
pub struct StringKeyValuePair {
    pub key: String,
    pub value: Option<String>,
}

/// Returns the latex command name
///
/// # Arguments
///
/// * `name` - The commands name
pub fn create_command_name(name: &str) -> String {
    let result = format!("\\{}", name);
    return result;
}

/// Returns a joined key-value string,
/// with a start and end character
///
/// # Arguments
///
/// * `pairs` - The key-value pairs array
/// * `start` - Character to place at the start
/// * `end` - Character to place at the end
pub fn create_key_val_str(pairs: &[StringKeyValuePair], start: char, end: char) -> String {
    let mut result = String::new();
    let mut at_start = true;
    result.push(start);
    for p in pairs {
        if at_start == false {
            result.push_str(",");
        }
        result.push_str(&p.key);
        if let Some(v) = &p.value {
            result.push_str(&format!("={}", v));
        }
        at_start = false;
    }
    result.push(end);
    return result;
}

/// Returns a latex parameter from given key-values,
/// is a wrapper for `create_key_val_str`
///
/// # Arguments
///
/// * `parameters` - The key-value pairs as an array
pub fn create_parameter(parameters: &[StringKeyValuePair]) -> String {
    return create_key_val_str(parameters, PARAMETER_START, PARAMETER_END);
}

/// Returns a latex argument from given key-values,
/// is a wrapper for `create_key_val_str`
///
/// # Arguments
///
/// * `arguments` - The key-value pairs as an array
pub fn create_argument(arguments: &[StringKeyValuePair]) -> String {
    return create_key_val_str(arguments, ARGUMENT_START, ARGUMENT_END);
}

/// Returns a combined/chained vector of key-value arrays,
/// used to create multiple latex arguments
///
/// # Arguments
///
/// * `chained_arguments` - The arguments to chain together as key-value pairs
pub fn create_chained_argument(chained_arguments: Vec<Vec<StringKeyValuePair>>) -> String {
    let mut result = String::new();
    for key_value in chained_arguments {
        result.push_str(&create_argument(&key_value));
    }
    return result;
}

/// Returns a latex command
///
/// # Arguments
///
/// * `name` - The name of the command
/// * `parameters` - Any parameters given as a array
/// * `arguments` - Any arguments given as a 2D vector
pub fn create_command(
    name: &str,
    parameters: Option<&[StringKeyValuePair]>,
    arguments: Option<Vec<Vec<StringKeyValuePair>>>,
) -> String {
    let mut result = create_command_name(name);

    if let Some(p) = parameters {
        result.push_str(&create_parameter(p));
    }

    if let Some(a) = arguments {
        result.push_str(&create_chained_argument(a));
    }

    return result;
}

/// Returns the given string to an
/// escaped version for a tex file
///
/// # Arguments
/// * `to_escape` - the string to escape reserved characters
pub fn escape_reserved(to_escape: &str) -> String {
    return to_escape
        .replace("\\", "\\textbackslash")
        .replace("#", "\\#")
        .replace("$", "\\$")
        .replace("%", "\\%")
        .replace("^", "\\^")
        .replace("&", "\\&")
        .replace("_", "\\_")
        .replace("{", "\\}")
        .replace("~", "\\~");
}
