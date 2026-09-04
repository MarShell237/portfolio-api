use entities::posts as post;
use sea_orm::EntityTrait;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260904_161323_post_seeder"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let posts: Vec<post::ActiveModel> =
            (0..8).map(|_| factories::post_factory::create()).collect();
        post::Entity::insert_many(posts).exec(db).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        post::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
