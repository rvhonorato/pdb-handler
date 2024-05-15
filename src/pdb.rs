use std::collections::HashMap;

use crate::constants::{AMINOACIDS, DNA};

pub fn identify_molecular_types(structure: pdbtbx::PDB) -> HashMap<String, Vec<String>> {
    let mut mol_types = HashMap::new();
    for chain in structure.chains() {
        let mut chain_mol_types = Vec::new();
        for res in chain.residues() {
            if AMINOACIDS.contains(&res.name().unwrap().to_uppercase().as_str()) {
                chain_mol_types.push("protein".to_string());
            } else if DNA.contains(&res.name().unwrap().to_uppercase().as_str()) {
                chain_mol_types.push("dna".to_string());
            } else {
                chain_mol_types.push("other".to_string());
            }
        }
        // Remove duplicates
        chain_mol_types.sort();
        chain_mol_types.dedup();

        mol_types.insert(chain.id().to_string(), chain_mol_types);
    }
    mol_types
}

pub fn identify_chains(structure: pdbtbx::PDB) -> Vec<String> {
    let mut chains: Vec<String> = Vec::new();
    for chain in structure.chains() {
        chains.push(chain.id().to_string());
    }
    chains
}

pub fn identify_residue_numbers(structure: pdbtbx::PDB) -> HashMap<String, Vec<String>> {
    let mut res_map = HashMap::new();

    for chain in structure.chains() {
        let mut chain_residues = Vec::new();
        for res in chain.residues() {
            chain_residues.push(res.serial_number().to_string());
        }
        // Remove duplicates
        chain_residues.sort();
        chain_residues.dedup();

        res_map.insert(chain.id().to_string(), chain_residues);
    }

    res_map
}

pub fn identify_unknowns(structure: pdbtbx::PDB) -> HashMap<String, Vec<String>> {
    let mut res_map = HashMap::new();

    let mut known_residues = Vec::new();
    known_residues.extend(AMINOACIDS);
    known_residues.extend(DNA);

    for chain in structure.chains() {
        let mut chain_residues = Vec::new();
        for res in chain.residues() {
            if !known_residues.contains(&res.name().unwrap().to_uppercase().as_str()) {
                chain_residues.push(res.name().unwrap().to_string());
            }
        }
        // Remove duplicates
        chain_residues.sort();
        chain_residues.dedup();

        res_map.insert(chain.id().to_string(), chain_residues);
    }

    res_map
}

pub fn chains_in_contact(structure: pdbtbx::PDB) -> Vec<(String, String)> {
    // Define an array of tuples to store the contacts
    let mut contacts: Vec<(String, String)> = Vec::new();

    // Calculate all possible combinations of chains
    for chain_x in structure.chains() {
        for chain_y in structure.chains() {
            // Ignore if they are the same chain
            if chain_x.id() == chain_y.id() {
                continue;
            }

            // Check if (chain_x, chain_y) are already in the contacts array
            let mut already_in_contacts = false;
            for contact in &contacts {
                if (contact.0 == *chain_x.id() && contact.1 == *chain_y.id())
                    || (contact.0 == *chain_y.id() && contact.1 == *chain_x.id())
                {
                    already_in_contacts = true;
                    break;
                }
            }

            if already_in_contacts {
                continue;
            }

            for res_x in chain_x.residues() {
                for res_y in chain_y.residues() {
                    for atom_i in res_x.atoms() {
                        for atom_j in res_y.atoms() {
                            let x_i = atom_i.x();
                            let y_i = atom_i.y();
                            let z_i = atom_i.z();

                            let x_j = atom_j.x();
                            let y_j = atom_j.y();
                            let z_j = atom_j.z();

                            // Calculate the euclidean distance between the two atoms
                            let dist =
                                ((x_i - x_j).powi(2) + (y_i - y_j).powi(2) + (z_i - z_j).powi(2))
                                    .sqrt();

                            if dist <= 5.0 {
                                contacts.push((chain_x.id().to_string(), chain_y.id().to_string()));
                            }
                        }
                    }
                }
            }
        }
    }

    // Remove duplicates
    contacts.sort();
    contacts.dedup();

    contacts
}

#[cfg(test)]
mod tests {
    use crate::pdb::*;

    #[test]
    fn test_identify_chains() {
        let (pdb, _) =
            pdbtbx::open_pdb("test-data/1ppe.pdb", pdbtbx::StrictnessLevel::Loose).unwrap();
        let chains = identify_chains(pdb);
        assert_eq!(chains, vec!["E", "I"]);
    }

    #[test]
    fn test_chains_in_contact() {
        let (pdb, _) =
            pdbtbx::open_pdb("test-data/8kgk.pdb", pdbtbx::StrictnessLevel::Loose).unwrap();

        let result = chains_in_contact(pdb);

        assert_eq!(result.len(), 6);

        // Expected vector is
        let expected_result = [
            ("A", "B"),
            ("A", "C"),
            ("B", "C"),
            ("B", "E"),
            ("C", "D"),
            ("C", "E"),
        ];

        // Assert that the result contains all the expected pairs
        for pair in expected_result.iter() {
            // Change to a string
            let pair = (pair.0.to_string(), pair.1.to_string());
            assert!(result.contains(&pair));
        }
    }
}
