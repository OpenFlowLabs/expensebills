mod v1;
mod cors;
mod pool;

#[macro_use]
extern crate rocket;
//use rocket_okapi::{swagger_ui::make_swagger_ui, openapi_get_routes};
use rocket::{routes};
use rocket::fairing::AdHoc;
use rocket::Config;
use sled_extensions::Db;
use sea_orm_rocket::{Database};
use pool::SQLDb;

pub struct SledDB {
    pub files_db: Db,
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();

    let config: Config = figment.extract().expect("config");
    let path = config.temp_dir.relative().parent().unwrap().join("files"); 

    let db = sled_extensions::Config::default()
        .path(&path)
        .open()
        .expect("Failed to open data path");

    
    rocket.attach(SQLDb::init())
        .attach(AdHoc::config::<Config>())
        .attach(cors::Cors)
        .manage(SledDB{
            files_db: db,
        })
        .mount("/api/v1/greeting", routes![v1::greeting::hello])
        .mount("/api/v1/receipts", v1::receipt_routes())
        //.mount("/docs/v1", make_swagger_ui(&openapi::get_docs()))

}
