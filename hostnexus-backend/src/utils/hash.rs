use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand_core::OsRng;

/// Возвращает хеш пароля по Argon2 с солью
pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}
