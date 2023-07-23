use leptos::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

#[component]
pub fn Drawer(
    cx: Scope,
    light_theme: RwSignal<String>,
    dark_theme: RwSignal<String>,
    current_theme: RwSignal<String>,
    current_prefers_dark_scheme: RwSignal<bool>,
) -> impl IntoView {
    view! { cx,
        <div class="drawer lg:drawer-open">
            <input id="drawer" type="checkbox" class="drawer-toggle" />
            <div class="drawer-side h-screen lg:h-[calc(100vh-5.5rem)] lg:m-3 lg:rounded-lg">
                <label for="drawer" class="drawer-overlay" />
                <ul class="menu p-4 w-64 h-full bg-base-200 lg:bg-base-200/[.7] text-base-content grow">
                    <li><A href="">"首頁"</A></li>
                    <div class="divider"/>
                    <p class="font-bold text-lg">"主題設定"</p>
                    <label class="label">"亮色"</label>
                    <Suspense fallback=move || view! { cx,  }>
                    {
                        move || view! { cx,
                            <select
                                class="select select-bordered select-ghost"
                                on:change=move |e| {
                                    let select = e.target().unwrap().dyn_into::<HtmlSelectElement>().unwrap().value();
                                    window().local_storage().ok().flatten().unwrap().set_item("theme.light", &select).unwrap();
                                    light_theme.set(select.clone());
                                    if !current_prefers_dark_scheme.get() {
                                        current_theme.set(select);
                                    }
                                }
                            >
                                <option disabled>"選擇主題"</option>
                                <option value="chisaki" label="ちさき（預設）" selected=move || light_theme.get() == "chisaki" />
                                <option value="light" label="Light" selected=move || light_theme.get() == "light" />
                                <option value="retro" label="Retro" selected=move || light_theme.get() == "retro" />
                            </select>
                        }.into_view(cx)
                    }
                    </Suspense>
                    <label class="label">"暗色"</label>
                    <Suspense fallback=move || view! { cx,  }>
                    {
                        move || view! { cx,
                            <select
                                class="select select-bordered select-ghost"
                                on:change=move |e| {
                                    let select = e.target().unwrap().dyn_into::<HtmlSelectElement>().unwrap().value();
                                    window().local_storage().ok().flatten().unwrap().set_item("theme.dark", &select).unwrap();
                                    dark_theme.set(select.clone());
                                    if current_prefers_dark_scheme.get() {
                                        current_theme.set(select);
                                    }
                                }
                            >
                                <option disabled>"選擇主題"</option>
                                <option value="coffee" label="Coffee（預設）" selected=move || dark_theme.get() == "coffee" />
                                <option value="dark" label="Dark" selected=move || dark_theme.get() == "dark" />
                                <option value="dracula" label="Dracula" selected=move || dark_theme.get() == "dracula" />
                            </select>
                        }.into_view(cx)
                    }
                    </Suspense>
                </ul>
          </div>
      </div>
    }
}
