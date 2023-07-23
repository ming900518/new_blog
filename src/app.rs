use crate::{
    component::{about::About, blog::Blog, drawer::Drawer, home::Home, navbar::Navbar},
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::MediaQueryList;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    let current_prefers_dark_scheme = create_rw_signal(cx, false);
    let current_theme = create_rw_signal(cx, String::new());
    let light_theme = create_rw_signal(cx, String::new());
    let dark_theme = create_rw_signal(cx, String::new());

    create_effect(cx, move |_| {
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

        let closure: Closure<dyn FnMut(_)> = Closure::new(move |e: MediaQueryList| {
            if e.matches() {
                current_prefers_dark_scheme.set(true);
                current_theme.set(dark_theme.get_untracked());
            } else {
                current_prefers_dark_scheme.set(false);
                current_theme.set(light_theme.get_untracked());
            }
        });

        prefers_color_scheme.set_onchange(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    });

    view! {
        cx,
        <Suspense fallback=move || view! { cx, <Html lang="zh-Hant" /> }>
            {
                move || view! { cx,  <Html lang="zh-Hant" attributes=AdditionalAttributes::from(vec![("data-theme", current_theme.get())]) /> }.into_view(cx)
            }
        </Suspense>
        <Title text="Ming Chang"/>
        <Stylesheet id="leptos" href="/pkg/new_blog.css" />
        <Router fallback=fallback>
            <main class="bg-scroll bg-cover bg-center" style="background-image: url(/bg.webp)">
                <div class="flex flex-col bg-gradient-to-b from-transparent to-black">
                    <Navbar />
                    <div class="flex flex-row">
                        <div class="drawer lg:drawer-open">
                            <Drawer light_theme dark_theme current_theme current_prefers_dark_scheme />
                            <div class="drawer-content flex flex-col items-start justify-start h-[calc(100vh-6.5rem)] lg:m-5 lg:ml-0 overflow-scroll">
                                <Routes>
                                    <Route path="/blog/:id" view= |cx| view! { cx, <Blog />}/>
                                    <Route path="/about" view= |cx| view! { cx, <About />}/>
                                    <Route path="" view= |cx| view! { cx, <Home /> }/>
                                </Routes>
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
