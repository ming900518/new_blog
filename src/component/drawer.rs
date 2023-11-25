use leptos::*;
use leptos_meta::Html;
use leptos_router::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{HtmlInputElement, HtmlSelectElement, MediaQueryList};

#[component]
pub fn Drawer() -> impl IntoView {
    let current_prefers_dark_scheme = RwSignal::new(false);
    let current_theme = RwSignal::new(String::new());
    let light_theme = RwSignal::new(String::new());
    let dark_theme = RwSignal::new(String::new());
    let max_chisaki_mode = RwSignal::new(false);
    let max_chisaki_mode_css = RwSignal::new(None);
    let show_max_chisaki_checkbox = RwSignal::new(false);
    let target = RwSignal::new(None);

    create_effect(move |_| {
        let local_stroage = window().local_storage().ok().flatten().unwrap();
        let chosen_light_theme = local_stroage.get_item("theme.light").ok().flatten();
        let chosen_dark_theme = local_stroage.get_item("theme.dark").ok().flatten();

        request_animation_frame(move || {
            let prefers_color_scheme = window()
                .match_media("(prefers-color-scheme:dark)")
                .ok()
                .flatten()
                .unwrap();

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

            let colorscheme_closure: Closure<dyn FnMut(_)> =
                Closure::new(move |e: MediaQueryList| {
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

            target.set(
                window()
                    .document()
                    .unwrap()
                    .get_elements_by_tag_name("head")
                    .get_with_index(0),
            );
        });
    });

    view! {
        <>
            <Suspense fallback=move || view! { <Html lang="zh-Hant" /> }>
                {
                    move || view! { <Html lang="zh-Hant" attr:data-theme={ current_theme.get() } /> }.into_view()
                }
            </Suspense>
            <input id="drawer" type="checkbox" class="drawer-toggle" />
            <div class="drawer-side h-full z-50">
                <label for="drawer" class="drawer-overlay" />
                <ul class="menu p-4 pt-5 pb-5 w-60 h-full bg-base-300 text-base-content overflow-scroll flex-nowrap">
                    <p class="font-bold text-lg mb-3">"傳送門"</p>
                    <li><A href="/">"文章列表"</A></li>
                    <li><a href="https://mingchang.tw/">"關於我"</a></li>
                    <div class="divider"/>
                    <p class="font-bold text-lg mt-3 mb-3">"主題設定"</p>
                    <label class="label">"亮色"</label>
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
                    <label class="label">"暗色"</label>
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
                </ul>
          </div>
      </>
    }
}
