use ulid::Ulid as OrigUlid;
use rocket::request::FromParam;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::{JsonSchema};
use thiserror::Error;
use schemars::schema::*;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct Ulid(OrigUlid);

#[derive(Error, Debug, Clone)]
pub enum UlidError {
    #[error("could not parse Ulid: {0}")]
    ParseError(#[from] ulid::DecodeError)
}

impl<'a> FromParam<'a> for Ulid {
    type Error = UlidError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Ok(Ulid(ulid::Ulid::from_string(param).map_err(UlidError::ParseError)?))
    }
}

impl JsonSchema for Ulid {

    fn schema_name() -> String {
        "Ulid".to_owned()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("ulid".to_string()),
            ..Default::default()
        }
        .into()
    }
}

impl Ulid {
    pub fn new() -> Self {
        Self(OrigUlid::new())
    }
}