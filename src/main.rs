use axum::{
    debug_handler,
    http::{HeaderMap, HeaderName, HeaderValue},
    routing::get,
    Router,
};
use axum_server::tls_openssl::OpenSSLConfig;
use mimalloc::MiMalloc;
use pages::{Article, Index, List};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::OnceLock,
    time::Instant,
};
use tokio::sync::Mutex;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing::{log::info, Level};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};
use types::BlogArticleContent;

mod pages;
mod types;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

static RENDERED_LIST: OnceLock<Mutex<(Instant, List)>> = OnceLock::new();
static RENDERED_PAGES: OnceLock<Mutex<HashMap<(String, String), BlogArticleContent>>> =
    OnceLock::new();

#[tokio::main]
async fn main() {
    let tracing_filter = filter::Targets::new().with_default(Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_filter)
        .init();

    let router = Router::new()
        .route("/", get(|| async { Index::list().await.get_html() }))
        .route(
            "/blog",
            get(|query| async { Index::article(query).await.get_html() }),
        )
        .route(
            "/article",
            get(|query| async { Article::from_query(query).await.get_html() }),
        )
        .route("/list", get(|| async { List::generate().await.get_html() }))
        .route("/style.css", get(get_style))
        .route("/script.js", get(get_script))
        .layer(CompressionLayer::new().no_gzip())
        .layer(TraceLayer::new_for_http())
        .into_make_service();

    if let Ok(ssl_config) = OpenSSLConfig::from_pem_file("ssl/ssl.pem", "ssl/ssl.key") {
        let addr = SocketAddr::from(([0, 0, 0, 0], 443));
        info!("SSL enabled. Listening on {}", addr);
        axum_server::bind_openssl(addr, ssl_config)
            .serve(router)
            .await
            .expect("Server startup failed.");
    } else {
        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        info!("SSL disabled. Listening on {}", addr);
        axum_server::bind(addr)
            .serve(router)
            .await
            .expect("Server startup failed.");
    }
}

#[debug_handler]
async fn get_style() -> (HeaderMap, String) {
    let mut header = HeaderMap::new();
    header.insert(
        HeaderName::from_lowercase(b"content-type").unwrap(),
        HeaderValue::from_str("text/css").unwrap(),
    );
    header.insert(
        HeaderName::from_lowercase(b"cache-control").unwrap(),
        HeaderValue::from_str("max-age=31536000").unwrap(),
    );
    (
        header,
        include_str!("../assets/styles/output.css").to_owned(),
    )
}

#[debug_handler]
async fn get_script() -> (HeaderMap, String) {
    let mut header = HeaderMap::new();
    header.insert(
        HeaderName::from_lowercase(b"content-type").unwrap(),
        HeaderValue::from_str("application/javascript").unwrap(),
    );
    header.insert(
        HeaderName::from_lowercase(b"cache-control").unwrap(),
        HeaderValue::from_str("max-age=31536000").unwrap(),
    );
    (
        header,
        include_str!("../assets/scripts/script.js").to_owned(),
    )
}
