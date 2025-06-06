use crate::scoring_logic::calculator::is_road_running_event;
use crate::models::Event;
use leptos::prelude::*;

#[component]
pub fn ElevationInput(
    event: ReadSignal<Event>,
    #[allow(unused_variables)] net_downhill: ReadSignal<Option<f64>>,
    set_net_downhill: WriteSignal<Option<f64>>,
) -> impl IntoView {
    view! {
        <Show
            when=move || { is_road_running_event(&event.get()) }
            fallback=|| view! { <div></div> }
        >
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-start">
                <label for="net_downhill" class="text-gray-800 font-medium">
                    "Net Downhill (m/km):"
                </label>
                <div class="md:col-span-2">
                    <input
                        id="net_downhill"
                        type="number"
                        step="0.1"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-black"
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            if value.is_empty() {
                                set_net_downhill.set(None);
                            } else {
                                let parsed_value = if value.is_empty() {
                                    0.0
                                } else {
                                    value.parse().unwrap_or(0.0)
                                };
                                set_net_downhill.set(Some(parsed_value));
                            }
                        }
                    />
                    <p class="mt-1 text-sm text-gray-500">
                        "Values over 1.0 m/km will result in point deductions"
                    </p>
                </div>
            </div>
        </Show>
    }
}