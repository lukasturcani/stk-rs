use crate::{Atom, Bond, MolecularGraph, ParseError};
use crate::atom::{Element, Chirality};
use crate::bond::BondType;
use std::collections::HashMap;

pub struct SmilesParser;

impl SmilesParser {
    pub fn parse(smiles: &str) -> Result<MolecularGraph, ParseError> {
        let mut parser = SmilesParserState::new(smiles);
        parser.parse()
    }
}

pub struct SmartsParser;

impl SmartsParser {
    pub fn parse(smarts: &str) -> Result<MolecularGraph, ParseError> {
        let mut parser = SmartsParserState::new(smarts);
        parser.parse()
    }
}

struct SmilesParserState {
    input: Vec<char>,
    pos: usize,
    graph: MolecularGraph,
    ring_bonds: HashMap<u8, (usize, Option<BondType>)>,
    current_atom: Option<usize>,
    branch_stack: Vec<usize>,
}

impl SmilesParserState {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            graph: MolecularGraph::new(),
            ring_bonds: HashMap::new(),
            current_atom: None,
            branch_stack: Vec::new(),
        }
    }

    fn parse(&mut self) -> Result<MolecularGraph, ParseError> {
        while self.pos < self.input.len() {
            match self.current_char() {
                Some('(') => {
                    if self.branch_stack.len() > 10 {
                        return Err(ParseError::InvalidSmiles("Too many nested branches".to_string()));
                    }
                    self.start_branch()?;
                }
                Some(')') => self.end_branch()?,
                Some(c) if c.is_ascii_digit() => self.handle_ring_bond()?,
                Some('%') => self.handle_two_digit_ring()?,
                Some(c) if self.is_bond_symbol(c) => {
                    let bond_type = BondType::from_char(c);
                    self.advance();
                    self.parse_next_atom(bond_type)?;
                }
                Some(c) if c.is_ascii_uppercase() || self.is_aromatic_atom(c) => {
                    self.parse_atom(None)?
                }
                Some('[') => self.parse_bracketed_atom(None)?,
                Some(c) => return Err(ParseError::InvalidSmiles(format!("Unexpected character: {}", c))),
                None => break,
            }
        }

        if !self.ring_bonds.is_empty() {
            return Err(ParseError::InvalidSmiles("Unclosed ring bonds".to_string()));
        }
        
        if !self.branch_stack.is_empty() {
            return Err(ParseError::InvalidSmiles("Unclosed branches".to_string()));
        }

        Ok(std::mem::take(&mut self.graph))
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn is_bond_symbol(&self, c: char) -> bool {
        matches!(c, '-' | '=' | '#' | ':' | '~')
    }

    fn is_aromatic_atom(&self, c: char) -> bool {
        matches!(c, 'c' | 'n' | 'o' | 's' | 'p')
    }

    fn parse_next_atom(&mut self, bond_type: Option<BondType>) -> Result<(), ParseError> {
        match self.current_char() {
            Some(c) if c.is_ascii_uppercase() || self.is_aromatic_atom(c) => {
                self.parse_atom(bond_type)
            }
            Some('[') => self.parse_bracketed_atom(bond_type),
            Some(c) => Err(ParseError::InvalidSmiles(format!("Expected atom after bond symbol, found: {}", c))),
            None => Err(ParseError::InvalidSmiles("Expected atom after bond symbol".to_string())),
        }
    }

    fn start_branch(&mut self) -> Result<(), ParseError> {
        if let Some(current) = self.current_atom {
            self.branch_stack.push(current);
        }
        self.advance();
        Ok(())
    }

    fn end_branch(&mut self) -> Result<(), ParseError> {
        if let Some(atom_id) = self.branch_stack.pop() {
            self.current_atom = Some(atom_id);
        } else {
            return Err(ParseError::InvalidSmiles("Unmatched closing parenthesis".to_string()));
        }
        self.advance();
        Ok(())
    }

    fn handle_ring_bond(&mut self) -> Result<(), ParseError> {
        let ring_num = self.current_char().unwrap().to_digit(10).unwrap() as u8;
        self.advance();

        if let Some(current) = self.current_atom {
            if let Some((other_atom, bond_type)) = self.ring_bonds.remove(&ring_num) {
                let bond = Bond::new(bond_type.unwrap_or(BondType::Single));
                self.graph.add_bond(current, other_atom, bond);
            } else {
                self.ring_bonds.insert(ring_num, (current, None));
            }
        }
        Ok(())
    }

    fn handle_two_digit_ring(&mut self) -> Result<(), ParseError> {
        self.advance(); // Skip '%'
        let mut ring_str = String::new();
        
        for _ in 0..2 {
            if let Some(c) = self.current_char() {
                if c.is_ascii_digit() {
                    ring_str.push(c);
                    self.advance();
                } else {
                    return Err(ParseError::InvalidSmiles("Expected digit after %".to_string()));
                }
            } else {
                return Err(ParseError::InvalidSmiles("Expected digit after %".to_string()));
            }
        }

        let ring_num = ring_str.parse::<u8>()
            .map_err(|_| ParseError::InvalidSmiles("Invalid ring number".to_string()))?;

        if let Some(current) = self.current_atom {
            if let Some((other_atom, bond_type)) = self.ring_bonds.remove(&ring_num) {
                let bond = Bond::new(bond_type.unwrap_or(BondType::Single));
                self.graph.add_bond(current, other_atom, bond);
            } else {
                self.ring_bonds.insert(ring_num, (current, None));
            }
        }
        Ok(())
    }

    fn parse_atom(&mut self, bond_type: Option<BondType>) -> Result<(), ParseError> {
        let mut atom_str = String::new();
        
        if let Some(c) = self.current_char() {
            atom_str.push(c);
            self.advance();
            
            // Only check for second character if the first is uppercase
            if c.is_ascii_uppercase() {
                if let Some(c2) = self.current_char() {
                    if c2.is_ascii_lowercase() {
                        atom_str.push(c2);
                        self.advance();
                    }
                }
            }
        }

        let (element, aromatic) = match atom_str.as_str() {
            "c" => (Element::C, true),
            "n" => (Element::N, true),
            "o" => (Element::O, true),
            "s" => (Element::S, true),
            "p" => (Element::P, true),
            _ => {
                let element = Element::from_str(&atom_str)
                    .ok_or_else(|| ParseError::UnknownAtom(atom_str.clone()))?;
                (element, false)
            }
        };

        let atom = Atom::new(element).with_aromatic(aromatic);
        let atom_id = self.graph.add_atom(atom);

        if let Some(prev_atom) = self.current_atom {
            let bond = Bond::new(bond_type.unwrap_or(BondType::Single));
            self.graph.add_bond(prev_atom, atom_id, bond);
        }

        self.current_atom = Some(atom_id);
        Ok(())
    }

    fn parse_bracketed_atom(&mut self, bond_type: Option<BondType>) -> Result<(), ParseError> {
        self.advance(); // Skip '['
        
        let mut bracket_content = String::new();
        let mut bracket_depth = 1;
        
        while bracket_depth > 0 && self.pos < self.input.len() {
            match self.current_char() {
                Some('[') => bracket_depth += 1,
                Some(']') => bracket_depth -= 1,
                _ => {}
            }
            
            if bracket_depth > 0 {
                bracket_content.push(self.current_char().unwrap());
            }
            self.advance();
        }

        if bracket_depth > 0 {
            return Err(ParseError::InvalidSmiles("Unclosed bracket".to_string()));
        }

        self.parse_bracketed_atom_content(&bracket_content, bond_type)
    }

    fn parse_bracketed_atom_content(&mut self, content: &str, bond_type: Option<BondType>) -> Result<(), ParseError> {
        let chars: Vec<char> = content.chars().collect();
        let mut pos = 0;

        // Parse element
        let mut element_str = String::new();
        if pos < chars.len() && chars[pos].is_ascii_uppercase() {
            element_str.push(chars[pos]);
            pos += 1;
            
            if pos < chars.len() && chars[pos].is_ascii_lowercase() {
                element_str.push(chars[pos]);
                pos += 1;
            }
        }

        let element = Element::from_str(&element_str)
            .ok_or_else(|| ParseError::UnknownAtom(element_str))?;

        let mut atom = Atom::new(element);

        // Parse chirality
        if pos < chars.len() && chars[pos] == '@' {
            pos += 1;
            if pos < chars.len() && chars[pos] == '@' {
                atom = atom.with_chirality(Chirality::CounterClockwise);
                pos += 1;
            } else {
                atom = atom.with_chirality(Chirality::Clockwise);
            }
        }

        // Parse hydrogen count
        if pos < chars.len() && chars[pos] == 'H' {
            pos += 1;
            let mut h_count = 1u8;
            
            if pos < chars.len() && chars[pos].is_ascii_digit() {
                h_count = chars[pos].to_digit(10).unwrap() as u8;
                pos += 1;
            }
            
            atom.explicit_hydrogens = h_count;
        }

        // Parse charge
        let mut charge = 0i8;
        if pos < chars.len() {
            match chars[pos] {
                '+' => {
                    pos += 1;
                    charge = 1;
                    if pos < chars.len() && chars[pos].is_ascii_digit() {
                        charge = chars[pos].to_digit(10).unwrap() as i8;
                    }
                }
                '-' => {
                    pos += 1;
                    charge = -1;
                    if pos < chars.len() && chars[pos].is_ascii_digit() {
                        charge = -(chars[pos].to_digit(10).unwrap() as i8);
                    }
                }
                _ => {}
            }
        }

        atom = atom.with_charge(charge);
        let atom_id = self.graph.add_atom(atom);

        if let Some(prev_atom) = self.current_atom {
            let bond = Bond::new(bond_type.unwrap_or(BondType::Single));
            self.graph.add_bond(prev_atom, atom_id, bond);
        }

        self.current_atom = Some(atom_id);
        Ok(())
    }
}

