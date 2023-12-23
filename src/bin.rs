use gedcom::util::parse;
use gedcom::GedcomData;
use std::env;
use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => usage("Missing filename."),
        s if s > 2 => usage(&format!("Found more args than expected: {:?}", &args[1..])),
        _ => (),
    };

    let filename = &args[1];

    if filename == "--help" || filename == "-h" {
        usage("");
    }

    let data = parse(filename)?;
    println!("Parsing complete!");
    data.stats();

    Ok(())
}

fn usage(msg: &str) {
    if !msg.is_empty() {
        println!("{}", msg);
    }
    println!("Usage: parse_gedcom ./path/to/gedcom.ged");
    std::process::exit(0x0100);
}

// fn exit_with_error(msg: &str) {
//     println!("Error! {}", msg);
//     std::process::exit(0x1);
// }
