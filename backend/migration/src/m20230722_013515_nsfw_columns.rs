use sea_orm_migration::prelude::*;

use crate::m20230721_114813_art_and_literature_tables::{Art, Literature};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Literature::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Literature::IsNsfw)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Art::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Art::IsNsfw)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(Table::alter().drop_column(Literature::IsNsfw).to_owned())
            .await?;

        manager
            .alter_table(Table::alter().drop_column(Art::IsNsfw).to_owned())
            .await?;

        Ok(())
    }
}
