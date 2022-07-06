mod v1;
mod cors;
mod openapi;
mod ulid_wrap;

#[macro_use]
extern crate rocket;
use rocket_okapi::{swagger_ui::make_swagger_ui, openapi_get_routes};
use rocket::{routes};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(cors::CORS)
        .mount("/api/v1/greeting", openapi_get_routes![v1::greeting::hello])
        .mount("/api/v1/receipts", routes![v1::receipts::upload_receipt])
        .mount("/docs/v1", make_swagger_ui(&openapi::get_docs()))
}
