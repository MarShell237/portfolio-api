use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CommentRequest {
    #[validate(length(
        max = 10000,
        message = "Le contenu du commentaire ne peut pas dépasser 10 000 caractères.",
    ))]
    pub content: String,
}
