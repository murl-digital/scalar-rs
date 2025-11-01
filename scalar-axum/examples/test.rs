use argon2::PasswordHasher;
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata, CoreResponseType},
    ClientId, ClientSecret, IssuerUrl, RedirectUrl,
};
use reqwest::ClientBuilder;
use std::env;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};
use axum::Router;
use axum_macros::FromRef;
use rgb::{RGB8, RGBA8};
use s3::{creds::Credentials, Bucket, Region};
use scalar_axum::{
    generate_routes,
    oidc::{CoreOidcState, OidcState},
};
use scalar_cms::{
    doc_enum,
    types::{Markdown, MultiLine, Toggle},
    validations::{ErroredField, Field, Validate, ValidationError},
    DateTime, Document, EditorField, NaiveDate, Utc,
};
use scalar_img::{ImageData, WrappedBucket};
use scalar_sqlx::{sqlite::Pool, ConnectionFactory};
use serde::{Deserialize, Serialize};
use sqlx::{query, Sqlite};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};
use url::Url;

#[allow(clippy::trivially_copy_pass_by_ref)]
// the document derive macro isn't smart enough to know when to pass a reference or the value.
fn int_test(int: &i32) -> Result<(), ValidationError> {
    (*int < 3)
        .then_some(())
        .ok_or(ValidationError::Single("int must be less than 3!".into()))
}

#[allow(clippy::trivially_copy_pass_by_ref)]
// the document derive macro isn't smart enough to know when to pass a reference or the value.
fn float_test(float: &f32) -> Result<(), ValidationError> {
    (float.sqrt().fract() == 0.0)
        .then_some(())
        .ok_or(ValidationError::Single(
            "sqrt of float must be an int!".into(),
        ))
}

fn string_test(string: impl AsRef<str>) -> Result<(), ValidationError> {
    (string.as_ref().len() >= 3)
        .then_some(())
        .ok_or(ValidationError::Single(
            "must be at least 3 characters long".into(),
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
    #[validate(with = string_test)]
    single_line: String,
    #[validate(skip)]
    multi_line: MultiLine,
    #[validate(skip)]
    markdown: Markdown,
    #[validate(skip)]
    array: Vec<String>,
    #[validate(skip)]
    toggle: Toggle<i32>,
    #[validate(skip)]
    date: NaiveDate,
    #[validate(skip)]
    date_time: DateTime<Utc>,
    #[validate(skip)]
    color: RGB8,
    #[validate(skip)]
    color_alpha: RGBA8,
    #[validate(skip)]
    image: ImageData<ImageInner>,
    enum_select: TestEnum,
    struct_test: StructTest,
}

#[derive(EditorField, Serialize, Deserialize)]
struct StructTest {
    info: String,
}

impl Validate for StructTest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.info.len() < 3 {
            Err(ValidationError::Composite(vec![ErroredField {
                field: Field("info".into()),
                error: ValidationError::Single("must be longer than 3 characters!".into()),
            }]))
        } else {
            Ok(())
        }
    }
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

#[derive(FromRef, Clone)]
struct AppState {
    pool: ConnectionFactory<Sqlite>,
    oidc_state: CoreOidcState,
    wrapped_bucket: WrappedBucket,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let http_client = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(env::var("OIDC_ISSUER_URL")?)?,
        &http_client,
    )
    .await?;
    // Create an OpenID Connect client by specifying the client ID, client secret, authorization URL
    // and token URL.
    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(env::var("OIDC_CLIENT_ID")?),
        Some(ClientSecret::new(env::var("OIDC_CLIENT_SECRET")?)),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(
        "http://localhost:3000/login/oidc".to_string(),
    )?);

    let region = Region::R2 {
        account_id: env::var("R2_ACCTID")?,
    };
    let credentials = Credentials::default()?;

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

    let state = AppState {
        pool: ConnectionFactory::try_new_random(pool).unwrap(),
        oidc_state: OidcState::new(client, http_client),
        wrapped_bucket,
    };

    let api_router = generate_routes!({
        db: ConnectionFactory<Sqlite>,
        oidc: {
            state: CoreOidcState,
            response_type: CoreResponseType,
            oidc_only: true
        }
    },
    [AllTypes, Test2]
    );

    let app = Router::new()
        .nest("/api", api_router)
        .with_state(state)
        .fallback_service(
            ServeDir::new("scalar-cp/build").fallback(ServeFile::new("scalar-cp/build/index.html")),
        )
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await?;
    Ok(())
}
