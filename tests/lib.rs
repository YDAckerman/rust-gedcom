#[cfg(test)]
pub mod util {
    use std::path::PathBuf;
    pub fn read_relative(path: &str) -> String {
        let path_buf: PathBuf = PathBuf::from(path);
        let absolute_path: PathBuf = std::fs::canonicalize(path_buf).unwrap();
        std::fs::read_to_string(absolute_path).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::util::read_relative;
    use gedcom::parser::Parser;
    use gedcom::types::event::HasEvents;

    #[test]
    fn parses_basic_gedcom() {
        let simple_ged: String = read_relative("./tests/fixtures/simple.ged");
        assert!(simple_ged.len() > 0);

        let mut parser = Parser::new(simple_ged.chars());
        let data = parser.parse_record();
        assert_eq!(data.individuals.len(), 3);
        assert_eq!(data.families.len(), 1);
        assert_eq!(data.submitters.len(), 1);

        // header
        assert_eq!(data.header.encoding.unwrap().as_str(), "ASCII");
        assert_eq!(data.header.submitter_tag.unwrap().as_str(), "@SUBMITTER@");
        assert_eq!(data.header.gedcom_version.unwrap().as_str(), "5.5");

        // names
        assert_eq!(
            data.individuals[0]
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
            data.individuals[0]
                .title
                .as_ref()
                .unwrap(),
            "title"
        );

        assert_eq!(
            data.individuals[1].title.is_none(),
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
        let events = data.families[0].events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event.to_string(), "Marriage");
        assert_eq!(events[0].date.as_ref().unwrap(), "1 APR 1950");
    }
}
