pub mod atom;
pub mod bond;
pub mod graph;
pub mod parser;

pub use atom::Atom;
pub use bond::Bond;
pub use graph::MolecularGraph;
pub use parser::{SmilesParser, SmartsParser};

impl MolecularGraph {
    /// Parse a SMILES string into a molecular graph
    pub fn from_smiles(smiles: &str) -> Result<Self, ParseError> {
        SmilesParser::parse(smiles)
    }

    /// Parse a SMARTS string into a molecular graph pattern
    pub fn from_smarts(smarts: &str) -> Result<Self, ParseError> {
        SmartsParser::parse(smarts)
    }

    /// Check if this molecular graph contains the given substructure pattern
    pub fn contains_substructure(&self, pattern: &MolecularGraph) -> bool {
        pattern.is_substructure_of(self)
    }

    /// Find all matches of a SMARTS pattern in this molecular graph
    pub fn find_smarts_matches(&self, smarts_pattern: &str) -> Result<Vec<std::collections::HashMap<crate::graph::AtomId, crate::graph::AtomId>>, ParseError> {
        let pattern = Self::from_smarts(smarts_pattern)?;
        Ok(self.find_all_substructure_matches(&pattern))
    }

    /// Find all substructure matches (returns atom mappings)
    pub fn find_all_substructure_matches(&self, pattern: &MolecularGraph) -> Vec<std::collections::HashMap<crate::graph::AtomId, crate::graph::AtomId>> {
        let mut all_matches = Vec::new();
        
        if pattern.atom_count() > self.atom_count() {
            return all_matches;
        }

        // Try each atom in the target as a potential starting point
        for &target_root in self.atoms.keys() {
            for &pattern_root in pattern.atoms.keys() {
                let mut mapping = std::collections::HashMap::new();
                if pattern.match_recursive(pattern_root, target_root, self, &mut mapping) {
                    // Check if this mapping is unique (not already found)
                    if !all_matches.iter().any(|existing_mapping| mappings_equivalent(existing_mapping, &mapping)) {
                        all_matches.push(mapping);
                    }
                }
            }
        }
        
        all_matches
    }
}

