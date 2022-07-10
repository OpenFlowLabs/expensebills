use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use chrono::NaiveDate;
use uuid::Uuid;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "receipts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub state: ReceiptState,
    pub file_hash: String,
    pub category: Option<String>,
    pub payment_date: Option<NaiveDate>
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, EnumIter, DeriveActiveEnum)]
#[serde(crate = "rocket::serde")]
#[sea_orm(rs_type="String", db_type="Enum", enum_name="receipt_state")]
pub enum ReceiptState {
    #[sea_orm(string_value = "Inbox")]
    Inbox,
    #[sea_orm(string_value = "Valid")]
    Valid,
    #[sea_orm(string_value = "Payed")]
    Payed,
    #[sea_orm(string_value = "Declined")]
    Declined,
    #[sea_orm(string_value = "Process")]
    Process,
    #[sea_orm(string_value = "Done")]
    Done
}

impl std::fmt::Display for ReceiptState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReceiptState::Inbox => write!(f, "inbox"),
            ReceiptState::Valid => write!(f, "valid"),
            ReceiptState::Payed => write!(f, "payed"),
            ReceiptState::Declined => write!(f, "declined"),
            ReceiptState::Process => write!(f, "process"),
            ReceiptState::Done => write!(f, "done"),
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("could not parse {0} as Receipt state accepted are inbox, valid, payed, declined, process and done")]
    ReceiptState(String),
}

impl <'a> rocket::request::FromParam<'a> for ReceiptState {
    type Error = ParseError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match param {
            "inbox" => Ok(Self::Inbox),
            "valid" => Ok(Self::Valid),
            "payed" => Ok(Self::Valid),
            "declined" => Ok(Self::Valid),
            "process" => Ok(Self::Valid),
            "done" => Ok(Self::Valid),
            x => Err(ParseError::ReceiptState(x.to_string())),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::recipient::Entity")]
    Recipient,
}

impl Related<super::recipient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recipient.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}