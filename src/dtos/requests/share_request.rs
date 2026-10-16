use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct ShareRequest {
    #[validate(length(
        max = 50,
        message = "la plateforme doit faire maximum 50 caractères"
    ))]
    pub platform: String,
}
