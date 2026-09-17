use entities::{users, users_roles};
use portfolio_api::{config, enums::user_role::UserRole, repositories::user_repositories};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    EntityTrait,
};
use sea_orm_migration::prelude::*;

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
        let admin = users::ActiveModel {
            id: NotSet,
            picture: NotSet,
            name: Set(config::ADMIN_NAME.clone()),
            email: Set(config::ADMIN_EMAIL.clone()),
            phone: Set(config::ADMIN_PHONE.clone()),
            email_verified_at: NotSet,
            password: Set(config::ADMIN_PASSWORD.clone()),
            deleted_at: NotSet,
            created_at: NotSet,
            updated_at: NotSet,
        };
        let admin_model = admin.insert(db).await?;

        let _ = user_repositories::assign_role(admin_model.id, UserRole::ADMIN, db).await;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        users::Entity::delete_many().exec(db).await?;
        users_roles::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
