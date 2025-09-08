"""Substructure search example."""

import itertools

from stk.molecular_graph import MolecularGraph


def main() -> None:
    """Run the example."""
    molecules = [
        MolecularGraph.from_smiles(""),
    ]
    patterns = [
        MolecularGraph.from_smarts(""),
    ]

    for molecule, pattern in itertools.product(molecules, patterns):
        pass


if __name__ == "__main__":
    main()
