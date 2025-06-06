use crate::models::{Event, PerformanceType};
use leptos::prelude::*;

#[component]
pub fn PerformanceInput(
    event: ReadSignal<Event>,
    performance_input: ReadSignal<String>,
    set_performance_input: WriteSignal<String>,
    set_performance: WriteSignal<f64>,
    parse_error: ReadSignal<Option<String>>,
    set_parse_error: WriteSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-start">
            <label for="performance" class="text-gray-800 font-medium">
                "Performance:"
            </label>
            <div class="md:col-span-2">
                <input
                    id="performance"
                    type="text"
                    value=move || performance_input.get()
                    class=move || {
                        if parse_error.get().is_some() {
                            "w-full px-3 py-2 border border-red-300 rounded-md focus:outline-none focus:ring-1 focus:ring-red-500 bg-red-50"
                        } else {
                            "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-black"
                        }
                    }
                    placeholder=move || {
                        match event.get().performance_type() {
                            PerformanceType::Time => "e.g., 10.50 or 1:30.25 or 2:15:30.50",
                            PerformanceType::Distance => "e.g., 8.95 (meters)",
                        }
                    }
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        set_performance_input.set(value.clone());

                        // Clear any previous parse errors when user starts typing
                        set_parse_error.set(None);

                        // Validate input and update parse error if needed
                        let validation_result = match event.get().performance_type() {
                            PerformanceType::Time => {
                                // Try to parse as time string first, then as direct seconds
                                Event::parse_time_to_seconds(&value).or_else(|_| {
                                    value.parse::<f64>().map_err(|_| "Invalid time format. Use formats like 10.50, 1:30.25, or 2:15:30.50".to_string())
                                })
                            }
                            PerformanceType::Distance => {
                                value.parse::<f64>().map_err(|_| "Invalid distance format. Enter a number in meters (e.g., 8.95)".to_string())
                            }
                        };

                        match validation_result {
                            Ok(parsed_value) => {
                                set_performance.set(parsed_value);
                                set_parse_error.set(None);
                            }
                            Err(error_msg) => {
                                if !value.is_empty() {
                                    set_parse_error.set(Some(error_msg));
                                }
                            }
                        }
                    }
                />
                // Error message for parsing errors
                <Show
                    when=move || parse_error.get().is_some()
                    fallback=move || {
                        view! {
                            <p class="mt-1 text-sm text-gray-500">
                                {move || {
                                    match event.get().performance_type() {
                                        PerformanceType::Time => "Enter time as seconds (10.50) or formatted time (mm:ss.mmm or hh:mm:ss.mmm)",
                                        PerformanceType::Distance => "Enter distance in meters (e.g., 8.95 for long jump)",
                                    }
                                }}
                            </p>
                        }
                    }
                >
                    <p class="mt-1 text-sm text-red-600 flex items-center">
                        <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"></path>
                        </svg>
                        {move || parse_error.get().unwrap_or_default()}
                    </p>
                </Show>
            </div>
        </div>
    }
}