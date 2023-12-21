use crate::types::{event::HasEvents, CustomData, Event};
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashMap;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

type Xref = String;

/// A Person within the family tree
#[derive(Debug, Serialize, Deserialize)]
pub struct Individual {
    pub name: Option<Name>,
    pub title: Option<String>,
    pub sex: Gender,
    pub fam_spouse: HashSet<Xref>,
    pub fam_child: HashMap<Xref, Option<Pedigree>>,
    pub custom_data: Vec<CustomData>,
    pub last_updated: Option<String>,
    events: Vec<Event>,
}

impl Individual {

    pub fn add_family(&mut self, xref: Xref, link: FamilyLink) {

        match link.0 {
            FamilyLinkType::Child => {
                self.fam_child.insert(xref, link.1);
            },
            FamilyLinkType::Spouse => {
                self.fam_spouse.insert(xref);
            },
        };

    }

    pub fn add_custom_data(&mut self, data: CustomData) {
        self.custom_data.push(data);
    }
    
}

impl HasEvents for Individual {
    fn add_event(&mut self, event: Event) -> Result<()>{
        self.events.push(event);
        Ok(())
    }
    fn events(&self) -> Vec<Event> {
        self.events.clone()
    }
}

/// Gender of an `Individual`
#[derive(Debug, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    // come at me LDS, i support "N" as a gender value
    Nonbinary,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
enum FamilyLinkType {
    Spouse,
    Child,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Pedigree {
    Adopted,
    Birth,
    Foster,
    Sealing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FamilyLink(FamilyLinkType, Option<Pedigree>);

impl FamilyLink {
    pub fn new(tag: &str) -> Result<FamilyLink> {
        let link_type = match tag {
            "FAMC" => FamilyLinkType::Child,
            "FAMS" => FamilyLinkType::Spouse,
            _ => return Err(anyhow!("Unrecognized family type tag: {}",
                                            tag)),
        };
        Ok(FamilyLink(link_type, None))
    }

    pub fn set_pedigree(&mut self, pedigree_text: &str) -> Result<()> {
        self.1 = match pedigree_text.to_lowercase().as_str() {
            "adopted" => Some(Pedigree::Adopted),
            "birth" => Some(Pedigree::Birth),
            "foster" => Some(Pedigree::Foster),
            "sealing" => Some(Pedigree::Sealing),
            _ => return Err(anyhow!("Unrecognized family link pedigree: {}",
                                    pedigree_text)),
        };
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Name {
    pub value: Option<String>,
    pub given: Option<String>,
    pub surname: Option<String>,
    pub prefix: Option<String>,
    pub surname_prefix: Option<String>,
    pub suffix: Option<String>,
}

impl Default for Individual {

    fn default() -> Self {
        Individual {
            name: None,
            title: None,
            sex: Gender::Unknown,
            events: Vec::new(),
            fam_spouse: HashSet::new(),
            fam_child: HashMap::new(),
            custom_data: Vec::new(),
            last_updated: None,
        }
    }
    
}
