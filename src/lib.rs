use cfg_if::cfg_if;

mod app;
mod fallback;
pub mod routes;
mod server;
pub mod server_fns;
mod user;

// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;
        use crate::app::*;
        use leptos::view;

        #[wasm_bindgen]
        pub fn hydrate() {
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            leptos::mount_to_body(|| {
                view! { <App/> }
            });
        }
    }
}
