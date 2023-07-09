use rand::Rng;
use std::fmt::{Display, Formatter};

#[derive(Hash, PartialEq, Clone)]
pub(crate) struct SessionKey {
    key: String,
}

impl TryFrom<&String> for SessionKey {
    type Error = ();

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        return match s.strip_prefix("Bearer:") {
            Some(val) => Ok(SessionKey { key: val.to_string() }),
            None => Err(())
        };
    }
}

impl Display for SessionKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.to_str());
    }
}

impl SessionKey {
    fn to_str(&self) -> String {
        return self.key.clone();
    }
}

impl Into<String> for SessionKey {
    fn into(self) -> String {
        return self.to_str();
    }
}
