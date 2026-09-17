use entities::{users, users_roles};
use portfolio_api::{enums::user_role::UserRole, repositories::user_repositories};
use sea_orm::EntityTrait;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260917_143323_user_seeder"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let visitors: Vec<users::ActiveModel> =
            (0..5).map(|_| factories::user_factory::create()).collect();
        let visitors_model = users::Entity::insert_many(visitors)
            .exec_with_returning(db)
            .await?;
        let _ = visitors_model
            .iter()
            .map(async |v| user_repositories::assign_role(v.id, UserRole::VISITOR, db).await);
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        users::Entity::delete_many().exec(db).await?;
        users_roles::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
