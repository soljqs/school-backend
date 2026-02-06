#[macro_export]
macro_rules! api_get {
    (
        path = $path:literal,
        tag = $tag:literal,
        resp = $resp:ty
        $(,
            params = ( $( $params:tt )* )
        )?
        =>
        $func:item
    ) => {
        #[utoipa::path(
            get,
            path = $path,
            tag = $tag,
            responses(
                (status = 200, body = $resp)
            ),
            security(
                ("bearerAuth" = [])
            )
            $(,
                params( $( $params )* )
            )?
        )]
        $func
    };
}


