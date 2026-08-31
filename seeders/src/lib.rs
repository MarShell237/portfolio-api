pub use sea_orm_migration::prelude::*;

mod m20260829_122936_tag_seeder;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260829_122936_tag_seeder::Migration)]
    }

    fn migration_table_name() -> DynIden {
        Alias::new("seaql_seeders").into_iden()
    }
}
