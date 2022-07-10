use log::info;
use rocket::form::{Form, Strict};
use rocket::fs::{TempFile};
use rocket::data::Capped;
use rocket::http::ContentType;
use rocket::{http::Status, response::Responder};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{State, Config};
use sea_orm::{Set, ColumnTrait, EntityTrait, QueryFilter, ModelTrait};
use crate::SledDB;
use thiserror::Error;
use chrono::NaiveDate;
use std::io::Read;
use anyhow::anyhow;
use entity::receipt::{self, Model as Receipt, ReceiptState};
use entity::recipient::{self, Model as Recipient};
use rocket::serde::uuid::Uuid;
use sea_orm::ActiveModelTrait;
use sea_orm_rocket::{Connection};
use crate::SQLDb;


#[derive(FromForm)]
pub struct ReceiptUploadRequest<'r> {
    name: &'r str,
    file: Capped<TempFile<'r>>,
}

type EndpointResult<T> = Result<T, ReceiptError>;

#[derive(Error, Debug)]
pub enum ReceiptError {
    #[error("sled db error")]
    Sled(#[from] anyhow::Error),
    #[error("io error")]
    IO(#[from] std::io::Error),
    #[error("error from SQL Database")]
    Sql(#[from] sea_orm::DbErr),
    #[error("no receipt found")]
    NotFound,
    #[error("uuid conversion error")]
    Uuid(#[from] uuid::Error),
}

impl<'r> Responder<'r, 'static> for ReceiptError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            ReceiptError::Sled(_) => Err(Status::InternalServerError),
            ReceiptError::IO(_) => Err(Status::InternalServerError),
            ReceiptError::Sql(_) => Err(Status::InternalServerError),
            ReceiptError::NotFound => Err(Status::NotFound),
            ReceiptError::Uuid(_) => Err(Status::BadRequest),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
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

fn uuid_conversion(uuid: Uuid) -> Result<uuid::Uuid, uuid::Error> {
    let s = uuid.hyphenated().to_string();
    uuid::Uuid::parse_str(&s)
}

fn sled_to_anyhow<E: std::fmt::Display>(err: E) -> anyhow::Error {
    anyhow!("{}", err)
}

// Needs https://github.com/GREsau/schemars/issues/103
//#[openapi]
#[post("/upload", data = "<upload>")]
pub async fn upload_receipt(config: &State<Config>, conn: Connection<'_, SQLDb>, db: &State<SledDB>, mut upload: Form<Strict<ReceiptUploadRequest<'_>>>) -> EndpointResult<Json<Receipt>> { 
    info!("received file: {}", upload.name);
    let hash = {
        let file_temp_id = uuid::Uuid::new_v4().to_hyphenated().to_string();
        let tmp_file = config.temp_dir.relative().join(file_temp_id);
        upload.file.persist_to(&tmp_file).await?;
        let mut file = std::fs::File::open(&tmp_file)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        let hash = sha256::digest_bytes(&content);
        db.files_db.insert(hash.as_bytes(), content).map_err(sled_to_anyhow)?;
        drop(file);
        std::fs::remove_file(&tmp_file)?;
        hash
    };

    let receipt = receipt::ActiveModel{
        name: Set(upload.name.to_owned()),
        state: Set(receipt::ReceiptState::Inbox),
        file_hash: Set(hash),
        ..Default::default()
    };
    let sql_db = conn.into_inner();

    let receipt: Receipt = receipt.insert(sql_db).await?;

    Ok(Json(receipt))
}

#[get("/box/<state>")]
pub async fn get_receipts(conn: Connection<'_, SQLDb>, state: ReceiptState) -> EndpointResult<Json<Vec<Receipt>>> {
    let sql_db = conn.into_inner();

    let receipts: Vec<Receipt> = receipt::Entity::find()
        .filter(receipt::Column::State.eq(state))
        .all(sql_db)
        .await?;
    Ok(Json(receipts))
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum ActionAnswer {
    #[serde(rename="data")]
    Receipt(Receipt),
    #[serde(rename="error")]
    Error(String),
    #[serde(rename="data")]
    ReceiptAndRecipient((Receipt, Recipient))
}

#[post("/<id>", data = "<action>")]
pub async fn post_receipt(conn: Connection<'_, SQLDb>, id: Uuid, action: Json<ReceiptAction>) -> EndpointResult<Json<ActionAnswer>> {
    let sql_db = conn.into_inner();

    let receipt = receipt::Entity::find_by_id(uuid_conversion(id)?).one(sql_db).await?;
    if let Some(model) = receipt {
        match action.0 {
            ReceiptAction::Accept => {
                if model.state == receipt::ReceiptState::Inbox {
                    let mut update_receipt: receipt::ActiveModel = model.into();
                    update_receipt.state = Set(ReceiptState::Valid);
                    let receipt: Receipt = update_receipt.update(sql_db).await?;
                    Ok(Json(ActionAnswer::Receipt(receipt)))
                } else {
                    Ok(Json(ActionAnswer::Error(format!("cannot accept receipt {} in state {}", model.name, model.state))))
                }
            },
            ReceiptAction::Decline => {
                if model.state == receipt::ReceiptState::Inbox {
                    let mut update_receipt: receipt::ActiveModel = model.into();
                    update_receipt.state = Set(ReceiptState::Declined);
                    let receipt: Receipt = update_receipt.update(sql_db).await?;
                    Ok(Json(ActionAnswer::Receipt(receipt)))
                } else {
                    Ok(Json(ActionAnswer::Error(format!("cannot decline receipt {} in state {}", model.name, model.state))))
                }
            },
            ReceiptAction::Pay => {
                if model.payment_date.is_some() {
                    let mut update_receipt: receipt::ActiveModel = model.into();
                    update_receipt.state = Set(ReceiptState::Payed);
                    let receipt = update_receipt.update(sql_db).await?;
                    Ok(Json(ActionAnswer::Receipt(receipt)))
                } else {
                    Ok(Json(ActionAnswer::Error(format!("payment dat not set {}", model.name))))
                }
            },
            ReceiptAction::ConfirmProcessStep(_) => todo!(),
            ReceiptAction::SetRecipient(form_recipient) => {
                let db_recipient = model.find_related(recipient::Entity).one(sql_db).await?;
                let mut update_recipient: recipient::ActiveModel = form_recipient.into();

                if let Some(db_model) = db_recipient {
                    update_recipient.id = Set(db_model.id);
                } else {
                    update_recipient.receipt_id = Set(model.id);
                }

                let recipient = update_recipient.update(sql_db).await?;

                Ok(Json(ActionAnswer::ReceiptAndRecipient((model, recipient))))
            },
            ReceiptAction::SetCategory(cat) => {
                let mut update_receipt: receipt::ActiveModel = model.into();
                update_receipt.category = Set(Some(cat));
                let receipt: Receipt = update_receipt.update(sql_db).await?;
                Ok(Json(ActionAnswer::Receipt(receipt)))
            },
            ReceiptAction::SetPaymentDate(date) => {
                let mut update_receipt: receipt::ActiveModel = model.into();
                update_receipt.payment_date = Set(Some(date));
                let receipt: Receipt = update_receipt.update(sql_db).await?;
                Ok(Json(ActionAnswer::Receipt(receipt)))
            },
        }
    } else {
        Err(ReceiptError::NotFound)
    }
}

#[get("/<id>")]
pub async fn get_receipt(conn: Connection<'_, SQLDb>, id: Uuid) -> EndpointResult<Json<(Receipt, Option<Recipient>)>> {    
    let sql_db = conn.into_inner();

    let receipt = receipt::Entity::find_by_id(uuid_conversion(id)?).one(sql_db).await?;

    if let Some(receipt) = receipt {
        let recipient = receipt.find_related(recipient::Entity).one(sql_db).await?;
        Ok(Json((receipt, recipient)))
    } else {
        Err(ReceiptError::NotFound)
    }
}

#[get("/download/<id>")]
pub async fn get_receipt_file(conn: Connection<'_, SQLDb>, db: &State<SledDB>, id: Uuid) -> EndpointResult<(ContentType, Vec<u8>)> {
    let sql_db = conn.into_inner();

    let receipt = receipt::Entity::find_by_id(uuid_conversion(id)?).one(sql_db).await?;

    if let Some(receipt) = receipt {
        let val = db.files_db.get(receipt.file_hash.as_bytes()).map_err(sled_to_anyhow)?;
        if let Some(file) = val {
            Ok((ContentType::Binary ,file.to_vec()))
        } else {
            Err(ReceiptError::NotFound)
        }
    } else {
        Err(ReceiptError::NotFound)
    }
}