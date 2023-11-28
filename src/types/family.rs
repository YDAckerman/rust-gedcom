use crate::types::{event::HasEvents, Event};
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

type Xref = String;

/// Family fact, representing a relationship between `Individual`s
///
/// This data representation understands that HUSB & WIFE are just poorly-named
/// pointers to individuals. no gender "validating" is done on parse.
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Family {
    pub xref: Option<Xref>,
    pub individual1: Option<Xref>, // mapped from HUSB
    pub individual2: Option<Xref>, // mapped from WIFE
    pub children: Vec<Xref>,
    pub num_children: Option<u8>,
    events: Vec<Event>, // why is this private?
}

impl Family {
    #[must_use]
    pub fn new(xref: Option<Xref>) -> Family {
        Family {
            xref,
            individual1: None,
            individual2: None,
            children: Vec::new(),
            num_children: None,
            events: Vec::new(),
        }
    }

    pub fn set_individual1(&mut self, xref: Xref) -> Result<()> {
        match self.individual1 {
            Some(_) => {
                return Err(anyhow!("Individual1 for family {} already exists",
                                   self.xref.as_ref().unwrap()))
            },
            None => self.individual1 = Some(xref),
        };
        Ok(())
    }

    pub fn set_individual2(&mut self, xref: Xref) -> Result<()>{
        match self.individual2 {
            Some(_) => {
                return Err(anyhow!("Individual2 for family {} already exists",
                                   self.xref.as_ref().unwrap()))
            },
            None => self.individual2 = Some(xref),
        };
        Ok(())
    }

    pub fn add_child(&mut self, xref: Xref) {
        self.children.push(xref);
    }
}

impl HasEvents for Family {
    fn add_event(&mut self, event: Event) -> Result<()> {
        let event_type = &event.event;
        for e in &self.events {
            if &e.event == event_type {
                return Err(anyhow!("Family already has a {:?} event", e.event))
            }
        }
        self.events.push(event);
        Ok(())
    }
    fn events(&self) -> Vec<Event> {
        self.events.clone()
    }
}
