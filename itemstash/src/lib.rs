use core::fmt;
use std::ops;

use toml_edit::DocumentMut;

#[derive(Debug, Default,)]
pub struct Stash {
    toml_document: DocumentMut,
}

impl Stash {
    pub fn new() -> Stash {
        let toml_document = DocumentMut::new();

        Stash {
            toml_document,
        }
    }

}

impl fmt::Display for Stash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.toml_document.fmt(f)
    }
}

impl<'s> ops::Index<&'s str> for Stash {
    type Output = toml_edit::Item;

    fn index(&self, key: &'s str) -> &toml_edit::Item {
        self.toml_document.index(key)
    }    
}

impl<'s> ops::IndexMut<&'s str> for Stash {
    fn index_mut(&mut self, key: &'s str) -> &mut toml_edit::Item {
        self.toml_document.index_mut(key)
    }    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stash() {
        let stash = Stash::new();
        let expected = r#""#;

        assert_eq!(stash.to_string(), expected);
    }

}
