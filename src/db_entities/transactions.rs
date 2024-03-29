//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "transactions")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Binary(BlobSize::Blob(None))"
    )]
    pub id: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub tx: Option<Vec<u8>>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub header: Option<Vec<u8>>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub result: Option<Vec<u8>>,
    pub layer: Option<i32>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub block: Option<Vec<u8>>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub principal: Option<Vec<u8>>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub nonce: Option<Vec<u8>>,
    pub timestamp: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
