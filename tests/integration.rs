use oxrdf::Graph;
use oxrdfio::{RdfFormat, RdfParser};
use std::fs;
use std::fs::File;
use xml2rdf::*;

#[test]
fn test_graph_writer() {
    let mut g = Graph::new();
    let mut w = writer::GraphWriter::new(&mut g);

    let res = convert::parse_xml(
        vec!["tests/resources/people.xml".to_string()],
        &mut w,
        "https://decisym.ai/xml2rdf/data",
    );
    assert!(res.is_ok());

    assert_eq!(g.len(), 273)
}

#[test]
fn test_file_writer() {
    let output = "out.nt".to_string();
    let _ = fs::remove_file(output.clone());

    let mut w = writer::FileWriter::new(output.clone()).expect("Failed to open output file");

    let res = convert::parse_xml(
        vec!["tests/resources/people.xml".to_string()],
        &mut w,
        "https://decisym.ai/xml2rdf/data",
    );
    assert!(res.is_ok());
    let f = File::open(output).expect("unable to open output file for result verification");
    let quads = RdfParser::from_format(RdfFormat::NTriples)
        .for_reader(f)
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse generated output file");

    assert_eq!(quads.len(), 273)
}
