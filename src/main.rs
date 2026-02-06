use actix_web::{App, HttpServer};
use utoipa::Modify;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
mod auth;
mod students;
mod swagger;
mod openapi;
mod openapi_security;

use openapi::ApiDoc;
use openapi_security::SecurityAddon;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // ⭐ Render PORT
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be number");

    let mut openapi = ApiDoc::openapi();
    SecurityAddon.modify(&mut openapi);

    println!("Server running on 0.0.0.0:{}", port);

    HttpServer::new(move || {
        App::new()
            .service(auth::handler::login)
            .service(auth::handler::me)
            .service(students::handler::list_students)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", openapi.clone()),
            )
    })
    .bind(("0.0.0.0", port))?   // ⭐ 必须
    .run()
    .await
}

