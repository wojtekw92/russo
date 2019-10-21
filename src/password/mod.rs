extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};

pub fn hash_password(password: &String) -> String {
    hash(password.to_string(), DEFAULT_COST).unwrap()
}

pub fn check_hash(password: &String, hash: &str) -> bool {
    verify(password.to_string(), hash).unwrap()
}