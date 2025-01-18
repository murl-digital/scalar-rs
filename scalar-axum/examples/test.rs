use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, State},
    http::StatusCode,
    routing::{get, put},
    Json,
};
use axum_macros::FromRef;
use rgb::{RGB8, RGBA8};
use sc_minio::{provider::StaticProvider, Minio};
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
    let client = Minio::builder()
        .endpoint("192.168.0.121:9000")
        .provider(StaticProvider::new(
            "4NPWPU3t08ulF8jOXQsm",
            "E0qqX4jUeBP5wA4dUMU9ctOYRfLbdhuhqiRYLD5S",
            None,
        ))
        .secure(false)
        .build()
        .unwrap();

    let wrapped_bucket = WrappedBucket::new(client.bucket("dev"), None::<String>)
        .await
        .unwrap();

    let factory = SurrealStore::<Client, Ws, _>::new(
        (
            "localhost:8000",
            Config::new().user(Root {
                username: "root",
                password: "root",
            }),
        ),
        "test".into(),
        "test".into(),
    );
    let conn = factory.init_system().await.unwrap();
    init!(conn, AllTypes, Test2);
    conn.query(
        "CREATE IF NOT EXISTS sc__editor:admin CONTENT {
        name: 'drac',
        email: 'contact@draconium.productions',
        password: crypto::argon2::generate('password'),
        admin: true
    }",
    )
    .await
    .unwrap();
    drop(conn);

    #[derive(FromRef, Clone)]
    struct AppState {
        factory: SurrealStore<Client, Ws, (&'static str, Config)>,
        wrapped_bucket: WrappedBucket,
    }

    let app = generate_routes!(factory, SurrealStore<Client, Ws, (&str, Config)>, AllTypes, Test2)
        .route(
            "/images/upload",
            put(upload).layer(DefaultBodyLimit::disable()),
        )
        .route("/images/list", get(list))
        .layer(CorsLayer::very_permissive())
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

async fn upload(State(client): State<WrappedBucket>, bytes: Bytes) -> Result<String, StatusCode> {
    client
        .upload(bytes.as_ref().into())
        .await
        .map_err(|e| match e {
            scalar_img::UploadError::MalformedImage => StatusCode::UNPROCESSABLE_ENTITY,
            scalar_img::UploadError::Client(error) => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

async fn list(State(client): State<WrappedBucket>) -> Result<Json<Vec<String>>, StatusCode> {
    Ok(Json(
        client
            .list()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}
