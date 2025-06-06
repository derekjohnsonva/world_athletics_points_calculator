use crate::components::inputs::{
    ElevationInput, EventSelectionInputs, PerformanceInput, PlacementInfoSection, ScoreDisplay,
    WindSpeedInput,
};
use crate::models::*;
use crate::scoring_logic::calculator::{
    calculate_world_athletics_score, is_road_running_event, is_wind_affected_event,
};
use crate::scoring_logic::coefficients::calculate_result_score;
use crate::scoring_logic::placement_score::{calculate_placement_score, RoundType};

use leptos::prelude::*;

#[component]
pub fn WorldAthleticsScoreForm() -> impl IntoView {
    // State for form inputs
    let (gender, set_gender) = signal(Gender::Men);
    let (event, set_event) = signal(Event::TrackAndField(
        crate::models::TrackAndFieldEvent::M100,
    ));
    let (_performance, set_performance) = signal(0.0);
    let (performance_input, set_performance_input) = signal(String::new());
    let (wind_speed, set_wind_speed) = signal(Some(0.0));
    let (net_downhill, set_net_downhill) = signal(None);
    let (competition_category, set_competition_category) = signal(CompetitionCategory::A);
    let (place, set_place) = signal(1);
    let (round, set_round) = signal(RoundType::Final);
    let (size_of_final, set_size_of_final) = signal(8);
    let (qualified_to_final, set_qualified_to_final) = signal(false);
    let (include_placement, set_include_placement) = signal(true);
    let (points, set_points) = signal(0.0);
    let (points_calculated, set_points_calculated) = signal(false);
    let (parse_error, set_parse_error) = signal(Option::<String>::None);

    // Submit handler
    let handle_submit = move || {
        // Check if there's a parsing error before calculating
        if parse_error.get().is_some() {
            return; // Don't calculate if there's a parsing error
        }

        // Parse performance based on event type
        let parsed_performance = match event.get().performance_type() {
            PerformanceType::Time => {
                // Try to parse as time string first, then as direct seconds
                match Event::parse_time_to_seconds(&performance_input.get()) {
                    Ok(seconds) => seconds,
                    Err(_) => {
                        // If time parsing fails, try to parse as direct number (seconds)
                        match performance_input.get().parse::<f64>() {
                            Ok(seconds) => seconds,
                            Err(_) => {
                                set_parse_error.set(Some("Invalid time format. Use formats like 10.50, 1:30.25, or 2:15:30.50".to_string()));
                                return;
                            }
                        }
                    }
                }
            }
            PerformanceType::Distance => {
                // For distance events, parse directly as meters
                match performance_input.get().parse::<f64>() {
                    Ok(distance) => distance,
                    Err(_) => {
                        set_parse_error.set(Some("Invalid distance format. Enter a number in meters (e.g., 8.95)".to_string()));
                        return;
                    }
                }
            }
        };

        let placement_info = if include_placement.get() {
            Some(PlacementInfo {
                competition_category: competition_category.get(),
                place: place.get(),
                round: round.get(),
                size_of_final: size_of_final.get(),
                qualified_to_final: qualified_to_final.get(),
            })
        } else {
            None
        };

        let input = WorldAthleticsScoreInput {
            gender: gender.get(),
            event: event.get(),
            performance: parsed_performance,
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

        // Calculate the score
        match calculate_world_athletics_score(input, calculate_result_score, calculate_placement_score) {
            Ok(score) => {
                set_points.set(score);
                set_points_calculated.set(true);
            }
            Err(e) => {
                log::error!("Error calculating score: {}", e);
                set_points_calculated.set(false);
            }
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
            <h2 class="text-xl font-semibold text-gray-800 mb-4">
                World Athletics Points Calculator
            </h2>

            <EventSelectionInputs
                gender=gender
                set_gender=set_gender
                event=event
                set_event=set_event
            />

            <PerformanceInput
                event=event
                performance_input=performance_input
                set_performance_input=set_performance_input
                set_performance=set_performance
                parse_error=parse_error
                set_parse_error=set_parse_error
            />

            <WindSpeedInput
                event=event
                wind_speed=wind_speed
                set_wind_speed=set_wind_speed
            />

            <ElevationInput
                event=event
                net_downhill=net_downhill
                set_net_downhill=set_net_downhill
            />

            <PlacementInfoSection
                include_placement=include_placement
                set_include_placement=set_include_placement
                competition_category=competition_category
                set_competition_category=set_competition_category
                place=place
                set_place=set_place
                round=round
                set_round=set_round
                size_of_final=size_of_final
                set_size_of_final=set_size_of_final
                qualified_to_final=qualified_to_final
                set_qualified_to_final=set_qualified_to_final
            />

            <ScoreDisplay
                points=points
                points_calculated=points_calculated
                parse_error=parse_error
            />
        </form>
    }
}