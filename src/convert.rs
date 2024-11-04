//! # XML2RDF Converter Library
//!
//! This library provides functionality for converting XML data into RDF format.
//! It uses `xml-rs` for XML parsing and `oxrdf` to build and manage RDF graphs.
//!
//! ## Overview
//! - Converts XML data structures into RDF triples, generating a graph representation.
//!
//! ## Features
//! - Converts nested XML Objects into RDF triples.
//! - Allows specifying a custom RDF namespace for generated predicates and objects.
//! - Outputs the RDF data to a specified file.

use const_format::concatcp;
use oxrdf::vocab::rdf::TYPE;
use oxrdf::vocab::rdfs::SUB_CLASS_OF;
use oxrdf::{Literal, NamedNode, NamedNodeRef, TermRef, TripleRef};
use uuid::Uuid;
use xml::reader::{EventReader, XmlEvent};

use crate::writer::RdfWriter;

#[derive(Debug, Clone)]
struct Node {
    path: String,
    id: NamedNode,
}

const X2R: &'static str = "https://decisym.ai/xml2rdf/model#";

const XML_ELEMENT: NamedNodeRef<'_> = NamedNodeRef::new_unchecked(concatcp!(X2R, "XmlNode"));
const XML_ATTRIBUTE: NamedNodeRef<'_> = NamedNodeRef::new_unchecked(concatcp!(X2R, "XmlAttribute"));
const HAS_CHILD: NamedNodeRef<'_> = NamedNodeRef::new_unchecked(concatcp!(X2R, "hasChild"));
const HAS_ATTRIBUTE: NamedNodeRef<'_> = NamedNodeRef::new_unchecked(concatcp!(X2R, "hasAttribute"));
const HAS_NAME: NamedNodeRef<'_> = NamedNodeRef::new_unchecked(concatcp!(X2R, "hasName"));
const HAS_VALUE: NamedNodeRef<'_> = NamedNodeRef::new_unchecked(concatcp!(X2R, "hasValue"));

/// Converts XML data to RDF format.
///
/// This function reads XML data from the specified file, processes it into RDF triples,
/// and outputs the RDF graph. Users can specify a namespace to use for RDF predicates and
/// an output file for saving the generated RDF data.
///
/// # Arguments
/// - `files`: Path to the XML file.
/// - `namespace`: Optional custom namespace for RDF predicates.
/// - `output`: use RdfWriter trait to add generated triples to desired format (File or Graph)
///
/// # Example
/// ```rust
/// use xml2rdf::convert::parse_xml;
/// use xml2rdf::writer;
/// use oxrdf::Graph;
///
/// let mut w = writer::FileWriter::new("output.nt".to_string()).unwrap();
/// parse_xml(vec!["data.xml".to_string()], &mut w, "https://decisym.ai/xml2rdf/data");
///
/// let mut g = Graph::new();
/// let mut w = writer::GraphWriter::new(&mut g);
/// parse_xml(vec!["data.xml".to_string()], &mut w, "https://decisym.ai/xml2rdf/data");
/// ```
pub fn parse_xml(
    files: Vec<String>,
    output: &mut dyn RdfWriter,
    namespace: &str,
) -> std::io::Result<()> {
    for file in files.into_iter() {
        // Initialize XML parser
        let file = std::fs::File::open(file)?;
        let file_reader = std::io::BufReader::new(file);
        let parser = EventReader::new(file_reader);

        let mut stack: Vec<Node> = Vec::new();
        let mut subject: Option<Node> = None;

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    // Define the subject as the IRI of the element
                    let id = Uuid::new_v4().hyphenated().to_string();
                    let path = if let Some(parent) = stack.last_mut() {
                        format!("{}.{}", parent.path, name.local_name)
                    } else {
                        format!("{X2R}{}", name.local_name)
                    };
                    subject = Some(Node {
                        id: NamedNode::new(format!("{}/{}", namespace, id).as_str()).unwrap(),
                        path,
                    });

                    if let Some(ref s) = subject {
                        if let Some(parent) = stack.last_mut() {
                            output.add_triple(TripleRef::new(
                                parent.id.as_ref(),
                                HAS_CHILD,
                                s.id.as_ref(),
                            ))?;
                        }
                        let object = Literal::new_simple_literal(s.path.clone());
                        output.add_triple(TripleRef::new(
                            s.id.as_ref(),
                            TYPE,
                            TermRef::Literal(object.as_ref()),
                        ))?;

                        let object = Literal::new_simple_literal(name.local_name.clone());
                        output.add_triple(TripleRef::new(
                            s.id.as_ref(),
                            HAS_NAME,
                            TermRef::Literal(object.as_ref()),
                        ))?;

                        output.add_triple(TripleRef::new(
                            s.id.as_ref(),
                            SUB_CLASS_OF,
                            XML_ELEMENT,
                        ))?;

                        stack.push(s.clone());
                    }

                    // Write triples for each attribute of the element
                    for attr in attributes {
                        if let Some(ref s) = subject {
                            let attrib_id = Uuid::new_v4().hyphenated().to_string();
                            let path = format!("{}.-{}", s.path, attr.name.local_name);

                            let attr_subject =
                                NamedNode::new(format!("{}/{}", namespace, attrib_id)).unwrap();

                            output.add_triple(TripleRef::new(
                                s.id.as_ref(),
                                HAS_ATTRIBUTE,
                                attr_subject.as_ref(),
                            ))?;

                            let attr_object = NamedNode::new(path).unwrap();
                            output.add_triple(TripleRef::new(
                                attr_subject.as_ref(),
                                TYPE,
                                attr_object.as_ref(),
                            ))?;

                            output.add_triple(TripleRef::new(
                                attr_object.as_ref(),
                                SUB_CLASS_OF,
                                XML_ATTRIBUTE,
                            ))?;

                            if attr.value != "" {
                                let attr_object = Literal::new_simple_literal(&attr.value);

                                output.add_triple(TripleRef::new(
                                    attr_subject.as_ref(),
                                    HAS_VALUE,
                                    TermRef::Literal(attr_object.as_ref()),
                                ))?;
                            } else {
                                print!("warning skipping empty attribute value?")
                            }
                        }
                    }
                }
                Ok(XmlEvent::Characters(text)) => {
                    // Handle text content within the current element
                    let text = text.trim(); // Strip unnecessary whitespace
                    if !text.is_empty() {
                        if let Some(ref s) = subject {
                            let content_object = Literal::new_simple_literal(text);
                            output.add_triple(TripleRef::new(
                                s.id.as_ref(),
                                HAS_VALUE,
                                TermRef::Literal(content_object.as_ref()),
                            ))?;
                        }
                    }
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    stack.pop();
                    subject = None; // Clear the subject when the element ends
                }
                _ => {}
            }
        }
    }

    Ok(())
}
