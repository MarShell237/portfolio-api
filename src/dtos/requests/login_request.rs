use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct LoginRequest {
    #[validate(
        required(message = "Le champ email est requis"),
        email(message = "L'adresse email doit être valide.")
    )]
    pub email: Option<String>,

    #[validate(required(message = "Le champ mot de passe est requis"))]
    pub password: Option<String>,
}

pub struct LoginRequestValidated {
    pub email: String,
    pub password: String,
}

impl From<LoginRequest> for LoginRequestValidated {
    fn from(l: LoginRequest) -> Self {
        Self {
            email: l.email.unwrap(),
            password: l.password.unwrap(),
        }
    }
}
