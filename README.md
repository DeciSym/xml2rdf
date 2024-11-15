# xml2rdf
CLI tool for converting XML to RDF

This Rust-based tool converts JSON data into RDF format, utilizing the `oxrdf` crate for RDF graph handling and `xml-rs` for efficient XML parsing. Generated triples can either be added to an `oxrdf::Graph` or written directly to file.
