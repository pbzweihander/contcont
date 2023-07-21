//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "art_vote")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub handle: String,
    pub instance: String,
    pub art_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::art::Entity",
        from = "Column::ArtId",
        to = "super::art::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Art,
}

impl Related<super::art::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Art.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
