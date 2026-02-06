use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa::Modify;
use utoipa_swagger_ui::SwaggerUi;

mod auth;
mod students;
mod swagger;
mod openapi;
mod openapi_security;

use openapi::ApiDoc;
use openapi_security::SecurityAddon;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut openapi = ApiDoc::openapi();
    SecurityAddon.modify(&mut openapi);

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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

