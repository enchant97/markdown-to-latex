use std::io::prelude::{BufRead, Write};
use std::string::String;

use crate::internal::markdown;
use crate::internal::metadata;
use crate::internal::tex;

pub const YAML_HEADER_MARKER: &str = "---";

pub fn get_latex_preamble(meta: metadata::Metadata) -> String {
    let mut header = String::new();

    header.push_str(
        tex::create_command(
            "documentclass",
            Option::Some(&[
                tex::StringKeyValuePair {
                    key: meta.paper_size,
                    value: None,
                },
                tex::StringKeyValuePair {
                    key: meta.font_size,
                    value: None,
                },
            ]),
            Option::Some(vec![vec![tex::StringKeyValuePair {
                key: "report".to_string(),
                value: None,
            }]]),
        )
        .as_str(),
    );
    header.push_str(
        tex::create_command(
            "usepackage",
            Option::Some(&[tex::StringKeyValuePair {
                key: "margin".to_string(),
                value: Option::Some(meta.margin),
            }]),
            Option::Some(vec![vec![tex::StringKeyValuePair {
                key: "geometry".to_string(),
                value: None,
            }]]),
        )
        .as_str(),
    );
    header.push_str(
        tex::create_command(
            "usepackage",
            None,
            Option::Some(vec![vec![tex::StringKeyValuePair {
                key: "fontspec".to_string(),
                value: None,
            }]]),
        )
        .as_str(),
    );
    header.push_str(
        tex::create_command(
            "setmainfont",
            None,
            Option::Some(vec![vec![tex::StringKeyValuePair {
                key: meta.font_family,
                value: None,
            }]]),
        )
        .as_str(),
    );
    header.push_str(
        tex::create_command(
            "usepackage",
            None,
            Option::Some(vec![vec![tex::StringKeyValuePair {
                key: "graphicx".to_string(),
                value: None,
            }]]),
        )
        .as_str(),
    );
    header.push_str(
        tex::create_command(
            "usepackage",
            None,
            Option::Some(vec![vec![tex::StringKeyValuePair {
                key: "placeins".to_string(),
                value: None,
            }]]),
        )
        .as_str(),
    );
    header.push_str(
        tex::create_command(
            "title",
            None,
            Option::Some(vec![vec![tex::StringKeyValuePair {
                key: meta.title,
                value: None,
            }]]),
        )
        .as_str(),
    );
    header.push_str(
        tex::create_command(
            "author",
            None,
            Option::Some(vec![vec![tex::StringKeyValuePair {
                key: meta.author,
                value: None,
            }]]),
        )
        .as_str(),
    );
    return header;
}

pub fn get_latex_doc_start() -> String {
    return "\\maketitle\\tableofcontents\\newpage".to_string();
}

fn process_text(line: &str) -> String {
    return tex::escape_reserved(line);
}

fn process_chapter(content: &str) -> String {
    let content = tex::escape_reserved(content);
    return tex::create_command(
        "chapter",
        None,
        Some(vec![vec![tex::StringKeyValuePair {
            key: content,
            value: None,
        }]]),
    );
}

fn process_section(content: &str, number: usize) -> Result<String, String> {
    if number < 1 || number > 3 {
        return Err("invalid section number must be 1-3".to_string());
    }
    let content = tex::escape_reserved(content);
    let command_name = match number {
        1 => "section",
        2 => "subsection",
        3 => "subsubsection",
        _ => panic!(),
    };
    return Ok(tex::create_command(
        command_name,
        None,
        Some(vec![vec![tex::StringKeyValuePair {
            key: content,
            value: None,
        }]]),
    ));
}

fn process_header(content: &str, count: usize) -> String {
    return match count {
        1 => process_chapter(content),
        2..=4 => process_section(content, count - 1).unwrap(),
        _ => process_text(content),
    };
}

fn process_line(line: &str) -> String {
    let mut processed_line: String;

    if line.len() == 0 {
        processed_line = String::new();
    } else if let Some(content) = markdown::is_heading_match(line) {
        processed_line = process_header(content.content.as_str(), content.count);
    } else {
        processed_line = process_text(line);
    }

    processed_line.push_str("\n");
    return processed_line;
}

/// Process the source (markdown file) and
/// output converted data the destination source
pub fn process<S: BufRead, D: Write>(src_stream: S, dst_stream: &mut D) {
    // TODO handle errors better than using 'unwrap'
    let mut meta_buffer = String::new();
    let mut at_start = true;
    let mut in_header = false;

    for line in src_stream.lines() {
        let current_line = line.unwrap();

        if at_start && current_line == YAML_HEADER_MARKER {
            in_header = true;
        } else if in_header && current_line == YAML_HEADER_MARKER {
            in_header = false;
            let loaded_metadata = metadata::load_from_yml(&meta_buffer);
            dst_stream
                .write(get_latex_preamble(loaded_metadata).as_bytes())
                .unwrap();
            dst_stream.write(b"\\begin{document}").unwrap();
            dst_stream.write(get_latex_doc_start().as_bytes()).unwrap();
            dst_stream.write(b"\n").unwrap();
        } else if !at_start && in_header {
            meta_buffer.push_str(format!("{}\n", &current_line).as_str());
        } else {
            let processed_line = process_line(current_line.as_str());
            dst_stream.write(processed_line.as_bytes()).unwrap();
        }

        at_start = false;
    }
    dst_stream.write(b"\\end{document}").unwrap();
    dst_stream.flush().unwrap();
}
