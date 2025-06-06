use crate::models::CompetitionCategory;
use crate::scoring_logic::placement_score::RoundType;
use leptos::prelude::*;
use strum::IntoEnumIterator;

#[component]
pub fn PlacementInfoSection(
    include_placement: ReadSignal<bool>,
    set_include_placement: WriteSignal<bool>,
    competition_category: ReadSignal<CompetitionCategory>,
    set_competition_category: WriteSignal<CompetitionCategory>,
    place: ReadSignal<i32>,
    set_place: WriteSignal<i32>,
    round: ReadSignal<RoundType>,
    set_round: WriteSignal<RoundType>,
    size_of_final: ReadSignal<i32>,
    set_size_of_final: WriteSignal<i32>,
    qualified_to_final: ReadSignal<bool>,
    set_qualified_to_final: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
            <label for="include_placement" class="text-gray-800 font-medium">
                "Include Placement Info:"
            </label>
            <div class="md:col-span-2 flex items-center">
                <input
                    id="include_placement"
                    type="checkbox"
                    checked=move || include_placement.get()
                    class="h-5 w-5 rounded border-gray-300 text-black focus:ring-black"
                    on:change=move |ev| {
                        set_include_placement.set(event_target_checked(&ev));
                    }
                />
                <label for="include_placement" class="ml-2 text-gray-700">
                    "Add placement information for additional points"
                </label>
            </div>
        </div>

        <Show
            when=move || include_placement.get()
            fallback=|| view! { <div></div> }
        >
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
                    .map(|c| {
                        view! {
                            <option
                                value=format!("{}", c)
                                selected=move || competition_category.get().to_string() == c.to_string()
                            >
                                {format!("{}", c)}
                            </option>
                        }
                    })
                    .collect_view()}
            </select>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
                <label for="place" class="text-gray-800 font-medium">
                    "Place:"
                </label>
                <input
                    id="place"
                    type="number"
                    min="1"
                    value=move || place.get()
                    class="md:col-span-2 w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-black"
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                            set_place.set(val);
                        }
                    }
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
                        match value.as_str() {
                            "Final" => set_round.set(RoundType::Final),
                            "Semifinal" => set_round.set(RoundType::SemiFinal),
                            "Other" => set_round.set(RoundType::Other),
                            _ => {}
                        }
                    }
                >
                    <option value="Final" selected=move || matches!(round.get(), RoundType::Final)>
                        "Final"
                    </option>
                    <option value="Semifinal" selected=move || matches!(round.get(), RoundType::SemiFinal)>
                        "Semifinal"
                    </option>
                    <option value="Other" selected=move || matches!(round.get(), RoundType::Other)>
                        "Other"
                    </option>
                </select>
            </div>

            <Show
                when=move || matches!(round.get(), RoundType::SemiFinal)
                fallback=|| view! { <div></div> }
            >
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
                    <label for="size_of_final" class="text-gray-800 font-medium">
                        "Size of Final:"
                    </label>
                    <input
                        id="size_of_final"
                        type="number"
                        min="1"
                        value=move || size_of_final.get()
                        class="md:col-span-2 w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-black"
                        on:input=move |ev| {
                            if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                set_size_of_final.set(val);
                            }
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
                            checked=move || qualified_to_final.get()
                            class="h-5 w-5 rounded border-gray-300 text-black focus:ring-black"
                            on:change=move |ev| {
                                set_qualified_to_final.set(event_target_checked(&ev));
                            }
                        />
                        <label for="qualified_to_final" class="ml-2 text-gray-700">
                            "Athlete qualified to the final round"
                        </label>
                    </div>
                </div>
            </Show>
        </Show>
    }
}