use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Literature::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Literature::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Literature::Title).string().not_null())
                    .col(ColumnDef::new(Literature::Text).string().not_null())
                    .col(ColumnDef::new(Literature::AuthorHandle).string().not_null())
                    .col(
                        ColumnDef::new(Literature::AuthorInstance)
                            .string()
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .col(Literature::AuthorHandle)
                            .col(Literature::AuthorInstance),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Art::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Art::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Art::Title).string().not_null())
                    .col(ColumnDef::new(Art::Data).binary().not_null())
                    .col(ColumnDef::new(Art::ThumbnailData).binary().not_null())
                    .col(ColumnDef::new(Art::AuthorHandle).string().not_null())
                    .col(ColumnDef::new(Art::AuthorInstance).string().not_null())
                    .index(
                        Index::create()
                            .unique()
                            .col(Art::AuthorHandle)
                            .col(Art::AuthorInstance),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Literature::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Art::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum Literature {
    Table,
    Id,
    Title,
    Text,
    AuthorHandle,
    AuthorInstance,
    IsNsfw,
}

#[derive(Iden)]
pub enum Art {
    Table,
    Id,
    Title,
    Data,
    ThumbnailData,
    AuthorHandle,
    AuthorInstance,
    Description,
    IsNsfw,
}
