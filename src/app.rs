use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    let theme = create_rw_signal(cx, String::new());
    create_effect(cx, move |_| {
        let chose_theme = window()
            .local_storage()
            .ok()
            .flatten()
            .unwrap()
            .get_item("theme")
            .ok()
            .flatten()
            .unwrap_or_else(|| String::from("chisaki"));
        theme.set(chose_theme);
    });

    view! {
        cx,
        <Suspense fallback=move || view! { cx, <Html lang="zh-Hant" attributes=AdditionalAttributes::from(vec![("data-theme", "chisaki")]) /> }>
            {
                move || view! { cx,  <Html lang="zh-Hant" attributes=AdditionalAttributes::from(vec![("data-theme", theme.get())]) /> }.into_view(cx)
            }
        </Suspense>
        <Title text="Blog"/>
        <Stylesheet id="leptos" href="/pkg/new_blog.css" />
        <Router fallback=fallback>
            <main>
                <div class="flex flex-col">
                    <Navbar />
                    <div class="flex flex-row">
                        <Drawer theme />
                        <Routes>
                            <Route path="" view=|cx| view! { cx, <></> }/>
                        </Routes>
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

#[component]
fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="navbar bg-base-300">
            <div class="flex-none">
                <label for="drawer" class="btn btn-square btn-ghost lg:hidden">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                    </svg>
                </label>
            </div>
            <p class="normal-case text-xl font-bold lg:ml-5">"Ming Chang"</p>
        </div>
    }
}

#[component]
fn Drawer(cx: Scope, theme: RwSignal<String>) -> impl IntoView {
    view! { cx,
        <div class="drawer lg:drawer-open">
            <input id="drawer" type="checkbox" class="drawer-toggle" />
            <div class="drawer-side h-screen lg:h-[calc(100vh-4rem)]">
                <label for="drawer" class="drawer-overlay" />
                <ul class="menu p-4 w-64 h-full bg-base-200 text-base-content grow">
                    <select
                        class="select select-bordered"
                        on:change=move |e| {
                            let select = e.target().unwrap().dyn_into::<HtmlSelectElement>().unwrap().value();
                            window().local_storage().ok().flatten().unwrap().set_item("theme", &select).unwrap();
                            theme.set(select);
                        }
                    >
                        <option disabled>"選擇主題"</option>
                        <option value="chisaki" label="ちさき (預設)" selected=move || theme.get() == "chisaki" />
                        <option value="light" label="Light" selected=move || theme.get() == "light" />
                        <option value="dark" label="Dark" selected=move || theme.get() == "dark" />
                        <option value="retro" label="Retro" selected=move || theme.get() == "retro" />
                    </select>
                    <div class="divider"/>
                    <li><a>"Home"</a></li>
                </ul>
          </div>
      </div>
    }
}
