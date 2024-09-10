pub use sea_orm_migration::prelude::*;

mod m20240910_164721_create_table_test;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![

            Box::new(m20240910_164721_create_table_test::Migration),
        ]
    }
}
