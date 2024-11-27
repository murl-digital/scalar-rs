use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use scalar::{db::DocumentModifyError, validations::ValidationError, Document, Item, Schema, DB};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait ScalarState<D> {
    fn get_db(&self) -> &D;
}

pub struct ValidationFailiure(pub ValidationError);

impl IntoResponse for ValidationFailiure {
    fn into_response(self) -> axum::response::Response {
        Response::builder()
            .status(StatusCode::NOT_ACCEPTABLE)
            .body(Body::from(self.0.to_string()))
            .expect("nothing's being parsed here, da hell?")
    }
}

#[macro_export]
macro_rules! generate_routes {
    ($state:ty, $db:ty, $($doc:ty),+) => {
        {
            let mut router = ::axum::Router::new();
            ::scalar_axum::crud_routes!(router, $state, $db, $($doc),+);
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

            let mut validators = ::std::collections::HashMap::new();

            $(validators.extend(<$doc>::validators(::scalar::validations::DataModel::Json));)*

            for (key, validator) in validators {
                router = router.route(&format!("/validators/{key}/verify"), ::axum::routing::post(|body: String| async move {
                    validator(body).map_err(::scalar_axum::ValidationFailiure)
                }))
            }

            router
        }
    };
}

#[macro_export]
macro_rules! crud_routes {
    ($router:ident, $state:ty, $db:ty, $doc:ty) => {
        let path = format!("/docs/{}", <$doc>::identifier());
        let drafts_path = format!("{path}/drafts/:id");
        $router = $router
            .route(&path, ::axum::routing::get(::scalar_axum::get_all_docs::<$doc, $state, $db>))
            .route(&format!("{path}/:id"), ::axum::routing::get(::scalar_axum::get_doc_by_id::<$doc, $state, $db>))
            .route(&drafts_path, ::axum::routing::put(::scalar_axum::update_draft::<$doc, $state, $db>))
            .route(&format!("{path}/schema"), ::axum::routing::get(::scalar_axum::get_schema::<$doc>));
    };

    ($router:ident, $state:ty, $db:ty, $doc:ty, $($docs:ty),+) => {
        ::scalar_axum::crud_routes!($router, $state, $db, $doc);
        ::scalar_axum::crud_routes!($router, $state, $db, $($docs),+);
    };
}

pub async fn get_schema<T: Document>() -> Json<Schema> {
    Json(T::schema())
}

pub async fn update_draft<
    T: Document + Serialize + DeserializeOwned + Send,
    S: ScalarState<D>,
    D: DB + Clone,
>(
    state: State<S>,
    Path(id): Path<String>,
    Json(data): Json<serde_json::Value>,
) -> Result<Json<Item<serde_json::Value>>, StatusCode> {
    Ok(Json(
        state
            .0
            .get_db()
            .draft::<T>(&id, data)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}

pub async fn get_all_docs<
    T: Document + Serialize + DeserializeOwned + Send,
    S: ScalarState<D>,
    D: DB + Clone,
>(
    state: State<S>,
) -> Result<Json<Vec<Item<serde_json::Value>>>, StatusCode> {
    let db = state.get_db();

    let items = db
        .get_all::<T>()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(items))
}

pub async fn get_doc_by_id<
    T: Document + Serialize + DeserializeOwned + Send,
    S: ScalarState<D>,
    D: DB + Clone,
>(
    state: State<S>,
    id: Path<String>,
) -> Result<Json<Item<serde_json::Value>>, StatusCode> {
    let db = state.get_db();

    db.get_by_id::<T>(id.as_str())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
