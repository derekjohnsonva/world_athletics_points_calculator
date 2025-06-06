use crate::models::{Event, Gender};
use leptos::prelude::*;
use strum::IntoEnumIterator;

#[component]
pub fn EventSelectionInputs(
    gender: ReadSignal<Gender>,
    set_gender: WriteSignal<Gender>,
    event: ReadSignal<Event>,
    set_event: WriteSignal<Event>,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
            <label for="gender" class="text-gray-800 font-medium">
                "Gender:"
            </label>
            <select
                id="gender"
                class="md:col-span-2 w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-black"
                on:change=move |ev| {
                    let value = event_target_value(&ev);
                    match value.as_str() {
                        "Men" => set_gender.set(Gender::Men),
                        "Women" => set_gender.set(Gender::Women),
                        _ => {}
                    }
                }
            >
                {Gender::iter()
                    .map(|g| {
                        view! {
                            <option value=format!("{}", g) selected=move || gender.get() == g>
                                {format!("{}", g)}
                            </option>
                        }
                    })
                    .collect_view()}
            </select>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
            <label for="event" class="text-gray-800 font-medium">
                "Event:"
            </label>
            <select
                id="event"
                class="md:col-span-2 w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-black"
                on:change=move |ev| {
                    let value = event_target_value(&ev);
                    log::info!("Select changed to: {}", value);
                    if let Some(event_type) = Event::from_string(&value) {
                        set_event.set(event_type);
                    }
                }
            >
                {Event::all_variants()
                    .into_iter()
                    .map(|e| {
                        view! {
                            <option
                                value=format!("{}", e)
                                selected=move || event.get().to_string() == e.to_string()
                            >
                                {format!("{}", e)}
                            </option>
                        }
                    })
                    .collect_view()}
            </select>
        </div>
    }
}