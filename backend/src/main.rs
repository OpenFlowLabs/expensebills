#[macro_use]
extern crate rocket;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Request, Response};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes, JsonSchema};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _request: &'r Request<'_>,
        response: &mut Response<'r>,
    ) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Credentials",
            "true",
        ));
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct Greeting<'r> {
    name: &'r str,
    greeting: String,
}

#[openapi]
#[get("/hello/<name>")]
fn hello(name: &str) -> Json<Greeting> {
    let greeting = format!("Hello {}!", name);
    Json(Greeting {
        name,
        greeting,
    })
}

fn get_docs() -> SwaggerUIConfig {
    #[allow(unused_imports)]
    use rocket_okapi::settings::UrlObject;

    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", openapi_get_routes![hello])
        .mount("/swagger", make_swagger_ui(&get_docs()))
}
