use sea_orm_migration::prelude::*;

#[derive(Iden)]
enum Instance {
    Table,
    Hostname,
    ClientId,
    ClientSecret,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Instance::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Instance::Hostname)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Instance::ClientId).string().not_null())
                    .col(ColumnDef::new(Instance::ClientSecret).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Instance::Table).to_owned())
            .await
    }
}
