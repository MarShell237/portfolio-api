pub use sea_orm_migration::prelude::*;

mod m20260813_091351_create_users_table;
mod m20260813_182846_create_tags_table;
mod m20260813_185036_create_projects_table;
mod m20260813_191029_create_posts_table;
mod m20260813_192818_create_comments_table;
mod m20260813_214858_create_likes_table;
mod m20260813_215215_create_shares_table;
mod m20260813_223402_create_roles_table;
mod m20260813_223424_create_permissions_table;
mod m20260917_160946_create_users_roles_table;
mod m20260917_161024_create_users_permissions_table;
mod m20260917_161115_create_posts_tags_table;
mod m20260917_161123_create_projects_tags_table;
mod m20260917_201146_create_roles_permissions_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260813_091351_create_users_table::Migration),
            Box::new(m20260813_182846_create_tags_table::Migration),
            Box::new(m20260813_185036_create_projects_table::Migration),
            Box::new(m20260813_191029_create_posts_table::Migration),
            Box::new(m20260813_192818_create_comments_table::Migration),
            Box::new(m20260813_214858_create_likes_table::Migration),
            Box::new(m20260813_215215_create_shares_table::Migration),
            Box::new(m20260813_223402_create_roles_table::Migration),
            Box::new(m20260813_223424_create_permissions_table::Migration),
            Box::new(m20260917_160946_create_users_roles_table::Migration),
            Box::new(m20260917_161024_create_users_permissions_table::Migration),
            Box::new(m20260917_161115_create_posts_tags_table::Migration),
            Box::new(m20260917_161123_create_projects_tags_table::Migration),
            Box::new(m20260917_201146_create_roles_permissions_table::Migration),
        ]
    }
}
