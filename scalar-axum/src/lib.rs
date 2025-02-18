use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::{self, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, Json,
};
use scalar::{
    db::{Credentials, DatabaseFactory, User},
    validations::ValidationError,
    DatabaseConnection, Document, Item, Schema,
};
use serde::{de::DeserializeOwned, Serialize};

pub struct ValidationFailiure(pub ValidationError);

impl IntoResponse for ValidationFailiure {
    fn into_response(self) -> axum::response::Response {
        let mut response = Json(self.0).into_response();
        *response.status_mut() = StatusCode::NOT_ACCEPTABLE;
        response
    }
}

#[macro_export]
macro_rules! generate_routes {
    ($db_instance:ident, $db:ty, $($doc:ty),+) => {
        {
            let mut router = ::axum::Router::new();
            ::scalar_axum::crud_routes__!(router, $db, $($doc),+);
            #[axum_macros::debug_handler]
            async fn get_docs() -> ::axum::Json<Vec<::scalar::DocInfo>> {
                ::axum::Json(vec![
                    $(::scalar::DocInfo {
                        identifier: <$doc>::identifier(),
                        title: <$doc>::title()
                    }),+
                ])
            }
            router = router.route("/docs", ::axum::routing::get(get_docs));

            router = router.route("/me", ::axum::routing::get(::scalar_axum::me::<$db>));
            router = router.layer(::axum::middleware::from_fn_with_state($db_instance.clone(), ::scalar_axum::authenticated_connection_middleware::<$db>));
            router = router.route("/signin", ::axum::routing::post(::scalar_axum::signin::<$db>));

            ::scalar_axum::validate_routes__!(router, $($doc),+);

            router
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! crud_routes__ {
    ($router:ident, $db:ty, $doc:ty) => {
        let path = format!("/docs/{}", <$doc>::identifier());
        let drafts_path = format!("{path}/drafts/:id");
        $router = $router
            .route(&path, ::axum::routing::get(::scalar_axum::get_all_docs::<$doc, $db>))
            .route(&format!("{path}/:id"), ::axum::routing::get(::scalar_axum::get_doc_by_id::<$doc, $db>))
            .route(&drafts_path, ::axum::routing::put(::scalar_axum::update_draft::<$doc, $db>))
            .route(&format!("{path}/schema"), ::axum::routing::get(::scalar_axum::get_schema::<$doc>));
    };

    ($router:ident, $db:ty, $($doc:ty),+) => {
        $(::scalar_axum::crud_routes__!($router, $db, $doc);)*
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! validate_routes__ {
    ($router:ident, $doc:ty) => {
        $router = $router
            .route(&format!("/docs/{}/validate", <$doc>::identifier()), ::axum::routing::post(::scalar_axum::validate::<$doc>));
    };

    ($router:ident, $($doc:ty),+) => {
        $(::scalar_axum::validate_routes__!($router, $doc);)*
    };
}

pub async fn authenticated_connection_middleware<F: DatabaseFactory + Clone>(
    State(db_factory): State<F>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode>
where
    <F as DatabaseFactory>::Connection: 'static,
{
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header.trim()
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let connection = db_factory.init().await.map_err(|e| {
        println!("{e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let token = auth_header
        .starts_with("Bearer ")
        .then(|| {
            auth_header
                .split_at_checked(7)
                .ok_or(StatusCode::UNAUTHORIZED)
        })
        .ok_or(StatusCode::UNAUTHORIZED)??
        .1;

    connection.authenticate(token).await.map_err(|e| match e {
        scalar::db::AuthenticationError::BadToken => StatusCode::UNAUTHORIZED,
        scalar::db::AuthenticationError::BadCredentials => StatusCode::UNAUTHORIZED,
        scalar::db::AuthenticationError::DatabaseError(e) => {
            println!("auth: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    })?;

    req.extensions_mut().insert(connection);

    Ok(next.run(req).await)
}

//#[axum_macros::debug_handler]
pub async fn signin<F: DatabaseFactory + Clone>(
    State(factory): State<F>,
    Json(credentials): Json<Credentials>,
) -> Result<String, StatusCode> {
    let connection = factory.init().await.map_err(|e| {
        println!("{e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    println!("connection");

    let token = connection.signin(credentials).await.map_err(|e| match e {
        scalar::db::AuthenticationError::BadToken => StatusCode::UNAUTHORIZED,
        scalar::db::AuthenticationError::BadCredentials => StatusCode::UNAUTHORIZED,
        scalar::db::AuthenticationError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(token)
}

pub async fn get_schema<T: Document>() -> Json<Schema> {
    Json(T::schema())
}

pub async fn validate<D: Document>(
    Json(doc): Json<D>,
) -> Result<(), (StatusCode, Json<ValidationError>)> {
    doc.validate()
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, Json(e)))
}

pub async fn me<F: DatabaseFactory>(
    state: Extension<<F as DatabaseFactory>::Connection>,
) -> Result<Json<User>, StatusCode> {
    Ok(Json(state.me().await.map_err(|e| {
        println!("{e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?))
}

pub async fn update_draft<T: Document + Serialize + DeserializeOwned + Send, F: DatabaseFactory>(
    state: Extension<<F as DatabaseFactory>::Connection>,
    Path(id): Path<String>,
    Json(data): Json<serde_json::Value>,
) -> Result<Json<Item<serde_json::Value>>, StatusCode> {
    Ok(Json(
        state
            .draft::<T>(&id, data)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}

pub async fn get_all_docs<T: Document + Serialize + DeserializeOwned + Send, F: DatabaseFactory>(
    state: Extension<<F as DatabaseFactory>::Connection>,
) -> Result<Json<Vec<Item<serde_json::Value>>>, StatusCode> {
    let items = state
        .get_all::<T>()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(items))
}

pub async fn get_doc_by_id<
    T: Document + Serialize + DeserializeOwned + Send,
    F: DatabaseFactory,
>(
    state: Extension<<F as DatabaseFactory>::Connection>,
    id: Path<String>,
) -> Result<Json<Item<serde_json::Value>>, StatusCode> {
    state
        .get_by_id::<T>(id.as_str())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
