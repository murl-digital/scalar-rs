use argon2::PasswordHasher;
use std::{env, sync::Arc};
use tower::Service;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};
use axum::{
    extract::{FromRef, Request, State},
    http::{self, StatusCode},
    middleware::{from_fn_with_state, Next},
    response::Response,
    Router,
};
use axum_macros::{debug_middleware, FromRef};
use rgb::{RGB8, RGBA8};
use s3::{creds::Credentials, Bucket, Region};
use scalar_axum::generate_routes;
use scalar_cms::{
    db::{Authenticated, DatabaseFactory},
    doc_enum,
    validations::{Validate, ValidationError},
    DateTime, Document, EditorField, Markdown, MultiLine, Utc,
};
use scalar_img::{ImageData, WrappedBucket};
use scalar_sqlx::{sqlite::Pool, Connection, ConnectionFactory};
use serde::{Deserialize, Serialize};
use sqlx::{query, Sqlite, SqlitePool};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};
use url::Url;

pub fn int_test(int: &i32) -> Result<(), ValidationError> {
    (*int > 3)
        .then_some(())
        .ok_or(ValidationError::Single("int must be less than 3!".into()))
}

pub fn float_test(float: &f32) -> Result<(), ValidationError> {
    (float.sqrt().fract() == 0.0)
        .then_some(())
        .ok_or(ValidationError::Single(
            "sqrt of float must be an int!".into(),
        ))
}

#[derive(Document, Serialize, Deserialize)]
struct AllTypes {
    #[validate(skip)]
    bool: bool,
    #[validate(with = int_test)]
    integer: i32,
    #[validate(with = float_test)]
    float: f32,
    #[validate(skip)]
    single_line: String,
    #[validate(skip)]
    multi_line: MultiLine,
    #[validate(skip)]
    markdown: Markdown,
    #[validate(skip)]
    array: Vec<String>,
    #[validate(skip)]
    date_time: DateTime<Utc>,
    #[validate(skip)]
    color: RGB8,
    #[validate(skip)]
    color_alpha: RGBA8,
    #[validate(skip)]
    image: ImageData<ImageInner>,
    enum_select: TestEnum,
}

#[derive(EditorField, Serialize, Deserialize)]
struct ImageInner {
    info: String,
}

#[derive(Document, Serialize, Deserialize, Clone)]
struct Test2 {
    #[validate(skip)]
    pub hello: String,
}

#[doc_enum]
#[derive(Clone)]
enum TestEnum {
    Unit,
    Struct { eeee: String },
}

impl Validate for TestEnum {
    fn validate(&self) -> Result<(), scalar_cms::validations::ValidationError> {
        match self {
            TestEnum::Struct { eeee } if eeee.is_empty() => Err(ValidationError::Single(
                "eeee must have something in it".into(),
            )),
            _ => Ok(()),
        }
    }
}

pub async fn authenticated_connection_middleware<F: DatabaseFactory + FromRef<S>, S>(
    state: S,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode>
where
    <F as scalar_cms::db::DatabaseFactory>::Connection: 'static,
{
    let db_factory = F::from_ref(&state);
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

    let connection = Authenticated::authenticate(connection, token)
        .await
        .map_err(|e| match e {
            scalar_cms::db::AuthenticationError::BadToken => StatusCode::UNAUTHORIZED,
            scalar_cms::db::AuthenticationError::BadCredentials => StatusCode::UNAUTHORIZED,
            scalar_cms::db::AuthenticationError::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    req.extensions_mut().insert(Arc::new(connection));

    Ok(next.run(req).await)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let region = Region::R2 {
        account_id: env::var("R2_ACCTID")?,
    };
    let credentials = Credentials::default()?;
    println!("{credentials:?}");

    let bucket = Bucket::new("drc-prd-test", region, credentials)?.with_path_style();

    let wrapped_bucket = WrappedBucket::new(*bucket).await?.with_public_url(
        Url::parse("https://super-secret-media-testing.draconium.music/").unwrap(),
    );

    let pool = Pool::connect_lazy("sqlite://scalar-sqlx.db").unwrap();
    scalar_sqlx::sqlite::migrate(&pool).await.unwrap();
    let password = Argon2::default()
        .hash_password(b"password", &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();
    query!(
        "INSERT OR IGNORE INTO sc__users(name, email, password_hash, admin) VALUES ('example user', 'example@example.com', $1, true)",
        password
    ).execute(&pool).await.unwrap();

    #[derive(FromRef, Clone)]
    struct AppState {
        pool: ConnectionFactory<Sqlite>,
        wrapped_bucket: WrappedBucket,
    }

    let state = AppState {
        pool: ConnectionFactory::new(pool),
        wrapped_bucket,
    };

    let api_router =
        generate_routes!({AppState}, factory: ConnectionFactory<Sqlite>, [AllTypes, Test2])
            .with_state(state);

    let app = Router::new()
        .nest("/api", api_router)
        .fallback_service(
            ServeDir::new("../scalar-cp/build")
                .fallback(ServeFile::new("../scalar-cp/build/index.html")),
        )
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await?;
    Ok(())
}
