//! This crate implements a nosql style database. the records are stored
//! is toml formated files. the record types are defined in the pam.toml
//! file. the entire database is backed by git.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PropertyType {
    String,
    Integer,
    Float,
    Boolean,
    Datetime,
    Array,
    Table,
}

pub type Object = std::collections::BTreeMap<String, PropertyType>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    objects: Vec<Object>,
}

#[cfg(test)]
mod tests {
    use super::*;

        
}
