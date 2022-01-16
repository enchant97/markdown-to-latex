use crate::internal::markdown;

#[test]
fn is_heading_match() {
    let section_line = "# My Chapter";
    let expected_result = markdown::HeadingMatch {
        content: "My Chapter".to_string(),
        count: 1,
    };
    let result = markdown::is_heading_match(section_line).unwrap();
    assert_eq!(result.content, expected_result.content);
    assert_eq!(result.count, expected_result.count);
}

#[test]
fn is_heading_match_section() {
    let section_line = "## My Section";
    let expected_result = markdown::HeadingMatch {
        content: "My Section".to_string(),
        count: 2,
    };
    let result = markdown::is_heading_match(section_line).unwrap();
    assert_eq!(result.content, expected_result.content);
    assert_eq!(result.count, expected_result.count);
}

#[test]
fn is_heading_match_no_match() {
    let section_line = "- A bullet list line";
    let result = markdown::is_heading_match(section_line);
    assert!(result.is_none());
}

#[test]
fn is_unordered_list_match() {
    let line = "- A bullet list line";
    let result = markdown::is_unordered_list_match(line);
    assert_eq!(Some("A bullet list line".to_string()), result);
}

#[test]
fn is_unordered_list_match_no_match() {
    let line = "1. Not a bullet list line";
    let result = markdown::is_unordered_list_match(line);
    assert_eq!(None, result);
}

#[test]
fn is_ordered_list_match() {
    let line = "1. A numbered list line";
    let result = markdown::is_ordered_list_match(line);
    assert_eq!(Some("A numbered list line".to_string()), result);
}

#[test]
fn is_ordered_list_match_no_match() {
    let line = "- Not a list line";
    let result = markdown::is_ordered_list_match(line);
    assert_eq!(None, result);
}
