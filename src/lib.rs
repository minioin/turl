use data_encoding::BASE64URL;
use std::fmt::Formatter;
use std::ops::Deref;

pub mod sha2;

#[derive(Debug)]
pub struct TinyId(u64);

impl TinyId {
    pub fn with(id: u64) -> Self {
        TinyId(id)
    }

    pub fn random() -> Self {
        todo!()
    }
}

impl Deref for TinyId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for TinyId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let bytes = BASE64URL.encode(&self.0.to_be_bytes());
        write!(f, "{}", bytes)
    }
}
