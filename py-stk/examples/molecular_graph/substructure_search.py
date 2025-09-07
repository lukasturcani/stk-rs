import stk
import itertools


def main() -> None:
    molecules = [
        stk.MolecularGraph.from_smiles(""),
    ]
    patterns = [
        stk.MolecularGraph.from_smarts(""),
    ]

    for molecule, pattern in itertools.product(molecules, patterns):
        pass


if __name__ == "__main__":
    main()
