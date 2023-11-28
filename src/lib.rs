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
mod util;

pub mod parser;
pub use parser::ParseError;
    
pub mod tokenizer;
pub mod types;

mod tree;
pub use tree::GedcomData;

use anyhow::Result;

#[must_use]
/// Helper function for converting GEDCOM file content stream to parsed data.
pub fn parse(content: std::str::Chars) -> Result<GedcomData> {
    let mut p = parser::Parser::new(content);
    let tree = p.parse_record()?;

    Ok(tree)
}
