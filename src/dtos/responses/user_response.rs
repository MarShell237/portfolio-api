use entities::users;
use sea_orm::{ConnectionTrait, DatabaseConnection, prelude::DateTime};
use serde::Serialize;

use crate::{errors::AppError, repositories::user_repositories};

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

impl UserResponse {
    pub async fn from_user(
        user: users::Model,
        db_pool: &DatabaseConnection,
    ) -> Result<Self, AppError>
// where
        // C: ConnectionTrait,
    {
        let role = user_repositories::get_role(user.id, db_pool).await?;
        Ok(Self {
            id: user.id,
            picture: user.picture,
            name: user.name,
            email: user.email,
            phone: user.phone,
            email_verified_at: user.email_verified_at,
            deleted_at: user.deleted_at,
            created_at: user.created_at,
            updated_at: user.updated_at,
            role: role.name,
        })
    }
}
