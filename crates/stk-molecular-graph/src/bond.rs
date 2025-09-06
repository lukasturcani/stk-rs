#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BondType {
    Single,
    Double,
    Triple,
    Aromatic,
    Any, // Used in SMARTS patterns
}

impl BondType {
    pub fn from_char(c: char) -> Option<BondType> {
        match c {
            '-' => Some(BondType::Single),
            '=' => Some(BondType::Double),
            '#' => Some(BondType::Triple),
            ':' => Some(BondType::Aromatic),
            '~' => Some(BondType::Any),
            _ => None,
        }
    }

    pub fn is_aromatic(&self) -> bool {
        matches!(self, BondType::Aromatic)
    }

    pub fn order(&self) -> Option<u8> {
        match self {
            BondType::Single => Some(1),
            BondType::Double => Some(2),
            BondType::Triple => Some(3),
            BondType::Aromatic => Some(1), // Aromatic bonds are typically single bond order
            BondType::Any => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bond {
    pub bond_type: BondType,
    pub stereo: Option<BondStereo>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BondStereo {
    Up,
    Down,
    Either,
}

impl Bond {
    pub fn new(bond_type: BondType) -> Self {
        Self {
            bond_type,
            stereo: None,
        }
    }

    pub fn single() -> Self {
        Self::new(BondType::Single)
    }

    pub fn double() -> Self {
        Self::new(BondType::Double)
    }

    pub fn triple() -> Self {
        Self::new(BondType::Triple)
    }

    pub fn aromatic() -> Self {
        Self::new(BondType::Aromatic)
    }

    pub fn any() -> Self {
        Self::new(BondType::Any)
    }

    pub fn with_stereo(mut self, stereo: BondStereo) -> Self {
        self.stereo = Some(stereo);
        self
    }
}