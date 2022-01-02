use super::{AstName, LexType};
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct NameTableEntry {
    value: AstName,
    type_: LexType,
}

impl NameTableEntry {
    pub fn new(name: &str, type_: LexType) -> Self {
        NameTableEntry {
            value: AstName::new(name),
            type_,
        }
    }

    pub fn get_hash(&self) -> u32 {
        self.value.hash()
    }

    pub fn get_name(&self) -> AstName {
        self.value.clone()
    }

    pub fn get_type(&self) -> LexType {
        self.type_.clone()
    }
}

pub struct NameTable {
    data: HashMap<u32, NameTableEntry>,
}

impl NameTable {
    pub fn add(&mut self, name: &str, type_: LexType) -> AstName {
        let entry = &NameTableEntry::new(name, type_);
        self.data.insert(entry.get_hash(), entry.clone());

        entry.get_name()
    }

    pub fn get_or_add(&mut self, name: &str) -> (AstName, LexType) {
        let entry = &NameTableEntry::new(name, LexType::Eof);

        if let Some(existed_entry) = self.data.get(&entry.get_hash()) {
            (existed_entry.get_name(), existed_entry.get_type())
        } else {
            let lex_type = &LexType::Name(String::from(name));
            let name = self.add(name, lex_type.clone());

            (name, lex_type.clone())
        }
    }

    pub fn get(&self, name: &str) -> (AstName, LexType) {
        let entry = &NameTableEntry::new(name, LexType::Eof);

        if let Some(existed_entry) = self.data.get(&entry.get_hash()) {
            (existed_entry.get_name(), existed_entry.get_type())
        } else {
            (AstName::new(""), LexType::Name(String::new()))
        }
    }
}
