use leptos::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement, HtmlSelectElement};

#[component]
pub fn Drawer(
    light_theme: RwSignal<String>,
    dark_theme: RwSignal<String>,
    current_theme: RwSignal<String>,
    current_prefers_dark_scheme: RwSignal<bool>,
    max_chisaki_mode: RwSignal<bool>,
    max_chisaki_mode_css: RwSignal<Option<Element>>,
    show_max_chisaki_checkbox: RwSignal<bool>,
) -> impl IntoView {
    let location = use_location();
    let target = create_rw_signal(None);

    create_effect(move |_| {
        target.set(
            window()
                .document()
                .unwrap()
                .get_elements_by_tag_name("head")
                .get_with_index(0),
        );
    });

    view! {
        <>
            <input id="drawer" type="checkbox" class="drawer-toggle" />
            <div class={move || {
                            if location.pathname.get() == "/" {
                                "drawer-side h-screen lg:h-[calc(100vh-6.5rem)] lg:m-5 lg:rounded-lg z-50"
                            } else {
                                "drawer-side h-screen z-50"
                            }
                        }}
            >
                <label for="drawer" class="drawer-overlay" />
                <ul class={move || {
                            if location.pathname.get() == "/" {
                                "menu p-4 pt-5 pb-5 w-60 h-full bg-base-200 lg:bg-base-200/[.7] text-base-content overflow-scroll flex-nowrap"
                            } else {
                                "menu p-4 pt-5 pb-5 w-60 h-full bg-base-200 text-base-content overflow-scroll flex-nowrap"
                            }
                        }}
            >
                    <p class="font-bold text-lg mb-3">"傳送門"</p>
                    <li><A href="">"文章列表"</A></li>
                    <li><a href="https://mingchang.tw/">"關於我"</a></li>
                    <div class="divider"/>
                    <p class="font-bold text-lg mt-3 mb-3">"主題設定"</p>
                    <label class="label">"亮色"</label>
                    <Suspense fallback=move || view! {}>
                        {view! {
                            <>
                            <select
                                class="select select-bordered select-ghost"
                                on:change=move |e| {
                                    let select = e.target().unwrap().dyn_into::<HtmlSelectElement>().unwrap().value();
                                    window().local_storage().ok().flatten().unwrap().set_item("theme.light", &select).unwrap();
                                    light_theme.set(select.clone());
                                    if select == "chisaki" {
                                        show_max_chisaki_checkbox.set(true);
                                        if max_chisaki_mode.get() {
                                            if let Some(element) = max_chisaki_mode_css.get() {
                                                target.get().unwrap()
                                                    .append_child(&element)
                                                    .ok();
                                            }
                                        }
                                    } else if max_chisaki_mode.get() {
                                        if let Some(element) = max_chisaki_mode_css.get() {
                                            target.get().unwrap()
                                                .remove_child(&element)
                                                .ok();
                                        }
                                    }
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
                            <div class="form-control">
                                <label class=move || if show_max_chisaki_checkbox.get() && current_theme.get() == "chisaki" { "label cursor-pointer" } else { "label cursor-pointer hidden" }>
                                    <span class="label-text">マックスちさき</span>
                                    <input type="checkbox" checked=move || max_chisaki_mode.get() class="toggle" on:change=move |e| {
                                        let status = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().checked();
                                        max_chisaki_mode.set(status);
                                        window().local_storage().ok().flatten().unwrap().set_item("max_chisaki_mode", &status.to_string()).unwrap();

                                        if let Some(element) = max_chisaki_mode_css.get() {
                                            if status {
                                                max_chisaki_mode.set(true);
                                                target.get().unwrap()
                                                    .append_child(&element)
                                                    .ok();
                                            } else {
                                                max_chisaki_mode.set(false);
                                                target.get().unwrap()
                                                    .remove_child(&element)
                                                    .ok();
                                            }
                                        }
                                    } />
                                </label>
                            </div>
                            </>
                        }.into_view()}
                    </Suspense>
                    <label class="label">"暗色"</label>
                    <Suspense fallback=move || view! {}>
                    {
                        move || view! {
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
                        }.into_view()
                    }
                    </Suspense>
                </ul>
          </div>
      </>
    }
}
