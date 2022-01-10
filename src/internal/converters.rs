use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};
use std::string::String;

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

pub fn process(src_path: &str, dst_path: &str) {
    // TODO handle errors better than using 'unwrap'
    let src_file = File::open(src_path).unwrap();
    let src_stream = BufReader::new(src_file);
    let dst_file = File::create(dst_path).unwrap();
    let mut dst_stream = BufWriter::new(dst_file);

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
            dst_stream
                .write(format!("{}\n", &current_line).as_bytes())
                .unwrap();
        }

        at_start = false;
    }
    dst_stream.write(b"\\end{document}").unwrap();
    dst_stream.flush().unwrap();
}
