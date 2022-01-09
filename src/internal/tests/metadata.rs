use crate::internal::metadata;

#[test]
fn load_from_yml() {
    let yml_source = "paper_size: a4paper\n\
    font_size: 12pt\n\
    document_type: report\n\
    margin: 1in\n\
    font_family: Carlito\n\
    title: My Report\n\
    author: Leo";

    let result = metadata::load_from_yml(&yml_source);
    assert_eq!(result.paper_size, "a4paper");
    assert_eq!(result.font_size, "12pt");
    assert_eq!(result.doc_type, "report");
    assert_eq!(result.margin, "1in");
    assert_eq!(result.font_family, "Carlito");
    assert_eq!(result.title, "My Report");
    assert_eq!(result.author, "Leo");
}
