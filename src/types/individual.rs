use crate::types::{event::HasEvents, CustomData, Event};
use anyhow::Result;
use anyhow::anyhow;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

type Xref = String;

/// A Person within the family tree
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Individual {
    pub xref: Option<Xref>, // this MUST exist, so remove Option
    pub name: Option<Name>,
    pub title: Option<String>,
    pub sex: Gender,
    pub families: Vec<FamilyLink>,
    pub custom_data: Vec<CustomData>,
    pub last_updated: Option<String>,
    events: Vec<Event>,
}

impl Individual {
    #[must_use]
    pub fn new(xref: Option<Xref>) -> Individual {
        Individual {
            xref,
            name: None,
            title: None,
            sex: Gender::Unknown,
            events: Vec::new(),
            families: Vec::new(),
            custom_data: Vec::new(),
            last_updated: None,
        }
    }

    pub fn add_family(&mut self, link: FamilyLink) {
        let mut do_add = true;
        let xref = &link.0;
        for FamilyLink(family, _, _) in &self.families {
            if family.as_str() == xref.as_str() {
                do_add = false;
            }
        }
        if do_add {
            self.families.push(link);
        }
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
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum Gender {
    Male,
    Female,
    // come at me LDS, i support "N" as a gender value
    Nonbinary,
    Unknown,
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
enum FamilyLinkType {
    Spouse,
    Child,
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
enum Pedigree {
    Adopted,
    Birth,
    Foster,
    Sealing,
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct FamilyLink(Xref, FamilyLinkType, Option<Pedigree>);

impl FamilyLink {
    pub fn new(xref: Xref, tag: &str) -> Result<FamilyLink> {
        let link_type = match tag {
            "FAMC" => FamilyLinkType::Child,
            "FAMS" => FamilyLinkType::Spouse,
            _ => return Err(anyhow!("Unrecognized family type tag: {}",
                                            tag)),
        };
        Ok(FamilyLink(xref, link_type, None))
    }

    pub fn set_pedigree(&mut self, pedigree_text: &str) -> anyhow::Result<()> {
        self.2 = match pedigree_text.to_lowercase().as_str() {
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

#[derive(Debug, Default, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Name {
    pub value: Option<String>,
    pub given: Option<String>,
    pub surname: Option<String>,
    pub prefix: Option<String>,
    pub surname_prefix: Option<String>,
    pub suffix: Option<String>,
}
