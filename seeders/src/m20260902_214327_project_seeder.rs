use entities::projects;
use sea_orm::EntityTrait;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260902_214327_project_seeder"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let projects: Vec<projects::ActiveModel> = (0..8)
            .map(|_| factories::project_factory::create())
            .collect();
        projects::Entity::insert_many(projects).exec(db).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        projects::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
