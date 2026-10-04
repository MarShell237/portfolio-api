use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use crate::errors::AppError;

pub fn make(password: String) -> Result<String, AppError> {
    Ok(Argon2::default()
        .hash_password(&password.as_bytes())
        .map_err(|e| AppError::internal(format!("Erreur de hachage: {e}")))?
        .to_string())
}

pub fn check(password: String, hash_password: String) -> Result<(), AppError> {
    let parsed_password = PasswordHash::new(&hash_password)
        .map_err(|e| AppError::bad_request(format!("Format de mot de passe invalide: {e}")))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_password)
        .map_err(|_| AppError::unauthorized(format!("Identifiants invalides. Veuillez vérifier votre adresse e-mail et votre mot de passe, puis réessayer.")))
}
