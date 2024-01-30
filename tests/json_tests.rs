use gedcom::util::parse;
use gedcom::types::Name;
use gedcom::Analyzer;
use serde_json;
use serde_test::{assert_tokens, Token};

#[test]
fn serde_simple_gedcom_data() {
    let name = Name {
        value: Some("Gregor Johann /Mendel/".into()),
        given: Some("Gregor Johann".into()),
        surname: Some("Mendel".into()),
        prefix: None,
        surname_prefix: None,
        suffix: None,
    };

    assert_tokens(
        &name,
        &[
            Token::Struct {
                name: "Name",
                len: 6,
            },
            Token::Str("value"),
            Token::Some,
            Token::String("Gregor Johann /Mendel/"),
            Token::Str("given"),
            Token::Some,
            Token::String("Gregor Johann"),
            Token::Str("surname"),
            Token::Some,
            Token::String("Mendel"),
            Token::Str("prefix"),
            Token::None,
            Token::Str("surname_prefix"),
            Token::None,
            Token::Str("suffix"),
            Token::None,
            Token::StructEnd,
        ],
    );
}

#[test]
fn serde_entire_gedcom_tree() {
    
    let data = parse("./tests/fixtures/simple.ged").unwrap();

    assert_eq!(
        serde_json::to_string_pretty(&data.families).unwrap(),
        "{
  \"@FAMILY@\": {
    \"husbs\": [
      \"@FATHER@\"
    ],
    \"wives\": [
      \"@MOTHER@\"
    ],
    \"children\": [
      \"@CHILD@\"
    ],
    \"num_children\": null,
    \"events\": [
      {
        \"event\": \"Marriage\",
        \"date\": \"1 APR 1950\",
        \"place\": \"marriage place\",
        \"citations\": []
      }
    ]
  }
}"
    );

    assert_eq!(
        serde_json::to_string_pretty(&data.individuals.get("@FATHER@").unwrap())
            .unwrap(),
        "{
  \"name\": {
    \"value\": \"/Father/\",
    \"given\": null,
    \"surname\": null,
    \"prefix\": null,
    \"surname_prefix\": null,
    \"suffix\": null
  },
  \"title\": \"title\",
  \"sex\": \"Male\",
  \"fam_spouse\": [
    \"@FAMILY@\"
  ],
  \"fam_child\": {},
  \"custom_data\": [],
  \"last_updated\": null,
  \"events\": [
    {
      \"event\": \"Birth\",
      \"date\": \"1 JAN 1899\",
      \"place\": \"birth place\",
      \"citations\": []
    },
    {
      \"event\": \"Death\",
      \"date\": \"31 DEC 1990\",
      \"place\": \"death place\",
      \"citations\": []
    }
  ]
}"
    );

    // let json_data = serde_json::to_string_pretty(&data.individuals).unwrap();
    // panic!("{:?}", json_data);
}

#[test]
fn serde_name_counts() {
    let data = parse("./tests/fixtures/simple.ged").unwrap();

    if let Ok(analyzer) = Analyzer::new(&data) {
        assert_eq!(
            analyzer.count_individual_names().unwrap(),
            "{\"/Child/\":1,\"/Father/\":1,\"/Mother/\":1}"
    )
    }
}
