use entities::{messages, users};
use sea_orm::{ActiveValue::Set, EntityTrait};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261017_095701_message_seeder"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_pool = manager.get_connection();
        let users = users::Entity::find().all(db_pool).await?;

        let mut messages_active_model = Vec::new();
        let user_count = users.len();

        for i in 0..211 {
            let random_user = &users[(100 * i) % user_count];
            let mut message_active_model = factories::message_factory::create();
            message_active_model.name = Set(random_user.name.clone());
            message_active_model.phone = Set(random_user.phone.clone());
            message_active_model.email = Set(random_user.email.clone());
            messages_active_model.push(message_active_model);
        }
        messages::Entity::insert_many(messages_active_model)
            .exec(db_pool)
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        messages::Entity::delete_many()
            .exec(manager.get_connection())
            .await?;
        Ok(())
    }
}