struct SmartsParserState {
    input: Vec<char>,
    pos: usize,
    graph: MolecularGraph,
    current_atom: Option<usize>,
    branch_stack: Vec<usize>,
    ring_bonds: HashMap<u8, (usize, Option<BondType>)>,
}

impl SmartsParserState {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            graph: MolecularGraph::new(),
            current_atom: None,
            branch_stack: Vec::new(),
            ring_bonds: HashMap::new(),
        }
    }

    fn parse(&mut self) -> Result<MolecularGraph, ParseError> {
        while self.pos < self.input.len() {
            match self.current_char() {
                Some('(') => self.start_branch()?,
                Some(')') => self.end_branch()?,
                Some('[') => self.parse_bracketed_atom()?,
                Some(c) if c.is_ascii_digit() => self.handle_ring_bond()?,
                Some('%') => self.handle_two_digit_ring()?,
                Some(c) if self.is_bond_symbol(c) => self.handle_explicit_bond()?,
                Some('*') => self.parse_wildcard_atom()?,
                Some(c) if c.is_ascii_uppercase() || c.is_ascii_lowercase() => {
                    self.parse_atom()?
                }
                Some(c) => return Err(ParseError::InvalidSmarts(format!("Unexpected character: {}", c))),
                None => break,
            }
        }

        Ok(std::mem::take(&mut self.graph))
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn is_bond_symbol(&self, c: char) -> bool {
        matches!(c, '-' | '=' | '#' | ':' | '~')
    }

    fn start_branch(&mut self) -> Result<(), ParseError> {
        if let Some(current) = self.current_atom {
            self.branch_stack.push(current);
        }
        self.advance();
        Ok(())
    }

    fn end_branch(&mut self) -> Result<(), ParseError> {
        if let Some(atom_id) = self.branch_stack.pop() {
            self.current_atom = Some(atom_id);
        }
        self.advance();
        Ok(())
    }

    fn handle_ring_bond(&mut self) -> Result<(), ParseError> {
        let ring_num = self.current_char().unwrap().to_digit(10).unwrap() as u8;
        self.advance();

        if let Some(current) = self.current_atom {
            if let Some((other_atom, bond_type)) = self.ring_bonds.remove(&ring_num) {
                let bond = Bond::new(bond_type.unwrap_or(BondType::Single));
                self.graph.add_bond(current, other_atom, bond);
            } else {
                self.ring_bonds.insert(ring_num, (current, None));
            }
        }
        Ok(())
    }

    fn handle_two_digit_ring(&mut self) -> Result<(), ParseError> {
        self.advance(); // Skip '%'
        let mut ring_str = String::new();
        
        for _ in 0..2 {
            if let Some(c) = self.current_char() {
                if c.is_ascii_digit() {
                    ring_str.push(c);
                    self.advance();
                }
            }
        }

        let ring_num = ring_str.parse::<u8>()
            .map_err(|_| ParseError::InvalidSmarts("Invalid ring number".to_string()))?;

        if let Some(current) = self.current_atom {
            if let Some((other_atom, bond_type)) = self.ring_bonds.remove(&ring_num) {
                let bond = Bond::new(bond_type.unwrap_or(BondType::Single));
                self.graph.add_bond(current, other_atom, bond);
            } else {
                self.ring_bonds.insert(ring_num, (current, None));
            }
        }
        Ok(())
    }

    fn handle_explicit_bond(&mut self) -> Result<(), ParseError> {
        let _bond_char = self.current_char().unwrap();
        self.advance();
        Ok(())
    }

    fn parse_wildcard_atom(&mut self) -> Result<(), ParseError> {
        self.advance(); // Skip '*'
        
        let atom = Atom::new(Element::C); // Placeholder for wildcard
        let atom_id = self.graph.add_atom(atom);

        if let Some(prev_atom) = self.current_atom {
            let bond = Bond::any();
            self.graph.add_bond(prev_atom, atom_id, bond);
        }

        self.current_atom = Some(atom_id);
        Ok(())
    }

    fn parse_atom(&mut self) -> Result<(), ParseError> {
        let mut atom_str = String::new();
        
        if let Some(c) = self.current_char() {
            atom_str.push(c);
            self.advance();
            
            // Only add second character if first is uppercase and second is lowercase
            // This prevents "cc" from being treated as a single atom
            if c.is_ascii_uppercase() {
                if let Some(c2) = self.current_char() {
                    if c2.is_ascii_lowercase() {
                        atom_str.push(c2);
                        self.advance();
                    }
                }
            }
        }

        let (element, aromatic) = match atom_str.as_str() {
            "c" => (Element::C, true),
            "n" => (Element::N, true),
            "o" => (Element::O, true),
            "s" => (Element::S, true),
            "p" => (Element::P, true),
            _ => {
                let element = Element::from_str(&atom_str.to_uppercase())
                    .ok_or_else(|| ParseError::UnknownAtom(atom_str.clone()))?;
                (element, false)
            }
        };

        let atom = Atom::new(element).with_aromatic(aromatic);
        let atom_id = self.graph.add_atom(atom);

        if let Some(prev_atom) = self.current_atom {
            let bond = Bond::single();
            self.graph.add_bond(prev_atom, atom_id, bond);
        }

        self.current_atom = Some(atom_id);
        Ok(())
    }

    fn parse_bracketed_atom(&mut self) -> Result<(), ParseError> {
        self.advance(); // Skip '['
        
        let mut bracket_content = String::new();
        let mut bracket_depth = 1;
        
        while bracket_depth > 0 && self.pos < self.input.len() {
            match self.current_char() {
                Some('[') => bracket_depth += 1,
                Some(']') => bracket_depth -= 1,
                _ => {}
            }
            
            if bracket_depth > 0 {
                bracket_content.push(self.current_char().unwrap());
            }
            self.advance();
        }

        self.parse_bracketed_atom_content(&bracket_content)
    }

    fn parse_bracketed_atom_content(&mut self, content: &str) -> Result<(), ParseError> {
        if content == "*" {
            let atom = Atom::new(Element::C);
            let atom_id = self.graph.add_atom(atom);

            if let Some(prev_atom) = self.current_atom {
                let bond = Bond::any();
                self.graph.add_bond(prev_atom, atom_id, bond);
            }

            self.current_atom = Some(atom_id);
            return Ok(());
        }

        // Simple element parsing for SMARTS
        let element = if !content.is_empty() {
            let first_char = content.chars().next().unwrap();
            if first_char.is_ascii_uppercase() {
                let mut element_str = String::new();
                element_str.push(first_char);
                
                if content.len() > 1 {
                    let second_char = content.chars().nth(1).unwrap();
                    if second_char.is_ascii_lowercase() {
                        element_str.push(second_char);
                    }
                }
                
                Element::from_str(&element_str)
                    .ok_or_else(|| ParseError::UnknownAtom(element_str))?
            } else {
                return Err(ParseError::InvalidSmarts("Invalid atom in brackets".to_string()));
            }
        } else {
            return Err(ParseError::InvalidSmarts("Empty brackets".to_string()));
        };

        let atom = Atom::new(element);
        let atom_id = self.graph.add_atom(atom);

        if let Some(prev_atom) = self.current_atom {
            let bond = Bond::single();
            self.graph.add_bond(prev_atom, atom_id, bond);
        }

        self.current_atom = Some(atom_id);
        Ok(())
    }
}