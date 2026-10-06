use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct LoginRequest {
    #[validate(email(message = "L'adresse email doit être valide."))]
    pub email: String,

    pub password: String,
}
