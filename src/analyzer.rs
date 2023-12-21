use crate::tree::GedcomData;
use anyhow::{Result, anyhow};
use std::collections::{HashMap, HashSet, BTreeSet};

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
                 to_visit: &mut BTreeSet<&'b Xref>,
                 marks: &mut HashMap<&'b Xref, Mark>,
                 sorted: &mut Vec<&'b Xref>,
                 xref: &'b Xref) -> Result<()> {

        to_visit.remove(xref);
        
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
                        visit(tree, to_visit, marks, sorted, xref_chld)?;
                    }
                }
            }
        }
        
        marks.insert(xref, Mark::Perm);
        sorted.push(xref);
        
        Ok(())
    }

    let mut to_visit: BTreeSet<&Xref> = tree.individuals.keys().collect();
    let mut marks: HashMap<&'b Xref, Mark> = HashMap::new();
    let mut sorted: Vec<&'b Xref> = Vec::new();

    while let Some(xref) = to_visit.pop_first() {
        visit(tree, &mut to_visit, &mut marks, &mut sorted, xref)?;
    }

    Ok(sorted)
}


pub fn connected_components<'b>(tree: &'b GedcomData) -> Vec<HashSet<&'b Xref>> {
    
    fn bfs<'b>(tree: &'b GedcomData,
               unvisited: &mut BTreeSet<&'b Xref>,
               xref: &'b Xref) -> HashSet<&'b Xref> {

        let mut stack = vec![xref];
        let mut visited: HashSet<&'b Xref> = HashSet::from([xref]);
        
        while let Some(current_xref) = stack.pop() {
            if let Some(indv) = tree.individuals.get(current_xref) {
                for xref_fam_sp in &indv.fam_spouse {
                    if let Some(fam_sp) = tree.families.get(xref_fam_sp) {
                        
                        for xref_chld in &fam_sp.children {
                            if unvisited.remove(xref_chld) {
                                stack.push(xref_chld);
                                visited.insert(xref_chld);
                            }
                            
                        }
                    }
                }
                for xref_fam_chld in indv.fam_child.keys() {
                    if let Some(fam_chld) = tree.families.get(xref_fam_chld) {
                        
                        for xref_parent in &fam_chld.husbs {
                            if unvisited.remove(xref_parent) {
                                stack.push(xref_parent);
                                visited.insert(xref_parent);
                            }
                        }

                        for xref_parent in &fam_chld.wives {
                            if unvisited.remove(xref_parent) {
                                stack.push(xref_parent);
                                visited.insert(xref_parent);
                            }
                        }
                    }
                }
            }
        }
        
        visited
    }

    let mut unvisited: BTreeSet<&'b Xref> = tree.individuals.keys().collect();
    let mut components = Vec::new();

    while let Some(xref) = unvisited.pop_first() {
        components.push(bfs(tree, &mut unvisited, xref));
    }

    components
    
}
