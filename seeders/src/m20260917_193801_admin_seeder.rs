use entities::users;
use portfolio_api::helpers::hash;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    EntityTrait,
};
use sea_orm_migration::prelude::*;

use crate::config;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260917_193801_admin_seeder"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        users::ActiveModel {
            id: NotSet,
            picture: NotSet,
            name: Set(config::ADMIN_NAME.clone()),
            email: Set(config::ADMIN_EMAIL.clone()),
            phone: Set(config::ADMIN_PHONE.clone()),
            email_verified_at: Set(Some(chrono::Utc::now().naive_utc())),
            password: Set(hash::make(config::ADMIN_PASSWORD.clone()).unwrap()),
            role: Set("ADMIN".to_string()),
            deleted_at: NotSet,
            created_at: NotSet,
            updated_at: NotSet,
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        users::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
