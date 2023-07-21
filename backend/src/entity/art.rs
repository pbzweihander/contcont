//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "art")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub data: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub thumbnail_data: Vec<u8>,
    pub author_handle: String,
    pub author_instance: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::art_vote::Entity")]
    ArtVote,
}

impl Related<super::art_vote::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ArtVote.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}