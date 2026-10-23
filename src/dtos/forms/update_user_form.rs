use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use validator::{Validate, ValidationError, ValidationErrors};

use crate::dtos::requests::update_user_request::{UpdateUserRequest, UpdateUserRequestValidated};

#[derive(MultipartForm)]
pub struct UpdateUserForm {
    pub name: Option<Text<String>>,
    pub picture: Option<TempFile>,
}

impl UpdateUserForm {
    pub fn validate_and_split(
        self,
    ) -> Result<(UpdateUserRequestValidated, Option<TempFile>), actix_web_validator::Error> {
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

        let update_user_request = UpdateUserRequest {
            name: self.name.map(|t| t.into_inner()),
        };

        if let Err(mut val_errs) = update_user_request.validate() {
            for (field, err) in val_errs.errors_mut() {
                errors.errors_mut().insert(field.clone(), err.clone());
            }
        }

        if !errors.is_empty() {
            return Err(actix_web_validator::Error::Validate(errors));
        }

        Ok((
            UpdateUserRequestValidated::from(update_user_request),
            self.picture,
        ))
    }
}
