use axum::{
    extract::DefaultBodyLimit,
    routing::{get, put},
    Router,
};
use axum_macros::FromRef;
use rgb::{RGB8, RGBA8};
use s3::{creds::Credentials, Bucket};
use scalar::{
    db::DatabaseFactory,
    doc_enum,
    validations::{Validate, ValidationError},
    DateTime, Document, EditorField, Markdown, MultiLine, Utc,
};
use scalar_axum::generate_routes;
use scalar_img::{
    axum::{list, upload_file, upload_image},
    ImageData, WrappedBucket,
};
use scalar_surreal::{init, SurrealStore};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::{auth::Root, Config},
};
use tower_http::cors::CorsLayer;

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
async fn main() {
    let mut bucket = Bucket::new(
        "dev",
        s3::Region::Custom {
            region: "".into(),
            endpoint: "http://192.168.0.121:9000".into(),
        },
        Credentials {
            access_key: Some("4NPWPU3t08ulF8jOXQsm".into()),
            secret_key: Some("E0qqX4jUeBP5wA4dUMU9ctOYRfLbdhuhqiRYLD5S".into()),
            security_token: None,
            session_token: None,
            expiration: None,
        },
    )
    .unwrap();
    bucket.set_path_style();

    let wrapped_bucket = WrappedBucket::new(*bucket, None::<String>).await.unwrap();

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
    // conn.query(
    //     "CREATE IF NOT EXISTS sc__editor:admin CONTENT {
    //     name: 'drac',
    //     email: 'contact@draconium.productions',
    //     password: crypto::argon2::generate('password'),
    //     admin: true
    // }",
    // )
    // .await
    // .unwrap();
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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!(
        "{}",
        serde_json::to_string_pretty(&scalar::Utc::now()).unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
