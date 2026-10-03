use entities::users;
use regex::Regex;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;
use std::sync::LazyLock;
use validator::{Validate, ValidationError, ValidationErrors};

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

impl RegisterRequest {
    pub async fn validate_uniqueness(
        &self,
        db: &DatabaseConnection,
    ) -> Result<(), actix_web_validator::Error> {
        let mut errors = ValidationErrors::new();

        if users::Entity::find()
            .filter(users::COLUMN.email.eq(&self.email))
            .one(db)
            .await
            .ok()
            .flatten()
            .is_some()
        {
            errors.add(
                "email",
                ValidationError::new("unique")
                    .with_message("La valeur du champ adresse e-mail est déjà utilisée.".into()),
            );
        }

        if users::Entity::find()
            .filter(users::COLUMN.phone.eq(&self.phone))
            .one(db)
            .await
            .ok()
            .flatten()
            .is_some()
        {
            errors.add(
                "phone",
                ValidationError::new("unique").with_message(
                    "La valeur du champ numeros de téléphone est déjà utilisée.".into(),
                ),
            );
        }

        if !errors.is_empty() {
            return Err(actix_web_validator::Error::Validate(errors));
        }

        Ok(())
    }
}
