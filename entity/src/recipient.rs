
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "recipients")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub receipt_id: Uuid,
    pub name: String,
    pub iban: String,
    pub address_line1: String,
    pub address_line2: String,
    pub address_line3: String,
    pub address_line4: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::receipt::Entity",
        from = "Column::ReceiptId",
        to = "crate::receipt::Column::Id"
    )]
    Receipt,
}

impl Related<super::receipt::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Receipt.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}