/*! A parser for GEDCOM files

```rust
use gedcom::util::parse;
use gedcom::GedcomData;

// the parser takes the gedcom file contents as a chars iterator
let gedcom_data = parse("./tests/fixtures/sample.ged").unwrap();

// output some stats on the gedcom contents
gedcom_data.stats();
```

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
pub use analyzer::Analyzer;

