use validator::Validate;

#[derive(Validate)]
pub struct UpdateUserRequest {
    #[validate(
        required(message = "Le champ nom est requis"),
        length(
            min = 5,
            max = 200,
            message = "Le nom doit contenir au moins 5 caractères."
        )
    )]
    pub name: Option<String>,
}

pub struct UpdateUserRequestValidated {
    pub name: String,
}

impl From<UpdateUserRequest> for UpdateUserRequestValidated {
    fn from(value: UpdateUserRequest) -> Self {
        Self {
            name: value.name.unwrap(),
        }
    }
}
