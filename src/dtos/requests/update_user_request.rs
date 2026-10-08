use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct UpdateUserRequest {
    #[validate(length(min = 5, message = "Le nom doit contenir au moins 5 caractères."))]
    pub name: String,
    // pub photo: Option<String>,
}
