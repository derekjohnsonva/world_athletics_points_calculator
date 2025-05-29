use leptos::prelude::*;
use world_athletics_points_calulator::App;

fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
