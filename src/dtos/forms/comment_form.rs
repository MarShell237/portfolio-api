use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use validator::{Validate, ValidationError, ValidationErrors};

use crate::dtos::requests::comment_request::{CommentRequest, CommentRequestValidated};

#[derive(Debug, MultipartForm)]
pub struct CommentForm {
    pub content: Option<Text<String>>,
    pub attachment: Option<TempFile>,
}

impl CommentForm {
    pub fn validate_and_split(
        self,
    ) -> Result<(CommentRequestValidated, Option<TempFile>), actix_web_validator::Error> {
        let mut errors = ValidationErrors::new();
        if let Some(ref my_file) = self.attachment {
            if my_file.size > 20 * 1024 * 1024 {
                errors.add(
                    "attachment",
                    ValidationError::new("max")
                        .with_message("Le fichier joint ne peut pas dépasser 20 Mo.".into()),
                );
            }

            match my_file.content_type {
                Some(ref content_type) => {
                    let allowed_content_type =
                        ["image/jpeg", "image/jpg", "image/png", "image/gif"];
                    if !allowed_content_type.contains(&content_type.to_string().as_str()) {
                        errors.add(
                        "attachment",
                        ValidationError::new("content_type").with_message(
                            "Le fichier joint doit être de type image (.jpg, .jpeg, .png, .gif)."
                                .into(),
                        ),
                    );
                    }
                }
                None => {
                    errors.add(
                        "attachment",
                        ValidationError::new("content_type")
                            .with_message("Le fichier joint est invalide".into()),
                    );
                }
            }
        }

        let comment_request = CommentRequest {
            content: self.content.map(|t| t.into_inner()),
        };

        if let Err(mut val_errs) = comment_request.validate() {
            for (field, err) in val_errs.errors_mut() {
                errors.errors_mut().insert(field.clone(), err.clone());
            }
        }

        if !errors.is_empty() {
            return Err(actix_web_validator::Error::Validate(errors));
        }

        Ok((
            CommentRequestValidated::from(comment_request),
            self.attachment,
        ))
    }
}
