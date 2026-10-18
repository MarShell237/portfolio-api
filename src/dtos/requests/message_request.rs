use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct MessageRequest {
    #[validate(length(min = 5, message = "Le nom doit contenir au moins 5 caractères."))]
    pub name: String,

    // pub file_path: Option<String>,
    #[validate(email(message = "L'adresse email doit être valide."))]
    pub email: String,

    #[validate(regex(path = *PHONE_REGEX, message = "un '+' suivi de l’indicatif du pays, et uniquement des chiffres, éventuellement séparés par des espaces, des tirets ou des parenthèses."))]
    pub phone: String,

    #[validate(length(
        max = 1500,
        message = "Le sujet ne doit pas dépasser 1500 caractères."
    ))]
    pub subject: String,

    #[validate(length(
        max = 10000,
        message = "Le message ne doit pas dépasser 10000 caractères."
    ))]
    pub message: String,
}

static PHONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\+\d{1,3}[\s\-\(\)]?(\d[\s\-\(\)]?){6,14}$").unwrap());
