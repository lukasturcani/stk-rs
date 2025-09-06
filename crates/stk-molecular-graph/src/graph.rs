use crate::{Atom, Bond};
use std::collections::{HashMap, HashSet};

pub type AtomId = usize;
pub type BondId = usize;

#[derive(Debug, Clone)]
pub struct MolecularGraph {
    pub atoms: HashMap<AtomId, Atom>,
    bonds: HashMap<BondId, (AtomId, AtomId, Bond)>,
    atom_bonds: HashMap<AtomId, HashSet<BondId>>,
    next_atom_id: AtomId,
    next_bond_id: BondId,
}

impl MolecularGraph {
    pub fn new() -> Self {
        Self {
            atoms: HashMap::new(),
            bonds: HashMap::new(),
            atom_bonds: HashMap::new(),
            next_atom_id: 0,
            next_bond_id: 0,
        }
    }

    pub fn add_atom(&mut self, atom: Atom) -> AtomId {
        let id = self.next_atom_id;
        self.atoms.insert(id, atom);
        self.atom_bonds.insert(id, HashSet::new());
        self.next_atom_id += 1;
        id
    }

    pub fn add_bond(&mut self, atom1: AtomId, atom2: AtomId, bond: Bond) -> Option<BondId> {
        if !self.atoms.contains_key(&atom1) || !self.atoms.contains_key(&atom2) {
            return None;
        }

        let bond_id = self.next_bond_id;
        self.bonds.insert(bond_id, (atom1, atom2, bond));
        
        self.atom_bonds.get_mut(&atom1)?.insert(bond_id);
        self.atom_bonds.get_mut(&atom2)?.insert(bond_id);
        
        self.next_bond_id += 1;
        Some(bond_id)
    }

    pub fn get_atom(&self, id: AtomId) -> Option<&Atom> {
        self.atoms.get(&id)
    }

    pub fn get_bond(&self, id: BondId) -> Option<&(AtomId, AtomId, Bond)> {
        self.bonds.get(&id)
    }

    pub fn get_atoms(&self) -> &HashMap<AtomId, Atom> {
        &self.atoms
    }

    pub fn get_bonds(&self) -> &HashMap<BondId, (AtomId, AtomId, Bond)> {
        &self.bonds
    }

    pub fn get_neighbors(&self, atom_id: AtomId) -> Vec<AtomId> {
        let mut neighbors = Vec::new();
        if let Some(bond_ids) = self.atom_bonds.get(&atom_id) {
            for &bond_id in bond_ids {
                if let Some((atom1, atom2, _)) = self.bonds.get(&bond_id) {
                    if *atom1 == atom_id {
                        neighbors.push(*atom2);
                    } else {
                        neighbors.push(*atom1);
                    }
                }
            }
        }
        neighbors
    }

    pub fn get_bond_between(&self, atom1: AtomId, atom2: AtomId) -> Option<&Bond> {
        if let Some(bond_ids) = self.atom_bonds.get(&atom1) {
            for &bond_id in bond_ids {
                if let Some((a1, a2, bond)) = self.bonds.get(&bond_id) {
                    if (*a1 == atom1 && *a2 == atom2) || (*a1 == atom2 && *a2 == atom1) {
                        return Some(bond);
                    }
                }
            }
        }
        None
    }

    pub fn atom_count(&self) -> usize {
        self.atoms.len()
    }

    pub fn bond_count(&self) -> usize {
        self.bonds.len()
    }

    pub fn is_substructure_of(&self, other: &MolecularGraph) -> bool {
        // Simple substructure matching - can be enhanced with more sophisticated algorithms
        if self.atom_count() > other.atom_count() {
            return false;
        }

        // Try to find a mapping from self atoms to other atoms
        for &self_root in self.atoms.keys() {
            for &other_root in other.atoms.keys() {
                let mut mapping = HashMap::new();
                if self.match_recursive(self_root, other_root, other, &mut mapping) {
                    return true;
                }
            }
        }
        false
    }

    pub fn match_recursive(
        &self,
        self_atom: AtomId,
        other_atom: AtomId,
        other_graph: &MolecularGraph,
        mapping: &mut HashMap<AtomId, AtomId>,
    ) -> bool {
        // Check if atoms are compatible
        if let (Some(self_atom_data), Some(other_atom_data)) = 
            (self.get_atom(self_atom), other_graph.get_atom(other_atom)) {
            
            if !self.atoms_compatible(self_atom_data, other_atom_data) {
                return false;
            }
        } else {
            return false;
        }

        // Check if already mapped differently
        if let Some(&mapped_other) = mapping.get(&self_atom) {
            return mapped_other == other_atom;
        }

        // Check if other_atom is already mapped to a different self_atom
        for (&mapped_self, &mapped_other) in mapping.iter() {
            if mapped_other == other_atom && mapped_self != self_atom {
                return false;
            }
        }

        mapping.insert(self_atom, other_atom);

        let self_neighbors = self.get_neighbors(self_atom);
        let other_neighbors = other_graph.get_neighbors(other_atom);

        // Check if we can match all self neighbors
        for self_neighbor in &self_neighbors {
            let mut found_match = false;
            
            for other_neighbor in &other_neighbors {
                if mapping.get(self_neighbor) == Some(other_neighbor) {
                    found_match = true;
                    break;
                }
                
                if !mapping.contains_key(self_neighbor) && 
                   !mapping.values().any(|&v| v == *other_neighbor) {
                    
                    // Check bond compatibility
                    if let (Some(self_bond), Some(other_bond)) = (
                        self.get_bond_between(self_atom, *self_neighbor),
                        other_graph.get_bond_between(other_atom, *other_neighbor)
                    ) {
                        if self.bonds_compatible(self_bond, other_bond) {
                            let mut new_mapping = mapping.clone();
                            if self.match_recursive(*self_neighbor, *other_neighbor, other_graph, &mut new_mapping) {
                                *mapping = new_mapping;
                                found_match = true;
                                break;
                            }
                        }
                    }
                }
            }
            
            if !found_match {
                mapping.remove(&self_atom);
                return false;
            }
        }

        true
    }

    fn atoms_compatible(&self, pattern_atom: &Atom, target_atom: &Atom) -> bool {
        // For SMARTS patterns, we might have wildcards or specific requirements
        pattern_atom.element == target_atom.element &&
        pattern_atom.charge == target_atom.charge &&
        pattern_atom.aromatic == target_atom.aromatic
    }

    fn bonds_compatible(&self, pattern_bond: &Bond, target_bond: &Bond) -> bool {
        use crate::bond::BondType;
        
        match &pattern_bond.bond_type {
            BondType::Any => true,
            _ => pattern_bond.bond_type == target_bond.bond_type,
        }
    }
}

impl Default for MolecularGraph {
    fn default() -> Self {
        Self::new()
    }
}