use gedcom::util::parse;
use gedcom::{GedcomData, Analyzer};
use gedcom::types::event::HasEvents;

#[test]
fn parses_basic_gedcom() {
    
    let data = parse("./tests/fixtures/simple.ged").unwrap();
    
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
fn does_topological_sort() {

    let data = parse("./tests/fixtures/simple.ged").unwrap();

    if let Ok(analyzer) = Analyzer::new(&data) {
        assert_eq!(analyzer.individuals_sorted[0], "@CHILD@");
        let father = String::from("@FATHER@");
        let mother = String::from("@MOTHER@");
        assert!(analyzer.individuals_sorted[1].eq(&father) | analyzer.individuals_sorted[1].eq(&mother));
        assert!(analyzer.individuals_sorted[2].eq(&father) | analyzer.individuals_sorted[2].eq(&mother));
    }
}

#[test]
fn finds_connected_components() {
    
    let data = parse("./tests/fixtures/simple.ged").unwrap();
    
    if let Ok(analyzer) = Analyzer::new(&data) {
        let child = String::from("@CHILD@");
        let father = String::from("@FATHER@");
        let mother = String::from("@MOTHER@");
        assert!(analyzer.components[0].contains(&child));
        assert!(analyzer.components[0].contains(&mother));
        assert!(analyzer.components[0].contains(&father));
    }
}

