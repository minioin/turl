use data_encoding::BASE64URL;
use std::fmt::Formatter;

pub mod sha2;

pub struct TinyId(u64);

impl TinyId {
    pub fn with(id: u64) -> Self {
        TinyId(id)
    }

    pub fn random() -> Self {
        TinyId(0)
    }
}

impl std::fmt::Display for TinyId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let bytes = BASE64URL.encode(&self.0.to_be_bytes());
        write!(f, "{}", bytes)
    }
}

