use std::cmp::Ordering;
use std::ops::{Div, Mul, Sub};

use leptos::html::Header;
use leptos::logging::log;
use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    let header_ref = create_node_ref::<Header>();

    let last_y = RwSignal::new(0_f64);
    let direction = RwSignal::new(Ordering::Equal);
    let moved = RwSignal::new(0_f64);

    let handle = window_event_listener(ev::scroll, move |_| {
        let y = window().page_y_offset().unwrap_or_default();
        let y_rem = 1.0_f64.div(16.0).mul(y);
        let new_sub = y_rem.sub(last_y.get());
        let orig_direction = direction.get();
        let new_direction = new_sub.total_cmp(&0_f64);

        if orig_direction == new_direction {
            moved.update(|sub| *sub += new_sub.abs())
        } else {
            moved.set(new_sub);
            direction.set(new_direction);
        }

        last_y.set(y_rem);

        if y.gt(&0.0) {
            moved.get().gt(&5.0).then(|| match direction.get() {
                Ordering::Greater => header_ref.get().map(|header| header.style("top", "-4rem")),
                Ordering::Less => header_ref.get().map(|header| header.style("top", "0")),
                _ => None,
            });
        }
    });

    on_cleanup(move || handle.remove());

    view! {
        <header class="fixed top-0 z-50 navbar bg-base-200/80 backdrop-blur-xl w-[100dvw]" style="transition: top 0.2s ease-in-out" _ref=header_ref>
            <div class="flex-none">
                <label for="drawer" class="btn btn-square btn-ghost">
                    <svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="23.0273" height="17.998">
                     <g>
                      <rect height="17.998" opacity="0" width="23.0273" x="0" y="0"/>
                      <path d="M3.06641 17.998L19.9609 17.998C22.0117 17.998 23.0273 16.9824 23.0273 14.9707L23.0273 3.04688C23.0273 1.03516 22.0117 0.0195312 19.9609 0.0195312L3.06641 0.0195312C1.02539 0.0195312 0 1.02539 0 3.04688L0 14.9707C0 16.9922 1.02539 17.998 3.06641 17.998ZM3.08594 16.4258C2.10938 16.4258 1.57227 15.9082 1.57227 14.8926L1.57227 3.125C1.57227 2.10938 2.10938 1.5918 3.08594 1.5918L19.9414 1.5918C20.9082 1.5918 21.4551 2.10938 21.4551 3.125L21.4551 14.8926C21.4551 15.9082 20.9082 16.4258 19.9414 16.4258ZM7.44141 16.7285L8.97461 16.7285L8.97461 1.29883L7.44141 1.29883ZM5.56641 5.21484C5.85938 5.21484 6.12305 4.95117 6.12305 4.66797C6.12305 4.375 5.85938 4.12109 5.56641 4.12109L3.4668 4.12109C3.17383 4.12109 2.91992 4.375 2.91992 4.66797C2.91992 4.95117 3.17383 5.21484 3.4668 5.21484ZM5.56641 7.74414C5.85938 7.74414 6.12305 7.48047 6.12305 7.1875C6.12305 6.89453 5.85938 6.65039 5.56641 6.65039L3.4668 6.65039C3.17383 6.65039 2.91992 6.89453 2.91992 7.1875C2.91992 7.48047 3.17383 7.74414 3.4668 7.74414ZM5.56641 10.2637C5.85938 10.2637 6.12305 10.0195 6.12305 9.72656C6.12305 9.43359 5.85938 9.17969 5.56641 9.17969L3.4668 9.17969C3.17383 9.17969 2.91992 9.43359 2.91992 9.72656C2.91992 10.0195 3.17383 10.2637 3.4668 10.2637Z" fill="#000000" fill-opacity="0.85"/>
                     </g>
                    </svg>
                </label>
            </div>
            <p class="normal-case text-xl font-bold ml-2 grow">"Ming Chang"</p>
            <div class="flex gap-2">
                <a class="btn btn-ghost" href="https://twitter.com/mingchang137" target="_blank">
                    <svg class="h-4 w-4 text-black" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z" />
                    </svg>
                </a>
                <a class="btn btn-ghost" href="https://github.com/ming900518" target="_blank">
                    <svg class="h-4 w-4 text-black" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22" />
                    </svg>
                </a>
                <a class="btn btn-ghost" href="mailto:mail@mingchang.tw" target="_blank">
                    <svg class="h-4 w-4 text-black" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" />
                        <polyline points="22,6 12,13 2,6" />
                    </svg>
                </a>
            </div>
        </header>
    }
}
