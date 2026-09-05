use entities::tags;
use sea_orm::EntityTrait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let tags: Vec<tags::ActiveModel> =
            (0..30).map(|_| factories::tag_factory::create()).collect();
        tags::Entity::insert_many(tags).exec(db).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        tags::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
