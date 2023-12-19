/*! A parser for GEDCOM files

```rust
use gedcom::parser::Parser;

// the parser takes the gedcom file contents as a chars iterator
let gedcom_source = std::fs::read_to_string("./tests/fixtures/sample.ged").unwrap();

let mut parser = Parser::new(gedcom_source.chars());
let gedcom_data = parser.parse_record().unwrap();

// output some stats on the gedcom contents
gedcom_data.stats();
```

This crate contains an optional `"json"` feature that implements serialization & deserialization to json with [`serde`](https://serde.rs).
*/

#![deny(clippy::pedantic)]
#![warn(missing_docs)]

#[macro_use]
pub mod util;
pub use util::parse;

pub mod parser;
pub use parser::ParseError;
    
pub mod tokenizer;
pub mod types;

mod tree;
pub use tree::GedcomData;

pub mod analyzer;
pub use analyzer::topological_sort;
pub use analyzer::connected_components;


