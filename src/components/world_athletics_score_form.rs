use crate::models::*;
use crate::scoring_logic::calculator::{
    calculate_world_athletics_score,
    is_wind_affected_event,
    is_road_running_event,
};
use crate::scoring_logic::coefficients::calculate_result_score;
use crate::scoring_logic::placement_score::calculate_placement_score;
use crate::scoring_logic::placement_score::RoundType;

use leptos::prelude::*;
use strum::IntoEnumIterator;

#[component]
pub fn WorldAthleticsScoreForm() -> impl IntoView {
    // State for form inputs
    let (gender, set_gender) = signal(Gender::Men);
    let (event, set_event) = signal(Event::TrackAndField(
        crate::models::TrackAndFieldEvent::M100,
    ));
    let (performance, set_performance) = signal(0.0);
    let (wind_speed, set_wind_speed) = signal(Some(0.0));
    let (net_downhill, set_net_downhill) = signal(None);
    let (competition_category, set_competition_category) = signal(CompetitionCategory::A);
    let (place, set_place) = signal(1);
    let (round, set_round) = signal(RoundType::Final);
    let (size_of_final, set_size_of_final) = signal(8);
    let (qualified_to_final, set_qualified_to_final) = signal(false);
    let (points, set_points) = signal(0.0);
    let (points_calculated, set_points_calculated) = signal(false);

    // Helper to determine if the round is not Final
    let is_not_final_round = move || round.get() != RoundType::Final;

    // Submit handler
    let handle_submit = move || {
        let placement_info = Some(PlacementInfo {
            competition_category: competition_category.get(),
            place: place.get(),
            round: round.get(),
            size_of_final: size_of_final.get(),
            qualified_to_final: qualified_to_final.get(),
        });

        let input = WorldAthleticsScoreInput {
            gender: gender.get(),
            event: event.get(),
            performance: performance.get(),
            wind_speed: if is_wind_affected_event(&event.get()) {
                wind_speed.get()
            } else {
                None
            },
            net_downhill: if is_road_running_event(&event.get()) {
                net_downhill.get()
            } else {
                None
            },
            placement_info,
        };

        log::info!("Submitted input: {:?}", input);
        // You can call your calculation function here with the input
        let calculated_points = calculate_world_athletics_score(
            input,
            calculate_result_score,
            calculate_placement_score,
        );
        match calculated_points {
            Ok(points) => {
                set_points.set(points);
                set_points_calculated.set(true);
            }
            Err(e) => {
                log::error!("Error calculating points: {}", e);
                set_points_calculated.set(false);
            }
        }
    };

    let handle_select_change = move |ev: leptos::ev::SubmitEvent| {
        let value = event_target_value(&ev);
        log::info!("Select changed to: {}", value);

        if let Some(event_type) = Event::from_string(&value) {
            set_event.set(event_type);
            // log::info!("Set event to: {:?}", event_type);
        }
    };

    view! {
        <form
            class="space-y-4"
            on:submit=move |ev| {
                ev.prevent_default();
                handle_submit();
            }
        >
            <div class="mb-6 text-center">
                <h2 class="text-2xl font-bold text-gray-900 mb-2">World Athletics Points Calculator</h2>
                <p class="text-gray-600">Enter event details below to calculate performance points based on World Athletics scoring tables</p>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
                <label for="gender" class="text-gray-800 font-medium">
                    "Gender:"
                </label>
                <select
                    id="gender"
                    class="md:col-span-2 w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-black"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        set_gender.set(if value == "Men" { Gender::Men } else { Gender::Women });
                    }
                >
                    <option value="Men" selected=move || gender.get() == Gender::Men>
                        "Men"
                    </option>
                    <option value="Women" selected=move || gender.get() == Gender::Women>
                        "Women"
                    </option>
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
                    {Event::all_variants().into_iter()
                        .map(|event_option| {
                            let event_string = event_option.to_string();
                            let is_selected = move || event.get() == event_option;

                            view! {
                                <option value=event_string selected=is_selected>
                                    {event_string.clone()}
                                </option>
                            }
                        })
                        .collect::<Vec<_>>()}
                </select>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
                <label for="performance" class="text-gray-800 font-medium">
                    "Performance:"
                </label>
                <input
                    id="performance"
                    type="number"
                    step="0.01"
                    class="md:col-span-2 w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-black"
                    on:input=move |ev| {
                        set_performance.set(event_target_value(&ev).parse().unwrap_or(0.0))
                    }
                />
            </div>
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
                            set_wind_speed.set(Some(event_target_value(&ev).parse().unwrap_or(0.0)))
                        }
                    />
                </div>
            </Show>
            
            <Show
                when=move || { is_road_running_event(&event.get()) }
                fallback=|| view! { <div></div> }
            >
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
                    <label for="net_downhill" class="text-gray-800 font-medium">
                        "Net Downhill (m/km):"
                    </label>
                    <div class="md:col-span-2">
                        <input
                            id="net_downhill"
                            type="number"
                            step="0.1"
                            min="0"
                            placeholder="Leave empty if not downhill"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-black"
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                if value.is_empty() {
                                    set_net_downhill.set(None);
                                } else {
                                    set_net_downhill.set(Some(value.parse().unwrap_or(0.0)));
                                }
                            }
                        />
                        <p class="mt-1 text-sm text-gray-500">
                            "Values over 1.0 m/km will result in point deductions"
                        </p>
                    </div>
                </div>
            </Show>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
                <label for="competition_category" class="text-gray-800 font-medium">
                    "Competition Category:"
                </label>
                <select
                    id="competition_category"
                    class="md:col-span-2 w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-black"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        log::info!("Select changed to: {}", value);
                        if let Some(event_type) = CompetitionCategory::from_string(&value) {
                            set_competition_category.set(event_type);
                        }
                    }
                >
                {CompetitionCategory::iter()
                    .map(|competition_category_option| {
                        let competition_category_option_string = competition_category_option.to_string();
                        let is_selected = move || competition_category.get() == competition_category_option;

                        view! {
                            <option value=competition_category_option_string selected=is_selected>
                                {competition_category_option_string.clone()}
                            </option>
                        }
                    })
                    .collect::<Vec<_>>()}

                </select>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
                <label for="place" class="text-gray-800 font-medium">
                    "Place:"
                </label>
                <input
                    id="place"
                    type="number"
                    class="md:col-span-2 w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-black"
                    on:input=move |ev| set_place.set(event_target_value(&ev).parse().unwrap_or(1))
                />
            </div>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
                <label for="round" class="text-gray-800 font-medium">
                    "Round:"
                </label>
                <select
                    id="round"
                    class="md:col-span-2 w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-black"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        set_round
                            .set(
                                match value.as_str() {
                                    "Final" => RoundType::Final,
                                    "SemiFinal" => RoundType::SemiFinal,
                                    _ => RoundType::Other,
                                },
                            );
                    }
                >
                    <option value="Final" selected=move || round.get() == RoundType::Final>
                        "Final"
                    </option>
                    <option value="SemiFinal" selected=move || round.get() == RoundType::SemiFinal>
                        "SemiFinal"
                    </option>
                    <option value="Other" selected=move || round.get() == RoundType::Other>
                        "Other"
                    </option>
                </select>
            </div>
            <Show
                when=move || { round.get() == RoundType::SemiFinal }
                fallback=|| view! { <div></div> }
            >
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
                    <label for="size_of_final" class="text-gray-800 font-medium">
                        "Size of Final:"
                    </label>
                    <input
                        id="size_of_final"
                        type="number"
                        class="md:col-span-2 w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-black"
                        on:input=move |ev| {
                            set_size_of_final.set(event_target_value(&ev).parse().unwrap_or(8))
                        }
                    />
                </div>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
                    <label for="qualified_to_final" class="text-gray-800 font-medium">
                        "Qualified to Final:"
                    </label>
                    <div class="md:col-span-2 flex items-center">
                        <input
                            id="qualified_to_final"
                            type="checkbox"
                            class="h-5 w-5 rounded border-gray-300 text-black focus:ring-black"
                            on:change=move |ev| {
                                set_qualified_to_final.set(event_target_checked(&ev))
                            }
                        />
                    </div>
                </div>
            </Show>

            <div class="mt-8 flex flex-col items-center">
                <button
                    type="submit"
                    class="px-8 py-3 bg-gray-900 text-white text-lg font-medium rounded-md hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 transition-colors shadow-sm"
                >
                    "Calculate Score"
                </button>

                <Show
                    when=move || points_calculated.get()
                    fallback=|| view! { <div class="mt-6 text-center text-gray-500 italic">"Submit the form to calculate points"</div> }
                >
                    <div class="mt-6 text-center p-4 bg-gray-50 rounded-lg border border-gray-200 shadow-sm">
                        <h3 class="text-2xl font-bold text-gray-800">{"Points: "}<span class="text-gray-900">{move || format!("{:.2}", points.get())}</span></h3>
                        <p class="text-sm text-gray-600 mt-1">Based on World Athletics scoring tables with adjustments for wind and elevation</p>
                    </div>
                </Show>
            </div>
        </form>
    }
}