fn mappings_equivalent(
    map1: &std::collections::HashMap<crate::graph::AtomId, crate::graph::AtomId>,
    map2: &std::collections::HashMap<crate::graph::AtomId, crate::graph::AtomId>
) -> bool {
    map1.len() == map2.len() && map1.iter().all(|(k, v)| map2.get(k) == Some(v))
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    InvalidSmiles(String),
    InvalidSmarts(String),
    UnknownAtom(String),
    InvalidBond(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidSmiles(msg) => write!(f, "Invalid SMILES: {}", msg),
            ParseError::InvalidSmarts(msg) => write!(f, "Invalid SMARTS: {}", msg),
            ParseError::UnknownAtom(atom) => write!(f, "Unknown atom: {}", atom),
            ParseError::InvalidBond(bond) => write!(f, "Invalid bond: {}", bond),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atom::Element;

    #[test]
    fn test_parse_simple_smiles() {
        let graph = MolecularGraph::from_smiles("CCO").unwrap();
        assert_eq!(graph.atom_count(), 3);
        assert_eq!(graph.bond_count(), 2);
    }

    #[test]
    fn test_parse_benzene_smiles() {
        let graph = MolecularGraph::from_smiles("c1ccccc1").unwrap();
        assert_eq!(graph.atom_count(), 6);
        assert_eq!(graph.bond_count(), 6);
    }

    #[test]
    fn test_parse_branched_smiles() {
        let graph = MolecularGraph::from_smiles("CC(C)O").unwrap();
        assert_eq!(graph.atom_count(), 4);
        assert_eq!(graph.bond_count(), 3);
    }

    #[test]
    fn test_parse_bracketed_atom() {
        let graph = MolecularGraph::from_smiles("[NH3+]").unwrap();
        assert_eq!(graph.atom_count(), 1);
        
        let atom = graph.get_atoms().values().next().unwrap();
        assert_eq!(atom.element, Element::N);
        assert_eq!(atom.charge, 1);
        assert_eq!(atom.explicit_hydrogens, 3);
    }

    #[test]
    fn test_parse_simple_smarts() {
        let pattern = MolecularGraph::from_smarts("CCO").unwrap();
        assert_eq!(pattern.atom_count(), 3);
        assert_eq!(pattern.bond_count(), 2);
    }

    #[test]
    fn test_parse_smarts_wildcard() {
        let pattern = MolecularGraph::from_smarts("C*O").unwrap();
        assert_eq!(pattern.atom_count(), 3);
        assert_eq!(pattern.bond_count(), 2);
    }

    #[test]
    fn test_substructure_search_simple() {
        let molecule = MolecularGraph::from_smiles("CCCO").unwrap();
        let pattern = MolecularGraph::from_smiles("CCO").unwrap();
        
        assert!(molecule.contains_substructure(&pattern));
    }

    #[test]
    fn test_substructure_search_not_found() {
        let molecule = MolecularGraph::from_smiles("CCC").unwrap();
        let pattern = MolecularGraph::from_smiles("CCO").unwrap();
        
        assert!(!molecule.contains_substructure(&pattern));
    }

    #[test]
    fn test_smarts_pattern_matching() {
        let molecule = MolecularGraph::from_smiles("CCCO").unwrap();
        let matches = molecule.find_smarts_matches("CCO").unwrap();
        
        assert!(!matches.is_empty());
    }

    #[test]
    fn test_benzene_ring_pattern() {
        let molecule = MolecularGraph::from_smiles("c1ccccc1").unwrap();
        let pattern = MolecularGraph::from_smarts("c1ccc(cc1)").unwrap();
        
        assert!(molecule.contains_substructure(&pattern));
    }

    #[test]
    fn test_aromatic_carbon_matching() {
        let molecule = MolecularGraph::from_smiles("c1ccccc1").unwrap();
        let pattern = MolecularGraph::from_smarts("c").unwrap();
        
        let matches = molecule.find_all_substructure_matches(&pattern);
        assert_eq!(matches.len(), 6); // Should match all 6 carbons in benzene
    }

    #[test]
    fn test_invalid_smiles() {
        let result = MolecularGraph::from_smiles("C(");
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_atom() {
        let result = MolecularGraph::from_smiles("CX");
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_ring_closure() {
        let graph = MolecularGraph::from_smiles("C1CCC1").unwrap();
        assert_eq!(graph.atom_count(), 4);
        assert_eq!(graph.bond_count(), 4); // 3 chain bonds + 1 ring closure
    }

    #[test]
    fn test_chirality_parsing() {
        let graph = MolecularGraph::from_smiles("[C@H](O)(N)C").unwrap();
        
        // Find the chiral carbon
        let chiral_atom = graph.get_atoms().values()
            .find(|atom| atom.chirality.is_some())
            .unwrap();
        
        assert_eq!(chiral_atom.element, Element::C);
        assert!(matches!(chiral_atom.chirality, Some(crate::atom::Chirality::Clockwise)));
    }

    #[test]
    fn test_charged_atom_parsing() {
        let graph = MolecularGraph::from_smiles("[O-2]").unwrap();
        
        let atom = graph.get_atoms().values().next().unwrap();
        assert_eq!(atom.element, Element::O);
        assert_eq!(atom.charge, -2);
    }

    #[test]
    fn test_hydrogen_count_parsing() {
        let graph = MolecularGraph::from_smiles("[CH3]").unwrap();
        
        let atom = graph.get_atoms().values().next().unwrap();
        assert_eq!(atom.element, Element::C);
        assert_eq!(atom.explicit_hydrogens, 3);
    }

    #[test]
    fn test_double_bond_parsing() {
        let graph = MolecularGraph::from_smiles("C=C").unwrap();
        assert_eq!(graph.atom_count(), 2);
        assert_eq!(graph.bond_count(), 1);
        
        // Check that the bond is a double bond
        let bond = graph.get_bonds().values().next().unwrap();
        assert_eq!(bond.2.bond_type, crate::bond::BondType::Double);
    }

    #[test]
    fn test_triple_bond_parsing() {
        let graph = MolecularGraph::from_smiles("C#C").unwrap();
        assert_eq!(graph.atom_count(), 2);
        assert_eq!(graph.bond_count(), 1);
        
        let bond = graph.get_bonds().values().next().unwrap();
        assert_eq!(bond.2.bond_type, crate::bond::BondType::Triple);
    }

    #[test]
    fn test_complex_molecule_parsing() {
        // Ethanol: CC(O)
        let graph = MolecularGraph::from_smiles("CCO").unwrap();
        assert_eq!(graph.atom_count(), 3);
        assert_eq!(graph.bond_count(), 2);
        
        // Check elements
        let elements: Vec<_> = graph.get_atoms().values()
            .map(|atom| &atom.element)
            .collect();
        
        assert!(elements.contains(&&Element::C));
        assert!(elements.contains(&&Element::O));
    }

    #[test]
    fn test_pattern_matching_with_branches() {
        let molecule = MolecularGraph::from_smiles("CC(C)(C)O").unwrap(); // tert-butanol
        let pattern = MolecularGraph::from_smiles("C(C)(C)C").unwrap(); // quaternary carbon pattern
        
        assert!(molecule.contains_substructure(&pattern));
    }
}