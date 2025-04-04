use crate::constants::{AMINOACIDS, DNA};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};

use std::collections::{HashMap, HashSet};

mod constants;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

/// Identifies molecular types in the given PDB structure.
///
/// This function analyzes the chains and residues in a PDB structure to categorize each residue
/// into molecular types such as Protein, DNA, or Other. It returns a `HashMap` where the keys
/// are chain IDs and the values are vectors of unique `MolecularType`s present in each chain.
///
/// # Arguments
///
/// * `structure` - A reference to a `pdbtbx::PDB` structure representing the PDB file to be analyzed.
///
/// # Returns
///
/// A `HashMap<String, Vec<MolecularType>>` where each key is a chain ID and each value is a vector
/// of unique `MolecularType`s found in that chain.
///
/// # Example
///
/// ```rust
/// use pdbtbx::PDB;
/// use pdb_handler::{identify_molecular_types, MolecularType};
///
/// let (mut pdb, _errors) = pdbtbx::open("example-pdbs/1crn.pdb").unwrap();
/// let mol_types = identify_molecular_types(&pdb);
///
/// for (chain_id, types) in mol_types {
///     println!("Chain {}: {:?}", chain_id, types);
/// }
/// ```
///
/// # Panics
///
/// This function will panic if the residue name cannot be retrieved (`res.name().unwrap()`).
///
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

        let mut unique_mol_types = HashSet::new();
        for mol_type in chain_mol_types {
            unique_mol_types.insert(mol_type);
        }

        let mut types = unique_mol_types.into_iter().collect::<Vec<MolecularType>>();
        types.sort();

        mol_types.insert(chain_id, types);
    }

    mol_types
}

/// Identifies all chain IDs in the given PDB structure.
///
/// This function iterates over all chains in a PDB structure and collects their IDs into a vector of strings.
///
/// # Arguments
///
/// * `structure` - A reference to a `pdbtbx::PDB` structure representing the PDB file to be analyzed.
///
/// # Returns
///
/// A `Vec<String>` containing the IDs of all chains present in the PDB structure.
///
/// # Example
///
/// ```rust
/// use pdbtbx::PDB;
/// use pdb_handler::identify_chains;
///
/// let (mut pdb, _errors) = pdbtbx::open("example-pdbs/1crn.pdb").unwrap();
/// let chains = identify_chains(&pdb);
///
/// for chain_id in chains {
///     println!("Chain ID: {}", chain_id);
/// }
/// ```
pub fn identify_chains(structure: &pdbtbx::PDB) -> Vec<String> {
    structure
        .chains()
        .map(|chain| chain.id().to_string())
        .collect()
}

/// Identifies residue numbers in each chain of the given PDB structure.
///
/// This function iterates over all chains in a PDB structure, collects the residue numbers
/// within each chain, and returns them in a `HashMap`. The keys in the `HashMap` are chain IDs,
/// and the values are vectors of unique residue numbers represented as strings.
///
/// # Arguments
///
/// * `structure` - A reference to a `pdbtbx::PDB` structure representing the PDB file to be analyzed.
///
/// # Returns
///
/// A `HashMap<String, Vec<String>>` where each key is a chain ID and each value is a vector of unique
/// residue numbers found in that chain.
///
/// # Example
///
/// ```rust
/// use pdbtbx::PDB;
/// use pdb_handler::identify_residue_numbers;
///
/// let (mut pdb, _errors) = pdbtbx::open("example-pdbs/1crn.pdb").unwrap();
/// let residue_numbers = identify_residue_numbers(&pdb);
///
/// for (chain_id, numbers) in residue_numbers {
///     println!("Chain {}: {:?}", chain_id, numbers);
/// }
/// ```
///
/// # Panics
///
/// This function will panic if the residue serial number cannot be retrieved.
pub fn identify_residue_numbers(structure: &pdbtbx::PDB) -> HashMap<String, Vec<String>> {
    structure
        .chains()
        .map(|chain| {
            let resnumbers: Vec<String> = chain
                .residues()
                .map(|res| res.serial_number().to_string())
                .collect::<Vec<_>>()
                .into_iter()
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            // Sort the residue numbers
            let mut resnumbers = resnumbers.into_iter().collect::<Vec<_>>();
            resnumbers.sort();
            (chain.id().to_string(), resnumbers)
        })
        .collect()
}

