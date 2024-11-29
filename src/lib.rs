//! This crate implements a nosql style database. 
//! 
//! [items.person]
//! name = "String"

use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize)]
pub struct ItemPropery {
    toml_type: String,
    required: bool,
}


impl ItemPropery {
    pub fn new(name: &str, toml_type: &str, required: bool) -> ItemPropery {

    }
}


#[cfg(test)]
mod tests {
    use super::*;


}
