///src/openapi.rs
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::auth::handler::login,
        crate::auth::handler::me,
        crate::students::handler::list_students,
    ),
    components(
        schemas(
            crate::auth::handler::LoginReq,
            crate::auth::handler::LoginResp,
            crate::auth::extractor::AuthUser,
            crate::students::handler::Student,
        )
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub struct ApiDoc;

