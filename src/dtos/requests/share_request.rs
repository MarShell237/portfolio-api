use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct ShareRequest {
    #[validate(
        required(message = "Le champ platform est requis"),
        length(max = 50, message = "la plateforme doit faire maximum 50 caractères")
    )]
    pub platform: Option<String>,
}

pub struct ShareRequestValidated {
    pub platform: String,
}

impl From<ShareRequest> for ShareRequestValidated {
    fn from(value: ShareRequest) -> Self {
        Self {
            platform: value.platform.unwrap(),
        }
    }
}
