use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct MessageRequest {
    #[validate(
        required(message = "Le champ nom est requis"),
        length(min = 5, message = "Le nom doit contenir au moins 5 caractères.")
    )]
    pub name: Option<String>,

    #[validate(
        required(message = "Le champ email est requis"),
        email(message = "L'adresse email doit être valide.")
    )]
    pub email: Option<String>,

    #[validate(required(message = "Le champ numeros de telephone est requis"), regex(path = *PHONE_REGEX, message = "un '+' suivi de l’indicatif du pays, et uniquement des chiffres, éventuellement séparés par des espaces, des tirets ou des parenthèses."))]
    pub phone: Option<String>,

    #[validate(
        required(message = "Le champ sujet est requis"),
        length(max = 1500, message = "Le sujet ne doit pas dépasser 1500 caractères.")
    )]
    pub subject: Option<String>,

    #[validate(
        required(message = "Le champ message est requis"),
        length(
            max = 10000,
            message = "Le message ne doit pas dépasser 10000 caractères."
        )
    )]
    pub message: Option<String>,
}

static PHONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\+\d{1,3}[\s\-\(\)]?(\d[\s\-\(\)]?){6,14}$").unwrap());

pub struct MessageRequestValidated {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub subject: String,
    pub message: String,
}

impl From<MessageRequest> for MessageRequestValidated {
    fn from(m: MessageRequest) -> Self {
        Self {
            name: m.name.unwrap(),
            email: m.email.unwrap(),
            phone: m.phone.unwrap(),
            subject: m.subject.unwrap(),
            message: m.message.unwrap(),
        }
    }
}
