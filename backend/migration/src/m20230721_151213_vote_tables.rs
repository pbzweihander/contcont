use sea_orm_migration::prelude::*;

use crate::m20230721_114813_art_and_literature_tables::{Art, Literature};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LiteratureVote::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LiteratureVote::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LiteratureVote::Handle).string().not_null())
                    .col(ColumnDef::new(LiteratureVote::Instance).string().not_null())
                    .col(
                        ColumnDef::new(LiteratureVote::LiteratureId)
                            .integer()
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .col(LiteratureVote::Handle)
                            .col(LiteratureVote::Instance)
                            .col(LiteratureVote::LiteratureId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(LiteratureVote::Table, LiteratureVote::LiteratureId)
                            .to(Literature::Table, Literature::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ArtVote::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ArtVote::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ArtVote::Handle).string().not_null())
                    .col(ColumnDef::new(ArtVote::Instance).string().not_null())
                    .col(ColumnDef::new(ArtVote::ArtId).integer().not_null())
                    .index(
                        Index::create()
                            .unique()
                            .col(ArtVote::Handle)
                            .col(ArtVote::Instance)
                            .col(ArtVote::ArtId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ArtVote::Table, ArtVote::ArtId)
                            .to(Art::Table, Art::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LiteratureVote::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ArtVote::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum LiteratureVote {
    Table,
    Id,
    Handle,
    Instance,
    LiteratureId,
}

#[derive(Iden)]
enum ArtVote {
    Table,
    Id,
    Handle,
    Instance,
    ArtId,
}
