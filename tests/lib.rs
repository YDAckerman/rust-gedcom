#[cfg(test)]
pub mod util {
    use std::path::PathBuf;
    use gedcom::GedcomData;
    use gedcom::parser::Parser;
    
    pub fn read_relative(path: &str) -> String {
        let path_buf: PathBuf = PathBuf::from(path);
        let absolute_path: PathBuf = std::fs::canonicalize(path_buf).unwrap();
        std::fs::read_to_string(absolute_path).unwrap()
    }

    pub fn parse(path: &str) -> GedcomData {
        let simple_ged: String = read_relative(path);
        let mut parser = Parser::new(simple_ged.chars());
        parser.parse_record().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::util::parse;
    use gedcom::GedcomData;
    use gedcom::types::event::HasEvents;
    use gedcom::analyzer::topological_sort;

    #[test]
    fn parses_basic_gedcom() {
        
        let data: GedcomData = parse("./tests/fixtures/simple.ged");
        
        assert_eq!(data.individuals.len(), 3);
        assert_eq!(data.families.len(), 1);
        assert_eq!(data.submitters.len(), 1);

        // header
        assert_eq!(data.header.encoding.unwrap().as_str(), "ASCII");
        assert_eq!(data.header.submitter_tag.unwrap().as_str(), "@SUBMITTER@");
        assert_eq!(data.header.gedcom_version.unwrap().as_str(), "5.5");

        // names
        assert_eq!(
            data.individuals.get("@FATHER@")
                .unwrap()
                .name
                .as_ref()
                .unwrap()
                .value
                .as_ref()
                .unwrap(),
            "/Father/"
        );

        // title
        assert_eq!(
            data.individuals.get("@FATHER@")
                .unwrap()
                .title
                .as_ref()
                .unwrap(),
            "title"
        );

        // no title
        assert_eq!(
            data.individuals.get("@MOTHER@")
                .unwrap()
                .title
                .is_none(),
            true
        );

        // family spouse
        assert_eq!(
            data.individuals.get("@FATHER@")
                .unwrap()
                .fam_spouse
                .contains("@FAMILY@"),
            true
        );

        // family child
        assert_eq!(
            data.individuals.get("@CHILD@")
                .unwrap()
                .fam_child
                .contains_key("@FAMILY@"),
            true
        );
        
        // addresses
        assert_eq!(
            data.submitters[0]
                .address
                .as_ref()
                .unwrap()
                .value
                .as_ref()
                .unwrap(),
            "Submitters address\naddress continued here"
        );

        // submitter comments
        assert_eq!(
            data.submitters[0]
                .comments
                .as_ref()
                .unwrap(),
            "message line 1\nmessage line 2\nmessage line 3"
        );

        // events
        let events = data.families.get("@FAMILY@").unwrap().events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event.to_string(), "Marriage");
        assert_eq!(events[0].date.as_ref().unwrap(), "1 APR 1950");
    }

    #[test]
    fn performs_topological_sort() {

        let data: GedcomData = parse("./tests/fixtures/simple.ged");

        if let Ok(sorted) = topological_sort(&data) {
            assert_eq!(sorted[0], "@CHILD@");
            assert_eq!(sorted[1], "@MOTHER@");
            assert_eq!(sorted[2], "@FATHER@");
        }
    }
    
}
