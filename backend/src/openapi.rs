use rocket_okapi::swagger_ui::SwaggerUIConfig;

pub fn get_docs() -> SwaggerUIConfig {
    #[allow(unused_imports)]
    use rocket_okapi::settings::UrlObject;

    SwaggerUIConfig {
        url: "/api/v1/openapi.json".to_string(),
        ..Default::default()
    }
}