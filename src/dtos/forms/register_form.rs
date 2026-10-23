use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use entities::users;
use sea_orm::DatabaseConnection;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::dtos::requests::register_request::{RegisterRequest, RegisterRequestValidated};

#[derive(MultipartForm)]
pub struct RegisterForm {
    pub name: Option<Text<String>>,
    pub email: Option<Text<String>>,
    pub phone: Option<Text<String>>,
    pub picture: Option<TempFile>,
    pub password: Option<Text<String>>,
    pub password_confirmation: Option<Text<String>>,
}

impl RegisterForm {
    pub async fn validated_and_split(
        self,
        db: &DatabaseConnection,
    ) -> Result<(RegisterRequestValidated, Option<TempFile>), actix_web_validator::Error> {
        let mut errors = ValidationErrors::new();

        if let Some(ref my_file) = self.picture {
            if my_file.size > 5 * 1024 * 1024 {
                errors.add(
                    "picture",
                    ValidationError::new("max")
                        .with_message("La photo ne peut pas dépasser 5 Mo.".into()),
                );
            }

            match my_file.content_type {
                Some(ref content_type) => {
                    let allowed_content_type = ["image/jpeg", "image/jpg", "image/png"];
                    if !allowed_content_type.contains(&content_type.to_string().as_str()) {
                        errors.add(
                            "picture",
                            ValidationError::new("content_type").with_message(
                                "La photo doit être de type image (.jpg, .jpeg, .png).".into(),
                            ),
                        );
                    }
                }
                None => {
                    errors.add(
                        "picture",
                        ValidationError::new("content_type")
                            .with_message("La photo est invalide".into()),
                    );
                }
            }
        }

        let register_request = RegisterRequest {
            name: self.name.map(|t| t.into_inner()),
            email: self.email.map(|t| t.into_inner()),
            phone: self.phone.map(|t| t.into_inner()),
            password: self.password.map(|t| t.into_inner()),
            password_confirmation: self.password_confirmation.map(|t| t.into_inner()),
        };

        if let Err(mut val_errs) = register_request.validate() {
            for (field, err) in val_errs.errors_mut() {
                errors.errors_mut().insert(field.clone(), err.clone());
            }
        }

        if !errors.is_empty() {
            return Err(actix_web_validator::Error::Validate(errors));
        }

        if !register_request
            .password
            .clone()
            .unwrap()
            .eq(&register_request.password_confirmation.clone().unwrap())
        {
            errors.add(
                "password_confirmation",
                ValidationError::new("password_match").with_message(
                    "La confirmation du mot de passe ne correspond pas au mot de passe.".into(),
                ),
            );
        }

        if users::Entity::find_by_email(register_request.email.clone().unwrap())
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

        if users::Entity::find_by_phone(register_request.phone.clone().unwrap())
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

        Ok((
            RegisterRequestValidated::from(register_request),
            self.picture,
        ))
    }
}
