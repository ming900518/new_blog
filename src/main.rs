use axum::{
    debug_handler,
    extract::Query,
    http::{HeaderMap, HeaderName, HeaderValue},
    response::Html,
    routing::get,
    Router,
};
use axum_server::tls_openssl::OpenSSLConfig;
use mimalloc::MiMalloc;
use pages::{Article, Index};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::OnceLock,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use tower_http::{
    compression::CompressionLayer,
    trace::TraceLayer,
};
use tracing::{log::info, Level};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};
use types::{BlogArticleContent, BlogParams};

mod pages;
mod types;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

static RENDERED_INDEX: OnceLock<Mutex<(Instant, Index)>> = OnceLock::new();

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
        .route("/", get(get_index))
        .route(
            "/blog",
            get(|query| async { Index::article(query).await.get_html() }),
        )
        .route("/article", get(show_article))
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
async fn get_index() -> Html<String> {
    async fn update_index() -> (Instant, Index) {
        (Instant::now(), Index::list().await)
    }

    if let Some(mutex) = RENDERED_INDEX.get() {
        let mut mutex = mutex.lock().await;
        if mutex.0.elapsed() > Duration::from_secs(86400) {
            let (new_instant, new_index) = update_index().await;
            mutex.0 = new_instant;
            mutex.1 = new_index;
        }
        mutex.1.get_html()
    } else {
        let (new_instant, new_index) = update_index().await;
        let html = new_index.get_html();
        RENDERED_INDEX.get_or_init(|| Mutex::new((new_instant, new_index)));
        html
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

#[debug_handler]
async fn show_article(Query(BlogParams { filename, commit }): Query<BlogParams>) -> Html<String> {
    let rendered_pages = RENDERED_PAGES
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .await;
    if let Some(content) = rendered_pages.get(&(filename.clone(), commit.clone())) {
        let content = content.clone();
        drop(rendered_pages);
        Article::success(content.title, content.content).get_html()
    } else {
        drop(rendered_pages);
        Article::from_filename_and_commit(filename, commit).await.get_html()
    }
}