/// Identifies unknown residues in each chain of the given PDB structure.
///
/// This function iterates over all chains in a PDB structure, filters out known residues (amino acids and DNA),
/// and collects the names of unknown residues. It returns a `HashMap` where the keys are chain IDs and the
/// values are vectors of unique unknown residue names.
///
/// # Arguments
///
/// * `structure` - A reference to a `pdbtbx::PDB` structure representing the PDB file to be analyzed.
///
/// # Returns
///
/// A `HashMap<String, Vec<String>>` where each key is a chain ID and each value is a vector of unique
/// unknown residue names found in that chain.
///
/// # Example
///
/// ```rust
/// use pdbtbx::PDB;
/// use pdb_handler::identify_unknowns;
///
/// let (mut pdb, _errors) = pdbtbx::open("example-pdbs/1crn.pdb").unwrap();
/// let unknown_residues = identify_unknowns(&pdb);
///
/// for (chain_id, residues) in unknown_residues {
///    println!("Chain {}: {:?}", chain_id, residues);
/// }
/// ```
///
/// # Panics
///
/// This function will panic if the residue name cannot be retrieved.
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

/// Identifies pairs of chains that are in close contact within the given PDB structure.
///
/// This function analyzes inter-chain interactions by checking for atoms from different chains
/// that are within a specified distance threshold (5.0 Å). It returns a list of unique chain pairs
/// where at least one pair of atoms from each chain is within the contact distance.
///
/// # Arguments
///
/// * `structure` - A reference to a `pdbtbx::PDB` structure representing the PDB file to be analyzed.
///
/// # Returns
///
/// A `Vec<(String, String)>` where each tuple represents a pair of chain IDs that are in contact.
/// The pairs are unordered and unique (e.g., if (A, B) is present, (B, A) will not be included).
///
/// # Example
///
/// ```rust
/// use pdbtbx::PDB;
/// use pdb_handler::chains_in_contact;
///
/// let (mut pdb, _errors) = pdbtbx::open("example-pdbs/1crn.pdb").unwrap();
/// let contacting_chains = chains_in_contact(&pdb);
///
/// for (chain_a, chain_b) in contacting_chains {
///     println!("Chains {} and {} are in contact", chain_a, chain_b);
/// }
/// ```
///
/// # Notes
///
/// - The contact distance threshold is fixed at 5.0 Å.
/// - Self-contacts (within the same chain) are ignored.
/// - The function uses a HashSet internally to ensure unique pairings.
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

/// Removes lines starting with "REMARK" from a PDB file and returns the filtered content as a BufReader.
///
/// This function reads a Protein Data Bank (PDB) file, filters out all lines that start with the keyword "REMARK",
/// and returns the remaining content as a `BufReader` over an in-memory buffer. This allows for further processing
/// of the filtered content without needing to write it to a temporary file.
///
/// # Arguments
///
/// * `pdb_f` - A string slice that holds the path to the input PDB file.
///
/// # Returns
///
/// * `BufReader<Cursor<Vec<u8>>>` - A `BufReader` containing the filtered content.
///
/// # Panics
///
/// This function will panic if the input file cannot be opened or read.
///
/// # Examples
///
/// ```
/// use pdb_handler::remove_remark;
/// use std::io::BufRead;
/// let reader = remove_remark("example-pdbs/1crn.pdb");
/// for line in reader.lines() {
///     println!("{:?}", line.unwrap());
/// }
/// ```
pub fn remove_remark(pdb_f: &str) -> BufReader<Cursor<Vec<u8>>> {
    // Open the input file
    let input_file = File::open(pdb_f).unwrap();
    let reader = BufReader::new(input_file);

    // Collect filtered lines into a vector
    let filtered_content: Vec<u8> = reader
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            if !line.starts_with("REMARK") {
                Some(line + "\n")
            } else {
                None
            }
        })
        .collect::<String>()
        .into_bytes();

    // Create a BufReader over an in-memory buffer
    BufReader::new(Cursor::new(filtered_content))
}

