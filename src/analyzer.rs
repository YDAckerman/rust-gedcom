use crate::tree::GedcomData;
use anyhow::{Result, anyhow};
use std::collections::HashMap;

type Xref = String;

// /// return true if the individual has children
// fn individual_has_children(tree: &GedcomData, xref: &Xref) -> bool {
//         match tree.individuals.get(xref) {
//             Some(indv) => {
//                 indv.fam_spouse
//                     .iter()
//                     .any(|fam_xref|
//                          family_has_children(tree, fam_xref))
//             },
//             None => false,
//         }
//     }

// /// return true if the family has children
// fn family_has_children(tree: &GedcomData, xref: &Xref) -> bool {
//     let count = match tree.families.get(xref) {
//         Some(fam) => {
//             if let Some(count) = fam.num_children {
//                 count
//             }
//             else {
//                 0
//             }
//         },
//         None => 0,
//     };
//     count > 0
// }

// /// get the terminal individuals of the tree
// pub fn get_terminal_leaves(tree: &GedcomData) -> Vec<&Xref> {
//     // lifetimes elided
//     tree.individuals.keys()
//         .filter(|xref|
//                 individual_has_children(tree, xref)
//         )
//         .collect::<Vec<&Xref>>()

// }

/// sort the individuals in topological order
pub fn topological_sort<'b>(tree: &'b GedcomData) -> Result<Vec<&'b Xref>>{

    enum Mark {
        Perm,
        Temp,
    }

    fn visit<'b>(tree: &'b GedcomData,
                 marks: &mut HashMap<&'b String, Mark>,
                 sorted: & mut Vec<&'b Xref>,
                 xref: &'b Xref) -> Result<()> {
        
        if let Some(mark) = marks.get(xref) {
            match mark {
                Mark::Perm => return Ok(()),
                Mark::Temp => return Err(anyhow!("tree has a cycle")),
            }
        }

        marks.insert(xref, Mark::Temp);

        if let Some(indv) = tree.individuals.get(xref) {
            for xref_fam in &indv.fam_spouse {
                if let Some(fam) = tree.families.get(xref_fam) {
                    for xref_chld in &fam.children {
                        visit(tree, marks, sorted, xref_chld)?;
                    }
                }
            }
        }
        
        marks.insert(xref, Mark::Perm);
        sorted.push(xref);
        
        Ok(())
    }

    let mut marks: HashMap<&'b Xref, Mark> = HashMap::new();
    let mut sorted: Vec<&'b Xref> = Vec::new();
    
    for xref in tree.individuals.keys() {
        visit(tree, &mut marks, &mut sorted, xref)?;
    }

    Ok(sorted)
}

