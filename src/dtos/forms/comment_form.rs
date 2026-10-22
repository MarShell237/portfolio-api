use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use validator::{ValidationError, ValidationErrors};

use crate::dtos::requests::comment_request::CommentRequest;

#[derive(Debug, MultipartForm)]
pub struct CommentForm {
    pub content: Text<String>,
    pub attachment: Option<TempFile>,
}

impl CommentForm {
    pub fn validate_and_split(
        self,
    ) -> Result<(CommentRequest, Option<TempFile>), actix_web_validator::Error> {
        let mut errors = ValidationErrors::new();
        if let Some(ref my_file) = self.attachment {
            if my_file.size > 20480 * 1024 {
                errors.add(
                    "attachment",
                    ValidationError::new("max")
                        .with_message("Le fichier joint ne peut pas dépasser 20 Mo.".into()),
                );
            }

            if let Some(ref content_type) = my_file.content_type {
                let allowed_content_type = ["image/jpeg", "image/jpg", "image/png", "image/gif"];
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
        }

        if !errors.is_empty() {
            return Err(actix_web_validator::Error::Validate(errors));
        }

        Ok((
            CommentRequest {
                content: self.content.to_string(),
            },
            self.attachment,
        ))
    }
}
