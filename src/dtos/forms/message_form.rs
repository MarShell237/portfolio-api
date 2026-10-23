use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use validator::{Validate, ValidationError, ValidationErrors};

use crate::dtos::requests::message_request::{MessageRequest, MessageRequestValidated};

#[derive(MultipartForm, Debug)]
pub struct MessageForm {
    pub name: Option<Text<String>>,
    pub attachment: Option<TempFile>,
    pub email: Option<Text<String>>,
    pub phone: Option<Text<String>>,
    pub subject: Option<Text<String>>,
    pub message: Option<Text<String>>,
}

impl MessageForm {
    pub fn validate_and_split(
        self,
    ) -> Result<(MessageRequestValidated, Option<TempFile>), ValidationErrors> {
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
                    let allowed_content_type = [
                        "image/jpeg",
                        "image/jpg",
                        "image/png",
                        "image/gif",
                        "application/msword",
                        "video/mp4",
                        "application/zip",
                        "application/x-zip-compressed",
                        "application/pdf",
                    ];
                    if !allowed_content_type.contains(&content_type.to_string().as_str()) {
                        errors.add(
                        "attachment",
                        ValidationError::new("content_type").with_message(
                            "Le fichier joint doit avoir l'extension (.jpg, .jpeg, .png, .gif, .pdf, .docx, .zip, .mp4)."
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

        let message_request = MessageRequest {
            name: self.name.map(|t| t.into_inner()),
            email: self.email.map(|t| t.into_inner()),
            phone: self.phone.map(|t| t.into_inner()),
            subject: self.subject.map(|t| t.into_inner()),
            message: self.message.map(|t| t.into_inner()),
        };

        if let Err(mut val_errs) = message_request.validate() {
            for (field, err) in val_errs.errors_mut() {
                errors.errors_mut().insert(field.clone(), err.clone());
            }
        }

        Ok((
            MessageRequestValidated::from(message_request),
            self.attachment,
        ))
    }
}
