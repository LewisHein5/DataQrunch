use rand::Rng;
use std::fmt::{Display, Formatter};

#[derive(Hash, PartialEq, Clone)]
pub(crate) struct SessionKey {
    key: Vec<bool>,
}

impl TryFrom<&String> for SessionKey {
    type Error = ();

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let mut key: Vec<bool> = Vec::new();
        if s.len() == 4 {
            for c in s.chars() {
                key.push(match c {
                    '0' => false,
                    '1' => true,
                    _ => break,
                })
            }
            return Ok(SessionKey { key });
        } else {
            return Err(());
        }
    }
}

impl Display for SessionKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.to_str());
    }
}

impl SessionKey {
    pub(crate) fn new() -> SessionKey {
        let mut rng = rand::thread_rng();
        let mut key: Vec<bool> = Vec::new();
        for _i in 0..4 {
            //TODO: Use a variable for key length
            key.push(rng.gen::<bool>());
        }

        return SessionKey { key };
    }

    fn to_str(&self) -> String {
        let mut str = String::new();
        for digit in &self.key {
            str.push(match digit {
                true => '1',
                false => '0',
            });
        }

        return str;
    }
}

impl Into<String> for SessionKey {
    fn into(self) -> String {
        return self.to_str();
    }
}
