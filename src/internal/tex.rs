use std::string::String;
use std::vec::Vec;

pub const PARAMETER_START: char = '[';
pub const PARAMETER_END: char = ']';
pub const ARGUMENT_START: char = '{';
pub const ARGUMENT_END: char = '}';

pub struct StringKeyValuePair {
    pub key: String,
    pub value: Option<String>,
}

pub fn create_command_name(name: &str) -> String {
    let result = format!("\\{}", name);
    return result;
}

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

pub fn create_parameter(parameters: &[StringKeyValuePair]) -> String {
    return create_key_val_str(parameters, PARAMETER_START, PARAMETER_END);
}

pub fn create_argument(arguments: &[StringKeyValuePair]) -> String {
    return create_key_val_str(arguments, ARGUMENT_START, ARGUMENT_END);
}

pub fn create_chained_argument(chained_arguments: Vec<Vec<StringKeyValuePair>>) -> String {
    let mut result = String::new();
    for key_value in chained_arguments {
        result.push_str(&create_argument(&key_value));
    }
    return result;
}

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
