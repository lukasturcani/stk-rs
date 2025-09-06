use stk_molecular_graph::MolecularGraph;

fn main() {
    // Parse SMILES strings into molecular graphs
    let ethanol = MolecularGraph::from_smiles("CCO").unwrap();
    let benzene = MolecularGraph::from_smiles("c1ccccc1").unwrap();
    let tert_butanol = MolecularGraph::from_smiles("CC(C)(C)O").unwrap();

    println!("Ethanol: {} atoms, {} bonds", ethanol.atom_count(), ethanol.bond_count());
    println!("Benzene: {} atoms, {} bonds", benzene.atom_count(), benzene.bond_count());
    println!("Tert-butanol: {} atoms, {} bonds", tert_butanol.atom_count(), tert_butanol.bond_count());

    // Parse SMARTS patterns for substructure searching
    let alcohol_pattern = MolecularGraph::from_smarts("CO").unwrap();
    let aromatic_carbon = MolecularGraph::from_smarts("c").unwrap();

    // Test substructure searches
    println!("\nSubstructure search results:");
    println!("Ethanol contains alcohol pattern: {}", ethanol.contains_substructure(&alcohol_pattern));
    println!("Benzene contains alcohol pattern: {}", benzene.contains_substructure(&alcohol_pattern));
    println!("Tert-butanol contains alcohol pattern: {}", tert_butanol.contains_substructure(&alcohol_pattern));

    // Find all aromatic carbons in benzene
    let aromatic_matches = benzene.find_all_substructure_matches(&aromatic_carbon);
    println!("Number of aromatic carbons in benzene: {}", aromatic_matches.len());

    // Use SMARTS pattern matching directly
    let hydroxyl_matches = tert_butanol.find_smarts_matches("O").unwrap();
    println!("Number of oxygen atoms in tert-butanol: {}", hydroxyl_matches.len());

    // Complex SMARTS pattern
    let quaternary_carbon = MolecularGraph::from_smarts("C(C)(C)(C)C").unwrap();
    println!("Tert-butanol contains quaternary carbon: {}", tert_butanol.contains_substructure(&quaternary_carbon));
}