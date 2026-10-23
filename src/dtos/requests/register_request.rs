use regex::Regex;
use std::sync::LazyLock;
use validator::Validate;

#[derive(Validate)]
pub struct RegisterRequest {
    #[validate(
        required(message = "Le champ nom est requis"),
        length(
            min = 5,
            max = 200,
            message = "Le nom doit contenir au moins 5 caractères."
        )
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
        required(message = "Le champ mot de passe est requis"),
        length(
            min = 8,
            max = 200,
            message = "Le mot de passe doit contenir au moins 8 caractères."
        )
    )]
    pub password: Option<String>,

    #[validate(required(message = "Le champ confirmation de mot de passe est requis"))]
    pub password_confirmation: Option<String>,
}

static PHONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\+\d{1,3}[\s\-\(\)]?(\d[\s\-\(\)]?){6,14}$").unwrap());

pub struct RegisterRequestValidated {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub password: String,
}
impl From<RegisterRequest> for RegisterRequestValidated {
    fn from(r: RegisterRequest) -> Self {
        Self {
            name: r.name.unwrap(),
            email: r.email.unwrap(),
            phone: r.phone.unwrap(),
            password: r.password.unwrap(),
        }
    }
}
