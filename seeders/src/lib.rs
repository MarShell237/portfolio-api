pub use sea_orm_migration::prelude::*;

mod m20260829_122936_tag_seeder;
mod m20260902_214327_project_seeder;
mod m20260904_161323_post_seeder;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260829_122936_tag_seeder::Migration),
            Box::new(m20260902_214327_project_seeder::Migration),
            Box::new(m20260904_161323_post_seeder::Migration),
        ]
    }

    fn migration_table_name() -> DynIden {
        Alias::new("seaql_seeders").into_iden()
    }
}
