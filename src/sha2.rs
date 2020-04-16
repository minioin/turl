use crate::TinyId;
use sha2::{Digest, Sha256};

impl From<&[u8]> for TinyId {
    fn from(input: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.input(input);
        let result = hasher.result();
        Self(to_u64(result.as_slice().split_at(8).0))
    }
}

impl From<&str> for TinyId {
    fn from(input: &str) -> Self {
        Self::from(input.as_bytes())
    }
}

fn to_u64(values: &[u8]) -> u64 {
    values.iter().take(8).fold(0, |x, &i| x << 8 | i as u64)
}

mod test {
    use crate::TinyId;

    #[test]

    fn test() {
        let url = "https://dbhattarai.info.np/";
        let id: TinyId = url.into();
        assert_eq!(format!("{}", id), "QMlX0WD1Fmg=");
    }
}
