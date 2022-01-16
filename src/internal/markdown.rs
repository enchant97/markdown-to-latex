use lazy_static::lazy_static;
use regex::{Captures, Regex};
use unicode_segmentation::UnicodeSegmentation;

const HEADING_RE: &str = r"^(#{1,4}) (.+)$";
const UNORDERED_LIST_RE: &str = r"^[-] (.*)$";
const ORDERED_LIST_RE: &str = r"^[1-9]\d*\. (.*)$";

/// Struct returned when a heading is matched
pub struct HeadingMatch {
    /// The heading's content
    pub content: String,
    /// What heading count e.g. ## would be 2
    pub count: usize,
}

fn get_section_from_match(capture: Captures) -> HeadingMatch {
    let markers = capture.get(1).unwrap().as_str();
    let content = capture.get(2).unwrap().as_str().to_string();
    let count = markers.graphemes(true).count();

    return HeadingMatch {
        content: content,
        count: count,
    };
}

/// Check whether a heading is found, if so returns the heading's content
///
/// # Argument
/// * 'src_line' - The text to check for a heading
pub fn is_heading_match(src_line: &str) -> Option<HeadingMatch> {
    lazy_static! {
        static ref RE: Regex = Regex::new(HEADING_RE).unwrap();
    }
    return match RE.captures(src_line) {
        Some(capture) => Some(get_section_from_match(capture)),
        None => None,
    };
}

/// Check whether a list is found, if so returns the list content
///
/// # Argument
/// * 'src_line' - The text to check for a heading
pub fn is_unordered_list_match(src_line: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(UNORDERED_LIST_RE).unwrap();
    }
    return match RE.captures(src_line) {
        Some(capture) => Some(capture.get(1).unwrap().as_str().to_string()),
        None => None,
    };
}

/// Check whether a list is found, if so returns the list content
///
/// # Argument
/// * 'src_line' - The text to check for a heading
pub fn is_ordered_list_match(src_line: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(ORDERED_LIST_RE).unwrap();
    }
    return match RE.captures(src_line) {
        Some(capture) => Some(capture.get(1).unwrap().as_str().to_string()),
        None => None,
    };
}
