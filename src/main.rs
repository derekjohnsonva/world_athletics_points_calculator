use leptos::prelude::*;
use world_athletics_points_calulator::scoring_logic::coefficients::load_coefficients;
use world_athletics_points_calulator::scoring_logic::placement_score::init_placement_score_calculator;
use world_athletics_points_calulator::App;

fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    match load_coefficients() {
        Ok(_) => log::debug!("Coefficients loaded successfully."),
        Err(e) => log::error!("Failed to load coefficients: {}", e),
    }

    match init_placement_score_calculator() {
        Ok(_) => log::debug!("Placement scores loaded successfully."),
        Err(e) => log::error!("Failed to load placement scores: {}", e),
    }

    mount_to_body(|| {
        view! { <App /> }
    })
}
