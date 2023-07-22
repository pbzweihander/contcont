pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230721_114813_art_and_literature_tables;
mod m20230721_151213_vote_tables;
mod m20230722_010433_art_description_column;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230721_114813_art_and_literature_tables::Migration),
            Box::new(m20230721_151213_vote_tables::Migration),
            Box::new(m20230722_010433_art_description_column::Migration),
        ]
    }
}
