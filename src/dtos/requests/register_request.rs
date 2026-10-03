use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate, Debug)]
pub struct RegisterRequest {
    #[validate(length(min = 5, message = "Le nom doit contenir au moins 5 caractères."))]
    pub name: String,

    // pub photo: Option<String>,
    #[validate(email(message = "L'adresse email doit être valide."))]
    pub email: String,

    #[validate(regex(path = *PHONE_REGEX, message = "un '+' suivi de l’indicatif du pays, et uniquement des chiffres, éventuellement séparés par des espaces, des tirets ou des parenthèses."))]
    pub phone: String,

    #[validate(length(
        min = 8,
        message = "Le mot de passe doit contenir au moins 8 caractères."
    ))]
    pub password: String,

    #[validate(must_match(
        other = "password",
        message = "La confirmation du mot de passe ne correspond pas au mot de passe."
    ))]
    pub password_confirmation: String,
}

static PHONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\+\d{1,3}[\s\-\(\)]?(\d[\s\-\(\)]?){6,14}$").unwrap());
