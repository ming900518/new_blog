use crate::{
    component::{blog::Blog, drawer::Drawer, home::Home, navbar::Navbar},
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
pub fn App() -> impl IntoView {
    provide_meta_context();
    let current_prefers_dark_scheme = create_rw_signal(false);
    let current_theme = create_rw_signal(String::new());
    let light_theme = create_rw_signal(String::new());
    let dark_theme = create_rw_signal(String::new());
    let max_chisaki_mode = create_rw_signal(false);
    let max_chisaki_mode_css = create_rw_signal(None);
    let show_max_chisaki_checkbox = create_rw_signal(false);
    let small_screen = create_rw_signal(false);
    let scrolling = create_rw_signal(ScrollDirection::None);
    let (last_top, set_last_top) = create_signal(0);

    create_effect(move |_| {
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

        max_chisaki_mode_css.set(
            window()
                .document()
                .unwrap()
                .create_element("link")
                .map(|element| {
                    element.set_attribute("rel", "stylesheet").ok();
                    element.set_attribute("type", "text/css").ok();
                    element.set_attribute("href", "/max-chisaki-mode.css").ok();
                    element
                })
                .ok(),
        );

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

        if let Ok(Some(value)) = local_stroage.get_item("max_chisaki_mode") {
            show_max_chisaki_checkbox.set(true);
            if value == "true" {
                max_chisaki_mode.set(true);
                if current_theme.get() == "chisaki" {
                    window()
                        .document()
                        .unwrap()
                        .get_elements_by_tag_name("head")
                        .get_with_index(0)
                        .unwrap()
                        .append_child(&max_chisaki_mode_css.get().unwrap())
                        .ok();
                }
            }
        }
    });

    view! {
        <Title text="Ming Chang"/>
        <Meta name="apple-mobile-web-app-capable" content="yes"/>
        <Meta name="apple-touch-fullscreen" content="yes"/>
        <Suspense fallback=move || view! { <Html lang="zh-Hant" /> }>
            {
                move || view! { <Html lang="zh-Hant" attr:data-theme={ current_theme.get() } /> }.into_view()
            }
        </Suspense>
        <Stylesheet id="leptos" href="/pkg/new_blog.css" />
        <Router fallback=fallback>
            <main class="bg-scroll bg-cover bg-center h-screen overflow-y-clip" style="background-image: url(/bg.webp)">
                <div class="flex flex-col bg-gradient-to-b from-transparent to-black">
                    <Navbar scrolling />
                    <div class="flex flex-row max-h-screen">
                        <div class={move || {
                            let location = use_location();
                            if location.pathname.get() == "/" {
                                "drawer lg:drawer-open"
                            } else {
                                "drawer"
                            }
                        }}>
                            <Drawer light_theme dark_theme current_theme current_prefers_dark_scheme max_chisaki_mode show_max_chisaki_checkbox max_chisaki_mode_css />
                            <div class={move || {
                                let location = use_location();
                                    if location.pathname.get() == "/" {
                                        "drawer-content flex flex-col items-start justify-start lg:m-5 lg:ml-0 overflow-scroll lg:max-h-[calc(100vh-6.5rem)]"
                                    } else {
                                        "drawer-content flex flex-col items-start justify-start lg:m-5 overflow-scroll lg:max-h-[calc(100vh-6.5rem)]"
                                    }
                                }}
                            >
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
                                        <Route path="/blog/:filename" view= || view! { <Blog />} ssr=SsrMode::Async />
                                        <Route path="" view= || view! { <Home /> } ssr=SsrMode::Async />
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

fn fallback() -> View {
    let mut outside_errors = Errors::default();
    outside_errors.insert_with_default_key(AppError::NotFound);
    view! {
        <ErrorTemplate outside_errors/>
    }
    .into_view()
}