/// Reads a text file specified by `pdb_f`, pads each line that starts with `ATOM` to 80 characters
/// with spaces, and returns a buffered reader over an in-memory buffer
/// containing the padded content.
///
/// # Arguments
///
/// * `pdb_f` - A string slice that holds the path to the input text file.
///
/// # Returns
///
/// A `BufReader` wrapped around a `Cursor<Vec<u8>>>`, where each line from
/// the input file is padded to 80 characters with spaces and newline character.
///
/// # Panics
///
/// This function panics if it encounters any I/O errors while reading or
/// processing the file.
///
/// # Examples
///
/// ```rust
/// use pdb_handler::pad_lines;
/// use std::io::Read;
/// use std::io::BufReader;
///
/// let mut padded_reader = pad_lines("example-pdbs/dna.pdb");
/// let mut buffer = String::new();
/// padded_reader.read_to_string(&mut buffer).unwrap();
/// println!("Padded content:\n{}", buffer);
/// ```
///
/// This example reads lines from "dna.pdb", pads each line that starts with `ATOM` with spaces
/// to reach 80 characters, and then prints out the padded content.
pub fn pad_lines(pdb_f: &str) -> BufReader<Cursor<Vec<u8>>> {
    // Open the input file
    let input_file = File::open(pdb_f).unwrap();
    let reader = BufReader::new(input_file);

    // Collect filtered lines into a vector
    let filtered_content: Vec<u8> = reader
        .lines()
        .flat_map(|line| {
            let line = line.unwrap();
            let mut processed_line = if line.starts_with("ATOM") {
                let mut padded_line = line.to_string();
                if line.len() <= 80 {
                    padded_line.push_str(" ".repeat(80 - line.len()).as_str());
                    padded_line
                } else {
                    line[..80].to_string()
                }
            } else {
                line
            };
            processed_line.push('\n'); // Append newline
            processed_line.into_bytes()
        })
        .collect();

    // Create a BufReader over an in-memory buffer
    BufReader::new(Cursor::new(filtered_content))
}

#[cfg(test)]
mod tests {

    use pdbtbx::ReadOptions;

    use super::*;
    // use pdbtbx::{Atom, Chain, Residue, PDB};
    use std::collections::HashMap;

    #[test]
    fn test_identify_molecular_types() {
        // Load the structure from the test_data folder
        let (structure, _) = ReadOptions::default()
            .set_format(pdbtbx::Format::Pdb)
            .read("test_data/prot_ligand.pdb")
            .unwrap();

        let mol_types = identify_molecular_types(&structure);

        let mut expected = HashMap::new();
        expected.insert(
            "A".to_string(),
            vec![MolecularType::Protein, MolecularType::Other],
        );

        assert_eq!(mol_types, expected);
    }

    #[test]
    fn test_identify_chains() {
        // Load the structure from the test_data folder
        let (structure, _) = ReadOptions::default()
            .set_format(pdbtbx::Format::Pdb)
            .read("test_data/chains.pdb")
            .unwrap();

        let chains = identify_chains(&structure);

        assert_eq!(
            chains,
            vec!["A".to_string(), "B".to_string(), "C".to_string()]
        );
    }

    #[test]
    fn test_identify_residue_numbers() {
        // Load the structure from the test_data folder
        let (structure, _) = ReadOptions::default()
            .set_format(pdbtbx::Format::Pdb)
            .read("test_data/prot_ligand.pdb")
            .unwrap();

        let residue_numbers = identify_residue_numbers(&structure);

        let mut expected = HashMap::new();
        expected.insert("A".to_string(), vec!["104".to_string(), "201".to_string()]);

        assert_eq!(residue_numbers, expected);
    }

    #[test]
    fn test_identify_unknowns() {
        // Load the structure from the test_data folder
        let (structure, _) = ReadOptions::default()
            .set_format(pdbtbx::Format::Pdb)
            .read("test_data/prot_ligand.pdb")
            .unwrap();

        let unknowns = identify_unknowns(&structure);

        let mut expected = HashMap::new();
        expected.insert("A".to_string(), vec!["I09".to_string()]);

        assert_eq!(unknowns, expected);
    }

    #[test]
    fn test_chains_in_contact() {
        // Load the structure from the test_data folder
        let (structure, _) = ReadOptions::default()
            .set_format(pdbtbx::Format::Pdb)
            .read("test_data/chains_in_contact.pdb")
            .unwrap();

        let contacts = chains_in_contact(&structure);

        let expected = vec![("A".to_string(), "B".to_string())];

        assert_eq!(contacts, expected);
    }

    #[test]
    fn test_remove_remarks() {
        let input_pdb = "test_data/pdb_w_remark.pdb";
        let reader = remove_remark(input_pdb);

        // Collect the lines from the reader and check if the REMARK lines are removed
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        assert!(!lines.iter().any(|line| line.starts_with("REMARK")));
    }

    #[test]
    fn test_pad_short_lines() {
        let input_pdb = "test_data/pdb_w_short_lines.pdb";

        let reader = pad_lines(input_pdb);

        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        assert!(lines
            .iter()
            .filter(|line| line.starts_with("ATOM"))
            .all(|line| line.len() == 80));
    }
    #[test]
    fn test_pad_long_lines() {
        let input_pdb = "test_data/pdb_w_long_lines.pdb";

        let reader = pad_lines(input_pdb);

        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        assert!(lines
            .iter()
            .filter(|line| line.starts_with("ATOM"))
            .all(|line| line.len() == 80));
    }
}
