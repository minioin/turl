use url::Url;
use crate::TinyId;
use sha2::{Sha256, Digest};

impl From<Url> for TinyId {
    fn from(url: Url) -> Self {
        let mut hasher = Sha256::new();
        hasher.input(url.as_str());
        let result = hasher.result();
        Self(to_u64(result.as_slice().split_at(8).0))
    }
}

fn to_u64(values: &[u8]) -> u64 {
    println!("{}", values.len());
    values.iter().fold(0, |x, &i| x << 8 | i as u64)
}

mod test {
    use crate::TinyId;
    use url::Url;

    #[test]

    fn test() {
        let url: Url = "https://dbhattarai.info.np/".parse::<Url>().unwrap();
        let id: TinyId = url.into();
        assert_eq!(format!("{}", id), "QMlX0WD1Fmg=");
    }
}