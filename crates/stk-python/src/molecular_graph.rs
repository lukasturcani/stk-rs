use std::sync::RwLock;

use pyo3::prelude::*;
use stk_molecular_graph::MolecularGraph;

#[pyclass(frozen)]
#[repr(transparent)]
pub struct PyMolecularGraph {
    pub graph: RwLock<MolecularGraph>,
}

impl PyMolecularGraph {
    pub fn new(graph: MolecularGraph) -> Self {
        Self {
            graph: RwLock::new(graph),
        }
    }
}

#[pymethods]
impl PyMolecularGraph {
    pub fn from_smiles(&self, smiles: &str) -> PyResult<PyMolecularGraph> {
        let graph = MolecularGraph::from_smiles(smiles).into()?;
        Ok(PyMolecularGraph::new(graph))
    }

    pub fn from_smarts(&self, smarts: &str) -> PyResult<PyMolecularGraph> {
        let graph = MolecularGraph::from_smarts(smarts)?;
        Ok(PyMolecularGraph::new(graph))
    }
}
