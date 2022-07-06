use log::info;
use rocket::form::{Form, Strict};
use rocket::fs::{TempFile, NamedFile};
use rocket::request::FromParam;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::status::Custom;
use rocket_okapi::{openapi, JsonSchema};
use crate::ulid_wrap::Ulid;
use thiserror::Error;
use chrono::NaiveDate;

#[derive(FromForm)]
pub struct ReceiptUploadRequest<'r> {
    name: &'r str,
    file: TempFile<'r>,
}

#[derive(Deserialize, Serialize, JsonSchema, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct Receipt {
    id: Ulid,
    name: String,
    state: ReceiptState,
    recipient: Option<Recipient>,
    category: Option<String>,
    payment_date: Option<NaiveDate>
}

#[derive(Error, Debug, Clone)]
pub enum ReceiptError {
    #[error("could not parse {0} as Receipt state accepted are inbox, valid, payed, declined, process and done")]
    ReceiptStateParseError(String),
    #[error("could not parse {0} as receipt action")]
    ReceiptActionParseError(String),
}

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
#[serde(crate = "rocket::serde")]
pub enum ReceiptState {
    Inbox,
    Valid,
    Payed,
    Declined,
    Process,
    Done
}

impl<'a> FromParam<'a> for ReceiptState {
    type Error = ReceiptError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match param {
            "inbox" => Ok(ReceiptState::Inbox),
            "valid" => Ok(ReceiptState::Valid),
            "payed" => Ok(ReceiptState::Payed),
            "declined" => Ok(ReceiptState::Declined),
            "process" => Ok(ReceiptState::Process),
            "done" => Ok(ReceiptState::Done),
            x => Err(ReceiptError::ReceiptStateParseError(x.to_owned()))
        }
    }
}

impl Default for ReceiptState {
    fn default() -> Self {
        Self::Inbox
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct Recipient {
    id: Ulid,
    name: String,
    iban: String,
    address_line1: String,
    address_line2: String,
    address_line3: String,
    address_line4: String,
}

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
#[serde(crate = "rocket::serde")]
pub enum ReceiptAction {
    Accept,
    Decline,
    Pay,
    ConfirmProcessStep(String),
    SetRecipient(Recipient),
    SetCategory(String),
    SetPaymentDate(NaiveDate),
}

// Needs https://github.com/GREsau/schemars/issues/103
//#[openapi]
#[post("/upload", data = "<upload>")]
pub fn upload_receipt(upload: Form<Strict<ReceiptUploadRequest<'_>>>) -> Json<Receipt> { 
    info!("received file: {}", upload.name);
    // TODO send to S3
    Json(Receipt{ 
        id: Ulid::new(), 
        name: upload.name.to_owned(),
        ..Receipt::default()
    })
}

#[openapi]
#[get("/<state>")]
pub fn get_receipts(state: ReceiptState) -> Json<Vec<Receipt>> {
    Json(vec![
        Receipt::default()
    ])
}

#[openapi]
#[post("/<id>", data = "<action>")]
pub fn post_receipt(id: Ulid, action: Json<ReceiptAction>) -> Result<Json<Receipt>, Custom<String>> {
    Ok(Json(
        Receipt::default()
    ))
}

#[openapi]
#[get("/<id>")]
pub fn get_receipt(id: Ulid) -> Json<Receipt> {
    todo!()
}

#[openapi]
#[get("/<id>/download")]
pub fn get_receipt_file(id: Ulid) -> NamedFile {
    todo!()
}