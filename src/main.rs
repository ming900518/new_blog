#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::net::SocketAddr;

    use axum::{routing::post, Router};
    use axum_server::tls_openssl::OpenSSLConfig;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use new_blog::app::*;
    use new_blog::fileserv::file_and_error_handler;

    simple_logger::init_with_level(log::Level::Info).expect("Couldn't initialize logging");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(|| view! { <App/> });

    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, || view! { <App/> })
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    match OpenSSLConfig::from_pem_file("ssl/ssl.pem", "ssl/ssl.key") {
        Ok(ssl_config) => {
            let addr = SocketAddr::from(([0, 0, 0, 0], 443));
            log::info!("SSL enabled. Listening on {}", addr);
            axum_server::bind_openssl(addr, ssl_config)
                .serve(app.into_make_service())
                .await
                .expect("Server startup failed.");
        }
        Err(_) => {
            let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
            log::info!("SSL disabled. Listening on {}", addr);
            axum_server::bind(addr)
                .serve(app.into_make_service())
                .await
                .expect("Server startup failed.");
        }
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
