use axum::{
    extract::{FromRef, Path, Request, State},
    http::{self, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, Json, Router,
};
use scalar::{
    db::{Credentials, DatabaseFactory, User},
    validations::{Valid, ValidationError},
    DatabaseConnection, Document, Item, Schema,
};
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "img")]
pub mod img;

pub struct ValidationFailiure(pub ValidationError);

impl IntoResponse for ValidationFailiure {
    fn into_response(self) -> axum::response::Response {
        let mut response = Json(self.0).into_response();
        *response.status_mut() = StatusCode::NOT_ACCEPTABLE;
        response
    }
}

#[cfg(feature = "img")]
#[doc(hidden)]
pub fn add_image_routes__<S: Clone + Send + Sync + 'static>(router: Router<S>) -> Router<S>
where
    scalar_img::WrappedBucket: FromRef<S>,
{
    use axum::extract::DefaultBodyLimit;
    use img::{list, upload_file, upload_image};

    let merge = Router::new()
        .route(
            "/images/upload",
            axum::routing::put(upload_image).layer(DefaultBodyLimit::max(25_000_000)),
        )
        .route(
            "/files/upload",
            axum::routing::put(upload_file).layer(DefaultBodyLimit::disable()),
        )
        .route("/images/list", axum::routing::get(list));

    router.merge(merge)
}

#[cfg(not(feature = "img"))]
#[doc(hidden)]
pub fn add_image_routes__<S: Clone + Send + Sync + 'static>(router: Router<S>) -> Router<S> {
    router
}

#[macro_export]
#[doc(hidden)]
macro_rules! crud_routes__ {
    ($router:ident, $db:ty, $doc:ty) => {
        let path = format!("/docs/{}", <$doc>::identifier());
        let drafts_path = format!("{path}/drafts/{{id}}");
        $router = $router
            .route(&path, ::axum::routing::get(::scalar_axum::get_all_docs::<$doc, $db>))
            .route(&format!("{path}/{{id}}"), ::axum::routing::get(::scalar_axum::get_doc_by_id::<$doc, $db>))
            .route(&drafts_path, ::axum::routing::put(::scalar_axum::update_draft::<$doc, $db>))
            .route(&format!("{path}/schema"), ::axum::routing::get(::scalar_axum::get_schema::<$doc>));
    };

    ($router:ident, $db:ty, $($doc:ty),+) => {
        $(::scalar_axum::crud_routes__!($router, $db, $doc);)*
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! publish_routes__ {
    ($router:ident, $db:ty, $doc:ty) => {
        let path = format!("/docs/{}", <$doc>::identifier());
        $router = $router
            .route(&format!("{path}/{{id}}/publish"), ::axum::routing::post(::scalar_axum::publish_doc::<$doc, $db>));
    };

    ($router:ident, $db:ty, $($doc:ty),+) => {
        $(::scalar_axum::publish_routes__!($router, $db, $doc);)*
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

#[macro_export]
macro_rules! generate_routes {
    ({$app_state:ty}, $db_instance:ident: $db:ty, [$($doc:ty),+]) => {
        {
            let mut router = ::axum::Router::<$app_state>::new();
            ::scalar_axum::crud_routes__!(router, $db, $($doc),+);
            ::scalar_axum::publish_routes__!(router, $db, $($doc),+);
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
            router = ::scalar_axum::add_image_routes__(router);
            router = router.layer(::axum::middleware::from_fn_with_state($db_instance.clone(), ::scalar_axum::authenticated_connection_middleware::<$db>));
            router = router.route("/signin", ::axum::routing::post(::scalar_axum::signin::<$db>));

            ::scalar_axum::validate_routes__!(router, $($doc),+);

            router
        }
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
        .map(|header| {
            header
                .to_str()
                .map(str::trim)
                .map_err(|_| StatusCode::BAD_REQUEST)
        })
        .ok_or(StatusCode::UNAUTHORIZED)??;

    let connection = db_factory.init().await.map_err(|e| {
        println!("{e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let (_, token) = auth_header
        .starts_with("Bearer ")
        .then(|| {
            auth_header
                .split_at_checked(7)
                .ok_or(StatusCode::UNAUTHORIZED)
        })
        .ok_or(StatusCode::UNAUTHORIZED)??;

    connection.authenticate(token).await.map_err(|e| match e {
        scalar::db::AuthenticationError::BadToken => StatusCode::UNAUTHORIZED,
        scalar::db::AuthenticationError::BadCredentials => StatusCode::UNAUTHORIZED,
        scalar::db::AuthenticationError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

pub async fn publish_doc<
    D: Document + Serialize + DeserializeOwned + Send + 'static,
    F: DatabaseFactory,
>(
    Path(id): Path<String>,
    state: Extension<<F as DatabaseFactory>::Connection>,
    doc: Json<D>,
) -> Result<(), StatusCode> {
    state
        .publish(
            &id,
            None,
            Valid::new(doc.0).map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
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
