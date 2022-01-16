use std::string::String;
use yaml_rust::{Yaml, YamlLoader};

/// A documents metadata used in conversion
pub struct Metadata {
    pub paper_size: String,
    pub font_size: String,
    pub doc_type: String,
    pub margin: String,
    pub font_family: String,
    pub title: String,
    pub author: String,
}

/// Returns a loaded Metadata from a yaml file
///
/// # Arguments
/// * `source` - The yaml file source
pub fn load_from_yml(source: &str) -> Metadata {
    let docs = YamlLoader::load_from_str(source).unwrap();
    let mut doc: &Yaml = &Yaml::from_str("");

    if docs.len() != 0 {
        doc = &docs[0];
    }

    return Metadata {
        paper_size: doc["paper_size"].as_str().unwrap_or("a4paper").to_string(),
        font_size: doc["font_size"].as_str().unwrap_or("12pt").to_string(),
        doc_type: doc["document_type"]
            .as_str()
            .unwrap_or("article")
            .to_string(),
        margin: doc["margin"].as_str().unwrap_or("1in").to_string(),
        font_family: doc["font_family"]
            .as_str()
            .unwrap_or("freesans")
            .to_string(),
        title: doc["title"].as_str().unwrap_or("Untitled").to_string(),
        author: doc["author"].as_str().unwrap_or("No Author").to_string(),
    };
}
