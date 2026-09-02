use entities::tags as tag;
use factories::tag_factory::TagFactory;
use sea_orm::EntityTrait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let tags: Vec<tag::ActiveModel> = (0..10).map(|_| TagFactory::create()).collect();
        tag::Entity::insert_many(tags).exec(db).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        tag::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
