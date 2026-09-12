use entities::{project_tag, projects};
use sea_orm::{ActiveValue::Set, EntityTrait};
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
        let projects_data: Vec<projects::ActiveModel> = (0..8)
            .map(|_| factories::project_factory::create())
            .collect();
        let inserted_projects = projects::Entity::insert_many(projects_data)
            .exec_with_returning(db)
            .await?;

        let mut pivot_relations = Vec::new();

        for project in inserted_projects {
            let tag_base_id = project.id;

            for offset in 0..3 {
                pivot_relations.push(project_tag::ActiveModel {
                    project_id: Set(project.id),
                    tag_id: Set(tag_base_id + offset),
                    ..Default::default()
                });
            }
        }

        project_tag::Entity::insert_many(pivot_relations)
            .exec(db)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        project_tag::Entity::delete_many().exec(db).await?;
        projects::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
