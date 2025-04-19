use std::env;

use axum::Router;
use axum_macros::FromRef;
use rgb::{RGB8, RGBA8};
use s3::{creds::Credentials, Bucket, Region};
use scalar::{
    db::DatabaseFactory,
    doc_enum,
    validations::{Validate, ValidationError},
    DateTime, Document, EditorField, Markdown, MultiLine, Utc,
};
use scalar_axum::generate_routes;
use scalar_img::{ImageData, WrappedBucket};
use scalar_surreal::{init, SurrealStore};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::local::{Db, RocksDb},
    opt::{auth::Root, Config},
};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};
use url::Url;

#[derive(Document, Serialize, Deserialize)]
struct AllTypes {
    #[validate(skip)]
    bool: bool,
    #[validate(skip)]
    integer: i32,
    #[validate(skip)]
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
    fn validate(&self) -> Result<(), scalar::validations::ValidationError> {
        match self {
            TestEnum::Struct { eeee } if eeee.is_empty() => Err(ValidationError::Single(
                "eeee must have something in it".into(),
            )),
            _ => Ok(()),
        }
    }
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

    let factory = SurrealStore::new::<RocksDb, _>(
        (
            "../db/hi.db",
            Config::new().user(Root {
                username: "root",
                password: "root",
            }),
        ),
        "test".into(),
        "test".into(),
    )
    .await
    .unwrap();
    let conn = factory.init_system().await.unwrap();
    init!(conn, AllTypes, Test2);
    conn.query(
        "UPSERT sc__editor CONTENT {
        name: 'drac 2',
        email: 'joseph.md.sorensen@gmail.com',
        password: crypto::argon2::generate('password'),
        admin: true
    }",
    )
    .await
    .unwrap();
    drop(conn);

    #[derive(FromRef, Clone)]
    struct AppState {
        factory: SurrealStore<Db>,
        wrapped_bucket: WrappedBucket,
    }

    let api_router = generate_routes!({AppState}, factory: SurrealStore<Db>, [AllTypes, Test2])
        .with_state(AppState {
            factory,
            wrapped_bucket,
        });

    let app = Router::new()
        .nest("/api", api_router)
        .fallback_service(
            ServeDir::new("../scalar-cp/build")
                .not_found_service(ServeFile::new("../scalar-cp/build/index.html")),
        )
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await?;
    Ok(())
}
