pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20240114_105650_create_login_history;
mod m20240127_130539_create_user_info;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20240114_105650_create_login_history::Migration),
            Box::new(m20240127_130539_create_user_info::Migration),
        ]
    }
}
