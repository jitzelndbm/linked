use std::{collections::HashMap, fs, path::PathBuf, str::FromStr};

use crate::error::{Error, Result};

pub type Username = String;
type PasswordHash = String;

#[derive(Clone)]
pub struct Users(HashMap<Username, PasswordHash>);

const COMMENT: char = '#';
const SEPARATOR: char = ':';

//impl Htpasswd {
//    fn verify(&self, user: User) -> Result<User, Box<dyn ::std::error::Error>> {
//        let hash = self.0.get(&user.username).ok_or("User not found")?;
//        if verify(user.password_hash.clone(), hash)? {
//            Ok(user)
//        } else {
//            Err("Password wrong".into())
//        }
//    }
//}

impl TryFrom<PathBuf> for Users {
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self> {
        let content = fs::read_to_string(value).map_err(|_| Error::HtpasswdFileNotFound)?;
        Self::from_str(&content)
    }
}

impl FromStr for Users {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // TODO: add checks if users are unique
        Ok(Self(
            s.lines()
                .map(str::trim)
                .filter(|line| !line.is_empty() && !line.starts_with(COMMENT))
                .map(|line| {
                    let mut parts = line.splitn(2, SEPARATOR);
                    let user = parts.next().ok_or(Error::Htpasswd)?;
                    let pass = parts.next().ok_or(Error::Htpasswd)?;
                    Ok((user.to_string(), pass.to_string()))
                })
                .collect::<Result<HashMap<Username, PasswordHash>>>()?,
        ))
    }
}
