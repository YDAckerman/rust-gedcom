/*! Utility functions for the gedcom crate

*/

use std::path::PathBuf;
use crate::parser::Parser;
use crate::tree::GedcomData;
use anyhow::Result;

/// Macro for displaying `Option`s in debug mode without the text wrapping.
#[macro_export]
macro_rules! fmt_optional_value {
    ($debug_struct: ident, $prop: literal, $val: expr) => {
        if let Some(value) = $val {
            $debug_struct.field($prop, value);
        } else {
            $debug_struct.field($prop, &"None");
        }
    };
}



/// Parses a Gedcom file into the `GedcomData` type
///
/// # Arguments
///
/// * 'path' - path to the gedcom file
///
/// # Errors
///
/// * see `ParserErrors`
/// 
pub fn parse(path: &str) -> Result<GedcomData> {
    let simple_ged: String = read_relative(path);
    let mut parser = Parser::new(simple_ged.chars());
    parser.parse_record()
}

/// Reads data from file to `String`
///
/// # Arguments
///
/// * 'path' - path to the gedcom file
///
fn read_relative(path: &str) -> String {
    let path_buf: PathBuf = PathBuf::from(path);
    let absolute_path: PathBuf = std::fs::canonicalize(path_buf).unwrap();
    std::fs::read_to_string(absolute_path).unwrap()
}
