use crate::{
    component::{blog::Blog, drawer::Drawer, home::Home, navbar::Navbar},
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    #[cfg(feature = "ssr")]
    if let Some(res_options) = use_context::<leptos_axum::ResponseOptions>() {
        res_options.append_header(
            http::header::CONTENT_TYPE,
            http::HeaderValue::from_static("text/html; charset=utf-8"),
        );
    }

    view! {
        <Title text="Ming Chang"/>
        <Meta name="apple-mobile-web-app-capable" content="yes"/>
        <Meta name="apple-touch-fullscreen" content="yes"/>
        <Stylesheet id="leptos" href="/pkg/new_blog.css" />
        <Body class="min-h-[100dvh] w-[100dvw] bg-base-200" />
        <Router fallback=fallback>
            <main>
                <div class="flex flex-col bg-base-100/60">
                    <Navbar />
                    <div class="flex flex-row h-full w-full drawer">
                        <Drawer />
                        <div class="drawer-content flex flex-col items-start justify-start">
                            <div id="content" class="overflow-y-scroll overflow-x-clip w-[100dvw] pt-[4rem]">
                                <Routes>
                                    <Route
                                        ssr=SsrMode::Async
                                        path="/blog"
                                        view=Blog
                                    />
                                    <Route
                                        ssr=SsrMode::Async
                                        path=""
                                        view=Home
                                    />
                                </Routes>
                            </div>
                        </div>
                    </div>
                </div>
            </main>
        </Router>
    }
}

fn fallback() -> View {
    let mut outside_errors = Errors::default();
    outside_errors.insert_with_default_key(AppError::NotFound);
    view! {
        <ErrorTemplate outside_errors/>
    }
    .into_view()
}
