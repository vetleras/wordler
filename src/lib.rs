pub mod guess;
pub mod guesser;

use std::fmt;
use std::str;

extern crate derive_more;

#[derive(Debug, Default, derive_more::Deref)]
pub struct Word(Vec<u8>);

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(str::from_utf8(&self.0).unwrap())
    }
}

impl From<&str> for Word {
    fn from(string: &str) -> Self {
        assert_eq!(string.len(), 5);
        Word(string.bytes().collect())
    }
}

impl From<String> for Word {
    fn from(string: String) -> Self {
        assert_eq!(string.len(), 5);
        Word(string.bytes().collect())
    }
}
