
use std::collections::HashMap;
use crate::types::{Family, Header, Individual, Media, Repository, Source, Submitter};

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

// use std::collections::HashMap;
type Xref = String;

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
/// The data structure representing all the data within a gedcom file
pub struct GedcomData {
    /// Header containing file metadata
    pub header: Header,
    /// List of submitters of the facts
    pub submitters: Vec<Submitter>,
    /// Individuals within the family tree
    pub individuals: HashMap<Xref, Individual>,
    /// The family units of the tree, representing relationships between individuals
    pub families: HashMap<Xref, Family>,
    /// A data repository where `sources` are held
    pub repositories: Vec<Repository>,
    /// Sources of facts. _ie._ book, document, census, etc.
    pub sources: Vec<Source>,
    /// A multimedia asset linked to a fact
    pub multimedia: Vec<Media>,
}

// should maybe store these by xref if available?
impl GedcomData {
    /// Adds a `Family` (a relationship between individuals) to the tree
    pub fn add_family(&mut self, xref: Option<Xref>, family: Family) {
        
        if let Some(id) = xref {
            self.families.insert(id, family);
        };
        
    }

    /// Adds an `Individual` to the tree
    pub fn add_individual(&mut self, xref:Option<Xref>, individual: Individual) {

        if let Some(id) = xref {
                self.individuals.insert(id, individual);
        };
        
    }

    /// Adds a data `Repository` to the tree
    pub fn add_repository(&mut self, repo: Repository) {
        self.repositories.push(repo);
    }

    /// Adds a `Source` to the tree
    pub fn add_source(&mut self, source: Source) {
        self.sources.push(source);
    }

    /// Adds a `Submitter` to the tree
    pub fn add_submitter(&mut self, submitter: Submitter) {
        self.submitters.push(submitter);
    }

    /// Outputs a summary of data contained in the tree to stdout
    pub fn stats(&self) {
        println!("----------------------");
        println!("| Gedcom Data Stats: |");
        println!("----------------------");
        println!("  submitters: {}", self.submitters.len());
        println!("  individuals: {}", self.individuals.len());
        println!("  families: {}", self.families.len());
        println!("  repositories: {}", self.repositories.len());
        println!("  sources: {}", self.sources.len());
        println!("  multimedia: {}", self.multimedia.len());
        println!("----------------------");
    }
}

impl GedcomData {

    fn individual_has_children(&self, xref: &Xref) -> bool {
        match self.individuals.get(xref) {
            Some(indv) => {
                indv.fam_spouse
                    .iter()
                    .any(|fam_xref|
                         self.family_has_children(fam_xref))
            },
            None => false,
        }
    }

    fn family_has_children(&self, xref: &Xref) -> bool {
        let count = match self.families.get(xref) {
            Some(fam) => {
                if let Some(count) = fam.num_children {
                    count
                }
                else {
                    0
                }
            },
            None => 0,
        };
        count > 0
    }

    #[must_use]
    /// get the terminal individuals of the tree
    pub fn get_terminal_leaves(&self) -> Vec<&Xref> {
        // lifetimes elided
        self.individuals.keys()
            .filter(|xref|
                    self.individual_has_children(xref)
            )
            .collect::<Vec<&Xref>>()

    }
    
}
