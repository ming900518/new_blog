use crate::{
    component::{about::About, blog::Blog, drawer::Drawer, home::Home, navbar::Navbar},
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::cmp::Ordering;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlDivElement, MediaQueryList};

#[derive(Debug, Default, Clone, Copy)]
pub enum ScrollDirection {
    Down,
    Up,
    #[default]
    None,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    let current_prefers_dark_scheme = create_rw_signal(cx, false);
    let current_theme = create_rw_signal(cx, String::new());
    let light_theme = create_rw_signal(cx, String::new());
    let dark_theme = create_rw_signal(cx, String::new());
    let small_screen = create_rw_signal(cx, false);
    let scrolling = create_rw_signal(cx, ScrollDirection::None);
    let (last_top, set_last_top) = create_signal(cx, 0);

    create_effect(cx, move |_| {
        if let Ok(width) = window().inner_width() {
            if width.as_f64().unwrap_or(f64::MAX) > 769.0 {
                small_screen.set(false);
            } else {
                small_screen.set(true);
            }
        } else {
            small_screen.set(false);
        }

        let prefers_color_scheme = window()
            .match_media("(prefers-color-scheme:dark)")
            .ok()
            .flatten()
            .unwrap();

        let local_stroage = window().local_storage().ok().flatten().unwrap();
        let chosen_light_theme = local_stroage.get_item("theme.light").ok().flatten();
        let chosen_dark_theme = local_stroage.get_item("theme.dark").ok().flatten();

        light_theme.set(chosen_light_theme.unwrap_or_else(|| String::from("chisaki")));
        dark_theme.set(chosen_dark_theme.unwrap_or_else(|| String::from("coffee")));

        if prefers_color_scheme.matches() {
            current_prefers_dark_scheme.set(true);
            current_theme.set(dark_theme.get_untracked());
        } else {
            current_prefers_dark_scheme.set(false);
            current_theme.set(light_theme.get_untracked());
        }

        let colorscheme_closure: Closure<dyn FnMut(_)> = Closure::new(move |e: MediaQueryList| {
            if e.matches() {
                current_prefers_dark_scheme.set(true);
                current_theme.set(dark_theme.get_untracked());
            } else {
                current_prefers_dark_scheme.set(false);
                current_theme.set(light_theme.get_untracked());
            }
        });

        prefers_color_scheme.set_onchange(Some(colorscheme_closure.as_ref().unchecked_ref()));
        colorscheme_closure.forget();
    });

    view! {
        cx,
        <Suspense fallback=move || view! { cx, <Html lang="zh-Hant" /> }>
            {
                move || view! { cx,  <Html lang="zh-Hant" attributes=AdditionalAttributes::from(vec![("data-theme", current_theme.get())]) /> }.into_view(cx)
            }
        </Suspense>
        <Title text="Ming Chang"/>
        <Meta name="apple-mobile-web-app-capable" content="yes"/>
        <Meta name="apple-touch-fullscreen" content="yes"/>
        <Stylesheet id="leptos" href="/pkg/new_blog.css" />
        <Router fallback=fallback>
            <main class="bg-scroll bg-cover bg-center h-screen overflow-y-clip" style="background-image: url(/bg.webp)">
                <div class="flex flex-col bg-gradient-to-b from-transparent to-black">
                    <Navbar scrolling />
                    <div class="flex flex-row max-h-screen">
                        <div class="drawer lg:drawer-open">
                            <Drawer light_theme dark_theme current_theme current_prefers_dark_scheme />
                            <div class="drawer-content flex flex-col items-start justify-start lg:m-5 lg:ml-0 overflow-scroll lg:max-h-[calc(100vh-6.5rem)]">
                                <div id="content" class="lg:rounded-lg lg:bg-base-200/[.7] pb-0 overflow-y-scroll overflow-x-clip w-full" on:scroll= move |_| {
                                    let target = window().document().unwrap().query_selector("#content").ok().flatten().unwrap().dyn_into::<HtmlDivElement>().unwrap();
                                    let top = target.scroll_top();
                                    let last_top = last_top.get();
                                    if small_screen.get() && top > 0 {
                                        match (top - last_top).cmp(&0) {
                                            Ordering::Greater => scrolling.set(ScrollDirection::Down),
                                            Ordering::Less => scrolling.set(ScrollDirection::Up),
                                            Ordering::Equal => scrolling.set(ScrollDirection::None)
                                        }
                                    } else {
                                        scrolling.set(ScrollDirection::None)
                                    }
                                    set_last_top.set(top);
                                }>
                                    <Routes>
                                        <Route path="/blog/:filename" view= |cx| view! { cx, <Blog />} ssr=SsrMode::Async/>
                                        <Route path="/about" view= |cx| view! { cx, <About />} ssr=SsrMode::Async/>
                                        <Route path="" view= |cx| view! { cx, <Home /> } ssr=SsrMode::Async/>
                                    </Routes>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </main>
        </Router>
    }
}

fn fallback(cx: Scope) -> View {
    let mut outside_errors = Errors::default();
    outside_errors.insert_with_default_key(AppError::NotFound);
    view! { cx,
        <ErrorTemplate outside_errors/>
    }
    .into_view(cx)
}
