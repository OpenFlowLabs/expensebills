mod cors;
mod migrations;
mod pool;
mod v1;

#[macro_use]
extern crate rocket;
use log::{error, info};
//use rocket_okapi::{swagger_ui::make_swagger_ui, openapi_get_routes};
use migrations::Migrator;
use pool::SQLDb;
use rocket::fairing::{self, AdHoc};
use rocket::routes;
use rocket::Config;
use rocket::{Build, Rocket};
use sea_orm_migration::MigratorTrait;
use sea_orm_rocket::Database;
use sled_extensions::Db;

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    if let Some(db) = SQLDb::fetch(&rocket) {
        // run migrations using `db`. get the inner type with &db.0.
        match Migrator::up(&db.conn, None).await {
            Ok(_) => {
                info!("DB migrations suceeded");
                Ok(rocket)
            },
            Err(err) => {
                error!("DB migrations failed with: {}", err);
                Err(rocket)
            },
        }
    } else {
        error!("No database configuration found");
        Err(rocket)
    }
}

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

    rocket
        .attach(AdHoc::config::<Config>())
        .attach(SQLDb::init())
        .attach(AdHoc::try_on_ignite("DB Migrations", run_migrations))
        .attach(cors::Cors)
        .manage(SledDB {
            files_db: db,
        })
        .mount("/api/v1/greeting", routes![v1::greeting::hello])
        .mount("/api/v1/receipts", v1::receipt_routes())
    //.mount("/docs/v1", make_swagger_ui(&openapi::get_docs()))
}
