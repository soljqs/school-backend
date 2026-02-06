///src/scheme.rs
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

impl ApiDoc {
    pub fn security_scheme() -> (&'static str, SecurityScheme) {
        (
            "bearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build()
            ),
        )
    }
}

