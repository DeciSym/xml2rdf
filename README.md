# xml2rdf
CLI tool for converting XML to RDF

This Rust-based tool converts XML data into RDF format, utilizing the `oxrdf` crate for RDF graph handling and `xml-rs` for efficient XML parsing. Generated triples can either be added to an `oxrdf::Graph` or written directly to file.

## Using the xml2rdf CLI

This library includes a CLI utility for parsing XML and generating N-Triple RDF using the `convert` subcommand. The binary can be built using `cargo build`.

```bash
$ xml2rdf convert --help
Convert XML to RDF format.

The `convert` command parses a XML file, converts it to RDF triples using `xml-rs` for parsing and `oxrdf` to construct the graph, and saves the output.

Usage: xml2rdf convert [OPTIONS]

Options:
  -n, --namespace <NAMESPACE>
          Namespace for RDF graph generation.

          A custom namespace to prefix RDF resources created from XML keys and values.

          [default: https://decisym.ai/xml2rdf/data]

  -x, --xml <XML>...
          Path to input XML file(s).

          Provide the path to one or more XML files that will be parsed and converted.

  -o, --output-file <OUTPUT_FILE>
          Path to output file.

          Optional: Specify the path to save the generated RDF data. If not provided, data will be written to stdout

  -h, --help
          Print help (see a summary with '-h')
```

## Using the convert library

The conversion functionality can also be called directly in Rust. The library supports writing results to a file or building an in-memory `oxrdf::Graph`.

```rust
use xml2rdf::convert::parse_xml;
use xml2rdf::writer;
use oxrdf::Graph;

// capture conversion results to file
let mut w = writer::FileWriter::to_file("output.nt".to_string()).unwrap();
parse_xml(vec!["data.xml".to_string()], &mut w, "https://decisym.ai/xml2rdf/data");

// capture conversion results to an oxrdf::Graph
let mut g = Graph::new();
let mut w = writer::GraphWriter::new(&mut g);
parse_xml(vec!["data.xml".to_string()], &mut w, "https://decisym.ai/xml2rdf/data");
```

## License
This project is licensed under the BSD 3-Clause License - see the [LICENSE](LICENSE) file for details.