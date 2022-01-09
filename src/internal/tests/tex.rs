use crate::internal::tex;

#[test]
fn create_command_name() {
    let result = tex::create_command_name("ttfamily");
    let expected_result = "\\ttfamily".to_string();
    assert_eq!(expected_result, result);
}

#[test]
fn create_key_val_str_with_value() {
    let params = [tex::StringKeyValuePair {
        key: "margin".to_string(),
        value: Option::Some("1in".to_string()),
    }];
    let result = tex::create_key_val_str(&params, tex::PARAMETER_START, tex::PARAMETER_END);
    let expected_result = "[margin=1in]".to_string();
    assert_eq!(expected_result, result);
}

#[test]
fn create_key_val_str_without_value() {
    let params = [tex::StringKeyValuePair {
        key: "hidelinks".to_string(),
        value: None,
    }];
    let result = tex::create_key_val_str(&params, tex::PARAMETER_START, tex::PARAMETER_END);
    let expected_result = "[hidelinks]".to_string();
    assert_eq!(expected_result, result);
}

#[test]
fn create_key_val_str_multiple() {
    let params = [
        tex::StringKeyValuePair {
            key: "a4paper".to_string(),
            value: None,
        },
        tex::StringKeyValuePair {
            key: "12pt".to_string(),
            value: None,
        },
    ];

    let result = tex::create_key_val_str(&params, tex::PARAMETER_START, tex::PARAMETER_END);
    let expected_result = "[a4paper,12pt]".to_string();
    assert_eq!(expected_result, result);
}

#[test]
fn create_command() {
    let params = [
        tex::StringKeyValuePair{key: "toc".to_string(), value: None},
        tex::StringKeyValuePair{key: "page".to_string(), value: None},
        tex::StringKeyValuePair{key: "header".to_string(), value: None},
    ];
    let args = vec![
        vec![tex::StringKeyValuePair{key: "appendix".to_string(), value: None}],
    ];
    let result = tex::create_command("usepackage", Option::Some(&params), Option::Some(args));
    let expected_result = "\\usepackage[toc,page,header]{appendix}".to_string();
    assert_eq!(expected_result, result);
}
