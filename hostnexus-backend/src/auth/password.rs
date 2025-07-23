use argon2::{self, Config};

pub fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let salt = b"somesalt"; // можно сделать рандом
    argon2::hash_encoded(password.as_bytes(), salt, &Config::default())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap_or(false)
}
