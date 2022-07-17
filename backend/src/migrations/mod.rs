pub use sea_orm_migration::prelude::*;

mod m20220717_000001_create_receipts_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220717_000001_create_receipts_tables::Migration)]
    }
}
