use entities::users;
use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub picture: Option<String>,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub email_verified_at: Option<String>,
    pub deleted_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub role: String,
}

impl From<users::Model> for UserResponse {
    fn from(user: users::Model) -> Self {
        Self {
            id: user.id,
            picture: user.picture,
            name: user.name,
            email: user.email,
            phone: user.phone,
            email_verified_at: user.email_verified_at,
            deleted_at: user.deleted_at,
            created_at: user.created_at,
            updated_at: user.updated_at,
            role: "VISITOR".to_string(),
        }
    }
}
