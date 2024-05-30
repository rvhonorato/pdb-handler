use crate::constants::{AMINOACIDS, DNA};
use serde::{Deserialize, Serialize};

use std::collections::{HashMap, HashSet};

mod constants;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MolecularType {
    Protein,
    Dna,
    Other,
}

impl From<MolecularType> for String {
    fn from(val: MolecularType) -> Self {
        match val {
            MolecularType::Protein => "protein".to_string(),
            MolecularType::Dna => "dna".to_string(),
            MolecularType::Other => "other".to_string(),
        }
    }
}

pub fn identify_molecular_types(structure: &pdbtbx::PDB) -> HashMap<String, Vec<MolecularType>> {
    let mut mol_types = HashMap::new();

    for chain in structure.chains() {
        let chain_id = chain.id().to_string();
        let chain_mol_types = chain.residues().map(|res| {
            let res_name = res.name().unwrap().to_uppercase();
            if AMINOACIDS.contains(&res_name.as_str()) {
                MolecularType::Protein
            } else if DNA.contains(&res_name.as_str()) {
                MolecularType::Dna
            } else {
                MolecularType::Other
            }
        });

        let unique_mol_types = chain_mol_types.into_iter().collect();

        mol_types.insert(chain_id, unique_mol_types);
    }

    mol_types
}

pub fn identify_chains(structure: &pdbtbx::PDB) -> Vec<String> {
    structure
        .chains()
        .map(|chain| chain.id().to_string())
        .collect()
}

pub fn identify_residue_numbers(structure: &pdbtbx::PDB) -> HashMap<String, Vec<String>> {
    structure
        .chains()
        .map(|chain| {
            let resnumbers = chain
                .residues()
                .map(|res| res.serial_number().to_string())
                .collect::<Vec<_>>()
                .into_iter()
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            (chain.id().to_string(), resnumbers)
        })
        .collect()
}

pub fn identify_unknowns(structure: &pdbtbx::PDB) -> HashMap<String, Vec<String>> {
    let mut res_map = HashMap::new();

    let known_residues: HashSet<_> = AMINOACIDS
        .iter()
        .chain(DNA.iter())
        .map(|s| s.to_uppercase())
        .collect();

    for chain in structure.chains() {
        let chain_residues: Vec<_> = chain
            .residues()
            .filter(|res| !known_residues.contains(&res.name().unwrap().to_uppercase()))
            .map(|res| res.name().unwrap().to_string())
            .collect();

        let mut chain_residues = chain_residues;

        chain_residues.sort();
        chain_residues.dedup();

        res_map.insert(chain.id().to_string(), chain_residues);
    }

    res_map
}

pub fn chains_in_contact(structure: &pdbtbx::PDB) -> Vec<(String, String)> {
    let mut contacts: HashSet<Vec<String>> = HashSet::new();

    for (chain_x, chain_y) in structure
        .chains()
        .flat_map(|cx| structure.chains().map(move |cy| (cx, cy)))
    {
        if chain_x.id() == chain_y.id() {
            continue;
        }

        let mut in_contacts = false;
        for contact in &contacts {
            if contact.contains(&chain_x.id().to_string())
                && contact.contains(&chain_y.id().to_string())
            {
                in_contacts = true;
                break;
            }
        }

        if in_contacts {
            continue;
        }

        for res_x in chain_x.residues() {
            for res_y in chain_y.residues() {
                for atom_i in res_x.atoms() {
                    for atom_j in res_y.atoms() {
                        let dist = atom_i.distance(atom_j);
                        if dist <= 5.0 {
                            contacts
                                .insert(vec![chain_x.id().to_string(), chain_y.id().to_string()]);
                        }
                    }
                }
            }
        }
    }

    contacts
        .into_iter()
        .map(|pair| (pair[0].clone(), pair[1].clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    // use pdbtbx::{Atom, Chain, Residue, PDB};
    use std::collections::HashMap;

    #[test]
    fn test_identify_unknowns() {
        // Load the structure from the test_data folder
        let (structure, _) =
            pdbtbx::open_pdb("test_data/prot_ligand.pdb", pdbtbx::StrictnessLevel::Loose).unwrap();

        let unknowns = identify_unknowns(&structure);

        let mut expected = HashMap::new();
        expected.insert("A".to_string(), vec!["I09".to_string()]);

        assert_eq!(unknowns, expected);
    }

    #[test]
    fn test_identify_chains() {
        // Load the structure from the test_data folder
        let (structure, _) =
            pdbtbx::open_pdb("test_data/chains.pdb", pdbtbx::StrictnessLevel::Loose).unwrap();

        let chains = identify_chains(&structure);

        assert_eq!(
            chains,
            vec!["A".to_string(), "B".to_string(), "C".to_string()]
        );
    }
}
