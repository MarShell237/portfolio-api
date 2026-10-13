use entities::{users, users_roles};
use portfolio_api::{enums::user_role::UserRole, helpers::hash, repositories::user_repositories};
use sea_orm::{ActiveValue::Set, EntityTrait};
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
        let mut visitors: Vec<users::ActiveModel> =
            (0..5).map(|_| factories::user_factory::create()).collect();
        let mut test_user = factories::user_factory::create();
        test_user.email = Set("user@example.com".to_string());
        test_user.password = Set(hash::make("user@example.com".to_string()).unwrap());
        test_user.email_verified_at = Set(Some(chrono::Utc::now().naive_utc()));
        visitors.push(test_user);
        let visitors_model = users::Entity::insert_many(visitors)
            .exec_with_returning(db)
            .await?;
        for v in &visitors_model {
            let _ = user_repositories::assign_role(v.id, UserRole::VISITOR, db).await;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        users::Entity::delete_many().exec(db).await?;
        users_roles::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
