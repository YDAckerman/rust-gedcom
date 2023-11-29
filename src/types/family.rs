use crate::types::{event::HasEvents, Event};
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

type Xref = String;

/// Family fact, representing a relationship between `Individual`s
///
/// This data representation understands that HUSB & WIFE are just poorly-named
/// pointers to individuals. no gender "validating" is done on parse.
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Family {
    pub husbs: Vec<Xref>, // mapped from HUSB
    pub wives: Vec<Xref>, // mapped from WIFE
    pub children: Vec<Xref>,
    pub num_children: Option<u8>,
    events: Vec<Event>, // why is this private?
}

impl Family {

    pub fn add_husb(&mut self, xref: Xref) {
        self.husbs.push(xref);
    }

    pub fn add_wife(&mut self, xref: Xref) {
        self.wives.push(xref);
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
