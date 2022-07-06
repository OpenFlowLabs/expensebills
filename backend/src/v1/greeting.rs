use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_okapi::{openapi, JsonSchema};

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct Greeting<'r> {
    name: &'r str,
    greeting: String,
}

#[openapi]
#[get("/<name>")]
pub fn hello(name: &str) -> Json<Greeting> {
    let greeting = format!("Hello {}!", name);
    Json(Greeting {
        name,
        greeting,
    })
}