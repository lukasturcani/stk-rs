#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Element {
    H, He, Li, Be, B, C, N, O, F, Ne,
    Na, Mg, Al, Si, P, S, Cl, Ar,
    K, Ca, Sc, Ti, V, Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr,
    Rb, Sr, Y, Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te, I, Xe,
    // Add more elements as needed
}

impl Element {
    pub fn from_str(s: &str) -> Option<Element> {
        match s {
            "H" => Some(Element::H),
            "He" => Some(Element::He),
            "Li" => Some(Element::Li),
            "Be" => Some(Element::Be),
            "B" => Some(Element::B),
            "C" => Some(Element::C),
            "N" => Some(Element::N),
            "O" => Some(Element::O),
            "F" => Some(Element::F),
            "Ne" => Some(Element::Ne),
            "Na" => Some(Element::Na),
            "Mg" => Some(Element::Mg),
            "Al" => Some(Element::Al),
            "Si" => Some(Element::Si),
            "P" => Some(Element::P),
            "S" => Some(Element::S),
            "Cl" => Some(Element::Cl),
            "Ar" => Some(Element::Ar),
            "K" => Some(Element::K),
            "Ca" => Some(Element::Ca),
            "Sc" => Some(Element::Sc),
            "Ti" => Some(Element::Ti),
            "V" => Some(Element::V),
            "Cr" => Some(Element::Cr),
            "Mn" => Some(Element::Mn),
            "Fe" => Some(Element::Fe),
            "Co" => Some(Element::Co),
            "Ni" => Some(Element::Ni),
            "Cu" => Some(Element::Cu),
            "Zn" => Some(Element::Zn),
            "Ga" => Some(Element::Ga),
            "Ge" => Some(Element::Ge),
            "As" => Some(Element::As),
            "Se" => Some(Element::Se),
            "Br" => Some(Element::Br),
            "Kr" => Some(Element::Kr),
            "Rb" => Some(Element::Rb),
            "Sr" => Some(Element::Sr),
            "Y" => Some(Element::Y),
            "Zr" => Some(Element::Zr),
            "Nb" => Some(Element::Nb),
            "Mo" => Some(Element::Mo),
            "Tc" => Some(Element::Tc),
            "Ru" => Some(Element::Ru),
            "Rh" => Some(Element::Rh),
            "Pd" => Some(Element::Pd),
            "Ag" => Some(Element::Ag),
            "Cd" => Some(Element::Cd),
            "In" => Some(Element::In),
            "Sn" => Some(Element::Sn),
            "Sb" => Some(Element::Sb),
            "Te" => Some(Element::Te),
            "I" => Some(Element::I),
            "Xe" => Some(Element::Xe),
            _ => None,
        }
    }

    pub fn atomic_number(&self) -> u8 {
        match self {
            Element::H => 1,
            Element::He => 2,
            Element::Li => 3,
            Element::Be => 4,
            Element::B => 5,
            Element::C => 6,
            Element::N => 7,
            Element::O => 8,
            Element::F => 9,
            Element::Ne => 10,
            Element::Na => 11,
            Element::Mg => 12,
            Element::Al => 13,
            Element::Si => 14,
            Element::P => 15,
            Element::S => 16,
            Element::Cl => 17,
            Element::Ar => 18,
            Element::K => 19,
            Element::Ca => 20,
            Element::Sc => 21,
            Element::Ti => 22,
            Element::V => 23,
            Element::Cr => 24,
            Element::Mn => 25,
            Element::Fe => 26,
            Element::Co => 27,
            Element::Ni => 28,
            Element::Cu => 29,
            Element::Zn => 30,
            Element::Ga => 31,
            Element::Ge => 32,
            Element::As => 33,
            Element::Se => 34,
            Element::Br => 35,
            Element::Kr => 36,
            Element::Rb => 37,
            Element::Sr => 38,
            Element::Y => 39,
            Element::Zr => 40,
            Element::Nb => 41,
            Element::Mo => 42,
            Element::Tc => 43,
            Element::Ru => 44,
            Element::Rh => 45,
            Element::Pd => 46,
            Element::Ag => 47,
            Element::Cd => 48,
            Element::In => 49,
            Element::Sn => 50,
            Element::Sb => 51,
            Element::Te => 52,
            Element::I => 53,
            Element::Xe => 54,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    pub element: Element,
    pub charge: i8,
    pub implicit_hydrogens: u8,
    pub explicit_hydrogens: u8,
    pub aromatic: bool,
    pub chirality: Option<Chirality>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Chirality {
    Clockwise,
    CounterClockwise,
}

impl Atom {
    pub fn new(element: Element) -> Self {
        Self {
            element,
            charge: 0,
            implicit_hydrogens: 0,
            explicit_hydrogens: 0,
            aromatic: false,
            chirality: None,
        }
    }

    pub fn with_charge(mut self, charge: i8) -> Self {
        self.charge = charge;
        self
    }

    pub fn with_aromatic(mut self, aromatic: bool) -> Self {
        self.aromatic = aromatic;
        self
    }

    pub fn with_chirality(mut self, chirality: Chirality) -> Self {
        self.chirality = Some(chirality);
        self
    }

    pub fn total_hydrogens(&self) -> u8 {
        self.implicit_hydrogens + self.explicit_hydrogens
    }
}