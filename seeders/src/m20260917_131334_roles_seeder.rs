use entities::roles;
use sea_orm::{ActiveValue::Set, EntityTrait};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260917_131334_roles_seeder"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let roles = [
            roles::ActiveModel {
                name: Set("ADMIN".to_owned()),
                ..Default::default()
            },
            roles::ActiveModel {
                name: Set("VISITOR".to_owned()),
                ..Default::default()
            },
        ];
        roles::Entity::insert_many(roles).exec(db).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        roles::Entity::delete_many()
            .exec(manager.get_connection())
            .await?;
        Ok(())
    }
}
