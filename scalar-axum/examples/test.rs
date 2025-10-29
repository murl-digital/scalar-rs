use argon2::PasswordHasher;
use openidconnect::{
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreErrorResponseType, CoreGenderClaim, CoreJsonWebKey,
        CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm, CoreResponseType,
        CoreRevocableToken, CoreRevocationErrorResponse, CoreTokenIntrospectionResponse,
        CoreTokenResponse, CoreUserInfoClaims,
    },
    AccessTokenHash, AuthorizationCode, CsrfToken, EmptyExtraTokenFields, EndpointMaybeSet,
    EndpointNotSet, EndpointSet, IdTokenFields, Nonce, OAuth2TokenResponse, PkceCodeChallenge,
    PkceCodeVerifier, RevocationErrorResponseType, Scope, StandardErrorResponse,
    StandardTokenIntrospectionResponse, StandardTokenResponse, TokenResponse,
};
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    Client, ClientId, ClientSecret, EmptyAdditionalClaims, IssuerUrl, RedirectUrl,
};
use reqwest::{ClientBuilder, StatusCode};
use scalar_cms::db::DatabaseFactory;
use scalar_cms::DatabaseConnection;
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::Mutex;
use tower::util::error::optional::None;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};
use axum::{
    extract::{Query, State},
    http::Response,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_macros::FromRef;
use rgb::{RGB8, RGBA8};
use s3::{creds::Credentials, Bucket, Region};
use scalar_axum::generate_routes;
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

pub fn int_test(int: &i32) -> Result<(), ValidationError> {
    (*int < 3)
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

type FunnyClient = Client<
    EmptyAdditionalClaims,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<CoreErrorResponseType>,
    CoreTokenResponse,
    CoreTokenIntrospectionResponse,
    CoreRevocableToken,
    CoreRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

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
        "http://localhost:5173/login/oidc".to_string(),
    )?);

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
        client: FunnyClient,
        http_client: reqwest::Client,
        active_auths: Arc<Mutex<HashMap<String, (PkceCodeVerifier, Nonce)>>>,
        wrapped_bucket: WrappedBucket,
    }

    let state = AppState {
        pool: ConnectionFactory::new(pool),
        active_auths: Default::default(),
        http_client,
        client,
        wrapped_bucket,
    };

    let api_router =
        generate_routes!({AppState}, factory: ConnectionFactory<Sqlite>, [AllTypes, Test2]);

    let app = Router::new()
        .nest("/api", api_router)
        .route("/api/login/oidc", get(begin_oidc_auth))
        .route("/api/login/oidc/complete", get(complete_oidc_auth))
        .with_state(state)
        .fallback_service(
            ServeDir::new("../scalar-cp/build")
                .fallback(ServeFile::new("../scalar-cp/build/index.html")),
        )
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await?;
    Ok(())
}

async fn begin_oidc_auth(
    State(client): State<FunnyClient>,
    State(active_auths): State<Arc<Mutex<HashMap<String, (PkceCodeVerifier, Nonce)>>>>,
) -> impl IntoResponse {
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorize_url, csrf_state, nonce) = client
        .authorize_url(
            openidconnect::AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scopes([
            Scope::new("openid".into()),
            Scope::new("profile".into()),
            Scope::new("email".into()),
        ])
        .set_pkce_challenge(challenge)
        .url();

    active_auths
        .lock()
        .await
        .insert(csrf_state.into_secret(), (verifier, nonce));

    Redirect::to(authorize_url.as_str())
}

#[derive(Deserialize)]
struct RedirectParams {
    code: String,
    state: String,
}

async fn complete_oidc_auth(
    State(client): State<FunnyClient>,
    State(http_client): State<reqwest::Client>,
    State(connection): State<ConnectionFactory<Sqlite>>,
    State(active_auths): State<Arc<Mutex<HashMap<String, (PkceCodeVerifier, Nonce)>>>>,
    Query(params): Query<RedirectParams>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let (verifier, nonce) = active_auths
        .lock()
        .await
        .remove(&params.state)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let exchange = client
        .exchange_code(AuthorizationCode::new(params.code))
        .expect("proper config")
        .set_pkce_verifier(verifier)
        .request_async(&http_client)
        .await
        .map_err(|e| match e {
            openidconnect::RequestTokenError::ServerResponse(e) => match e.error() {
                CoreErrorResponseType::InvalidClient
                | CoreErrorResponseType::InvalidGrant
                | CoreErrorResponseType::InvalidRequest
                | CoreErrorResponseType::InvalidScope => StatusCode::BAD_REQUEST,
                CoreErrorResponseType::UnauthorizedClient
                | CoreErrorResponseType::UnsupportedGrantType => StatusCode::UNAUTHORIZED,
                CoreErrorResponseType::Extension(_) => todo!(),
            },
            openidconnect::RequestTokenError::Request(_) => StatusCode::INTERNAL_SERVER_ERROR,
            openidconnect::RequestTokenError::Parse(error, items) => StatusCode::BAD_REQUEST,
            openidconnect::RequestTokenError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let id_token = exchange.id_token().unwrap();
    let id_token_verifier = client.id_token_verifier();
    let claims = id_token.claims(&id_token_verifier, &nonce).unwrap();

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            exchange.access_token(),
            id_token.signing_alg().unwrap(),
            id_token.signing_key(&id_token_verifier).unwrap(),
        )
        .unwrap();
        if actual_access_token_hash != *expected_access_token_hash {
            panic!("invalid access token");
        }
    }

    println!("{:#?}", claims);

    // The user_info request uses the AccessToken returned in the token response. To parse custom
    // claims, use UserInfoClaims directly (with the desired type parameters) rather than using the
    // CoreUserInfoClaims type alias.
    let userinfo: CoreUserInfoClaims = client
        .user_info(exchange.access_token().to_owned(), None)
        .unwrap()
        .request_async(&http_client)
        .await
        .unwrap();

    println!("{:#?}", userinfo);

    Ok(connection
        .init()
        .await
        .unwrap()
        .signin_oidc(claims.to_owned())
        .await
        .unwrap())
}
