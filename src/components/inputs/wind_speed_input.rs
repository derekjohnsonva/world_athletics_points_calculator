use crate::scoring_logic::calculator::is_wind_affected_event;
use crate::models::Event;
use leptos::prelude::*;

#[component]
pub fn WindSpeedInput(
    event: ReadSignal<Event>,
    #[allow(unused_variables)] wind_speed: ReadSignal<Option<f64>>,
    set_wind_speed: WriteSignal<Option<f64>>,
) -> impl IntoView {
    view! {
        <Show
            when=move || { is_wind_affected_event(&event.get()) }
            fallback=|| view! { <div></div> }
        >
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
                <label for="wind_speed" class="text-gray-800 font-medium">
                    "Wind Speed (m/s):"
                </label>
                <input
                    id="wind_speed"
                    type="number"
                    step="0.1"
                    class="md:col-span-2 w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-black"
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        let parsed_value = if value.is_empty() {
                            0.0
                        } else {
                            value.parse().unwrap_or(0.0)
                        };
                        set_wind_speed.set(Some(parsed_value));
                    }
                />
            </div>
        </Show>
    }
}