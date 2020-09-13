#[macro_use]
extern crate serde_derive;

use getrandom::getrandom;
use itertools::Itertools;
use std::num::ParseIntError;
use thiserror::Error;

const RANDOM_HASH_SIZE: usize = 32;
pub type Seed = [u8; RANDOM_HASH_SIZE];

#[derive(Error, Debug)]
pub enum ObjectIdError {
    #[error("Impossible to generate a random hash")]
    GenerationImpossible,
    #[error("Impossible to get ID from String : impossible to convert string")]
    CastFromStringImpossibleDecode,
    #[error("Impossible to get ID from String : string size incorrect")]
    CastFromStringImpossibleSize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjectId {
    pub seed: Seed,
}

impl ObjectId {
    #[inline]
    pub fn new() -> Result<Self, ObjectIdError> {
        let mut random = [0u8; RANDOM_HASH_SIZE];
        match getrandom(&mut random) {
            Ok(_) => Ok(ObjectId { seed: random }),
            Err(_) => Err(ObjectIdError::GenerationImpossible),
        }
    }

    #[inline]
    pub fn from_string(id: String) -> Result<Self, ObjectIdError> {
        match ObjectId::hex_string_to_vec(&id) {
            Ok(vec) => match vec.len() == RANDOM_HASH_SIZE {
                true => {
                    let mut seed = [0u8; RANDOM_HASH_SIZE];
                    seed.clone_from_slice(vec.as_slice());
                    Ok(ObjectId { seed: seed })
                }
                false => Err(ObjectIdError::CastFromStringImpossibleSize),
            },
            Err(_) => Err(ObjectIdError::CastFromStringImpossibleDecode),
        }
    }

    fn hex_string_to_vec(s: &str) -> Result<Vec<u8>, ParseIntError> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
    }
}

impl AsRef<[u8]> for ObjectId {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.seed
    }
}

impl ToString for ObjectId {
    #[inline(always)]
    fn to_string(&self) -> String {
        format!("{:02x}", self.seed.as_ref().iter().format(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_id() {
        let id = ObjectId::new().unwrap();
        println!("{:?}", id.to_string());
        println!("{:?}", id.to_string().len());
        assert_eq!(id.seed.len(), RANDOM_HASH_SIZE);
    }

    #[test]
    fn get_id_from_digest() {
        let digest: String =
            "74dc1afb9a999c65b40e8e8d20cfa5651a6178b0280397419271fdc86439f6e2".to_owned();

        let id = ObjectId::from_string(digest.clone()).unwrap();
        assert_eq!(id.to_string(), digest.to_owned());
        assert_eq!(id.seed.len(), 32);
    }
}
