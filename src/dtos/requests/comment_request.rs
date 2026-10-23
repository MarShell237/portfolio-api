use validator::Validate;

#[derive(Validate)]
pub struct CommentRequest {
    #[validate(
        required(message = "Le champ contenu est requis"),
        length(
            max = 10000,
            min = 1,
            message = "Le contenu du commentaire ne peut pas dépasser 10 000 caractères.",
        )
    )]
    pub content: Option<String>,
}

pub struct CommentRequestValidated {
    pub content: String,
}

impl From<CommentRequest> for CommentRequestValidated {
    fn from(c: CommentRequest) -> Self {
        Self {
            content: c.content.unwrap(),
        }
    }
}
