// src/scoring_logic/calculator.rs
use crate::models::{Event, Gender, TrackAndFieldEvent, WorldAthleticsScoreInput};

use super::placement_score::PlacementScoreCalcInput;

/// Determines if an event is a road running event
pub fn is_road_running_event(event: &Event) -> bool {
    matches!(event, Event::RoadRunning(_))
}

/// Determines if an event is affected by wind for scoring modifications.
/// The wind modification applies in the following events:
/// 100m, 200m, 100m Hurdles, 110mHurdles, Long Jump, Triple Jump

pub fn is_wind_affected_event(event: &Event) -> bool {
    matches!(
        event,
        Event::TrackAndField(TrackAndFieldEvent::M100)
        | Event::TrackAndField(TrackAndFieldEvent::M200)
        | Event::TrackAndField(TrackAndFieldEvent::M100H) // Women's hurdles
        | Event::TrackAndField(TrackAndFieldEvent::M110H) // Men's hurdles
        | Event::TrackAndField(TrackAndFieldEvent::LJ)
        | Event::TrackAndField(TrackAndFieldEvent::TJ)
    )
}

/// Calculates the wind adjustment points based on wind speed.
///
/// Rules:
/// - 1 m/s wind is equivalent to 6 points.
/// - For wind readings in between those identified in the table, the allocation of points is ±0.6 points for every ±0.1 m/s.
/// - Tailwind (positive wind speed): No modification between 0 m/s and +2.0 m/s.
///   Deduction starts from +2.1 m/s, while the calculation of the points to be deducted still starts from 0.0 m/s.
/// - Headwind (negative wind speed): Adds points.
/// - No Wind Information (NWI): Deduct 30 points from the Result Score.
///
/// # Arguments
/// * `wind_speed` - An `Option<f64>` representing the wind speed in m/s.
///
/// # Returns
/// The points to be added or deducted due to wind.
pub(crate) fn calculate_wind_adjustment(wind_speed: Option<f64>) -> f64 {
    const POINTS_PER_M_S: f64 = 6.0;
    const NWI_PENALTY: f64 = -30.0;
    const TAILWIND_THRESHOLD: f64 = 2.0; // No deduction up to +2.0 m/s

    match wind_speed {
        Some(wind_value) => {
            if wind_value > 0.0 {
                // Tailwind
                if wind_value > TAILWIND_THRESHOLD {
                    // For tailwind > +2.0 m/s, deduction applies.
                    // The rule "calculation of the points to be deducted still starts from 0.0 m/s"
                    // implies a linear deduction from 0.0 m/s, but only applied if wind > 2.0.
                    // E.g., +2.5 m/s -> -(2.5 * 6.0) = -15.0 pts
                    -(wind_value * POINTS_PER_M_S)
                } else {
                    // No deduction for tailwind <= +2.0 m/s
                    0.0
                }
            } else {
                // Headwind (negative wind_value) or exactly 0.0 m/s
                // Headwind adds points. E.g., -1.0 m/s -> -(-1.0 * 6.0) = +6.0 pts
                // 0.0 m/s -> 0.0 pts
                -wind_value * POINTS_PER_M_S
            }
        }
        None => NWI_PENALTY, // No Wind Information (NWI)
    }
}

/// Calculates the downhill adjustment points based on net elevation drop for road running events.
///
/// Rules:
/// - No deduction if the net drop is within the allowed 1 m/km.
/// - A net drop of 1 m/km of the race distance is equivalent to 6 points deduction.
/// - For each additional 0.1 m/km drop, an additional 0.6 points are deducted.
///
/// # Arguments
/// * `net_downhill` - An `Option<f64>` representing the net elevation drop in m/km.
///
/// # Returns
/// The points to be deducted due to downhill course.
pub(crate) fn calculate_downhill_adjustment(net_downhill: Option<f64>) -> f64 {
    const POINTS_PER_M_KM: f64 = 6.0;
    const POINTS_PER_0_1_M_KM: f64 = 0.6;
    const THRESHOLD: f64 = 1.0; // No deduction below 1 m/km

    match net_downhill {
        Some(drop) => {
            if drop <= THRESHOLD {
                // No deduction for drops within allowed limit
                0.0
            } else {
                // Calculate excess drop above threshold
                let excess = drop - THRESHOLD;
                // Convert to 0.1 m/km units and calculate deduction
                let deduction_base = POINTS_PER_M_KM; // 6 points for the first 1 m/km over threshold
                let deduction_additional = (excess * 10.0) * POINTS_PER_0_1_M_KM; // 0.6 points per 0.1 m/km
                -(deduction_base + deduction_additional)
            }
        }
        None => 0.0, // No adjustment if no drop specified
    }
}

/// Calculates the World Athletics Score for a given performance.
///
/// This function retrieves the appropriate coefficients based on gender and event,
/// then applies the scoring formula. It accepts a `coeff_fetcher` function
/// to allow for mocking in tests.
///
/// # Arguments
/// * `input` - A `WorldAthleticsScoreInput` struct containing all necessary performance details.
/// * `coeff_fetcher` - A function that takes `Gender` and `event_name` (as `&str`) and
///                     returns `Option<Coefficients>`. This allows mocking the coefficient
///                     lookup for testing purposes.
///
/// # Returns
/// A `Result` containing either a `WorldAthleticsScoreOutput` with the calculated points
/// or a `String` error message if coefficients are not found.
pub fn calculate_world_athletics_score(
    input: WorldAthleticsScoreInput,
    result_score_calculator: fn(f64, Gender, &str) -> Result<f64, String>,
    placement_score_calculator: fn(PlacementScoreCalcInput) -> Option<i32>,
) -> Result<f64, String> {
    log::info!("Calculating score for input: {:?}", input);

    let event_id = input.event.to_string(); // e.g., "100m", "TJ"

    // The input.performance is assumed to be already in the standard unit (f64)
    let mut result_score = result_score_calculator(input.performance, input.gender, &event_id)?;

    // Modify result score due to wind for some track events
    // The wind modification applies in the following events:
    if is_wind_affected_event(&input.event) {
        result_score += calculate_wind_adjustment(input.wind_speed);
    }

    // Apply downhill adjustment for road running events
    if is_road_running_event(&input.event) {
        result_score += calculate_downhill_adjustment(input.net_downhill);
    }

    let mut placing_score = 0;

    if let Some(placement_info) = input.placement_info {
        placing_score += placement_score_calculator(PlacementScoreCalcInput {
            event: input.event,
            competition_category: placement_info.competition_category,
            round_type: placement_info.round,
            place: placement_info.place,
            qualified_to_final: placement_info.qualified_to_final,
            size_of_final: placement_info.size_of_final,
        })
        .unwrap_or(0);
    }
    log::debug!(
        "result score = {} and placement score = {}",
        result_score,
        placing_score
    );
    let points = result_score + (placing_score as f64);

    Ok(points)
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module
    use crate::models::*;
    use crate::scoring_logic::placement_score::RoundType;
    use assert_approx_eq::assert_approx_eq;

    // --- Mock function for results score calculator ---
    /// A mock implementation of `result_score_calculator` for testing.
    /// It simulates the calculation of World Athletics points based on a performance result.
    /// It will always return the performance
    fn mock_result_score_calculator(
        performance: f64,
        _gender: Gender,
        _event_name: &str,
    ) -> Result<f64, String> {
        Ok(performance)
    }
    // --- Mock function for placement_score_calculator ---
    /// A mock implementation of `placement_score_calculator` for testing.
    /// It returns a fixed score based on the placement.
    /// This is a simplified mock for testing purposes.
    /// # Arguments
    /// * `input` - A `PlacementScoreCalcInput` struct containing placement details.
    /// # Returns
    /// An `Option<i32>` representing the placement score.
    /// This mock simply returns a fixed score based on the place.
    /// If the place is 1, it returns 100 points; otherwise, it returns 0.
    fn mock_placement_score_calculator(input: PlacementScoreCalcInput) -> Option<i32> {
        // For simplicity, let's say 1st place gets 100 points, others get 0.
        if input.place == 1 {
            Some(100)
        } else {
            Some(0)
        }
    }

    /// Tests the `calculate_wind_adjustment` helper function.
    #[test]
    fn test_calculate_wind_adjustment() {
        // Test cases for tailwind (positive wind_value)
        assert_eq!(calculate_wind_adjustment(Some(0.0)), 0.0); // At 0.0 m/s
        assert_eq!(calculate_wind_adjustment(Some(1.0)), 0.0); // +1.0 m/s (no deduction <= 2.0)
        assert_eq!(calculate_wind_adjustment(Some(1.9)), 0.0); // +1.9 m/s (no deduction <= 2.0)
        assert_eq!(calculate_wind_adjustment(Some(2.0)), 0.0); // +2.0 m/s (no deduction <= 2.0)
        assert_approx_eq!(calculate_wind_adjustment(Some(2.1)), -12.6); // +2.1 m/s (2.1 * 6 = 12.6, deducted)
        assert_approx_eq!(calculate_wind_adjustment(Some(2.5)), -15.0); // +2.5 m/s (2.5 * 6 = 15.0, deducted)
        assert_approx_eq!(calculate_wind_adjustment(Some(3.0)), -18.0); // +3.0 m/s (matches table)
        assert_approx_eq!(calculate_wind_adjustment(Some(4.0)), -24.0); // +4.0 m/s (matches table)

        // Test cases for headwind (negative wind_value)
        assert_eq!(calculate_wind_adjustment(Some(-0.0)), 0.0); // Exactly 0.0 m/s
        assert_approx_eq!(calculate_wind_adjustment(Some(-0.1)), 0.6); // -0.1 m/s (+0.6 pts)
        assert_approx_eq!(calculate_wind_adjustment(Some(-0.5)), 3.0); // -0.5 m/s (+3.0 pts)
        assert_approx_eq!(calculate_wind_adjustment(Some(-1.0)), 6.0); // -1.0 m/s (matches table)
        assert_approx_eq!(calculate_wind_adjustment(Some(-1.5)), 9.0); // -1.5 m/s (+9.0 pts)
        assert_approx_eq!(calculate_wind_adjustment(Some(-2.0)), 12.0); // -2.0 m/s (matches table)
        assert_approx_eq!(calculate_wind_adjustment(Some(-3.0)), 18.0); // -3.0 m/s (matches table)
        assert_approx_eq!(calculate_wind_adjustment(Some(-4.0)), 24.0); // -4.0 m/s (matches table)

        // Test case for No Wind Information (NWI)
        assert_eq!(calculate_wind_adjustment(None), -30.0);
    }

    /// Tests the `calculate_downhill_adjustment` helper function.
    #[test]
    fn test_calculate_downhill_adjustment() {
        // Test cases for downhill courses
        assert_eq!(calculate_downhill_adjustment(None), 0.0); // No downhill data
        assert_eq!(calculate_downhill_adjustment(Some(0.0)), 0.0); // Flat course
        assert_eq!(calculate_downhill_adjustment(Some(0.5)), 0.0); // 0.5 m/km (within allowed)
        assert_eq!(calculate_downhill_adjustment(Some(1.0)), 0.0); // 1.0 m/km (exactly allowed)

        // Beyond allowed limit:
        assert_approx_eq!(calculate_downhill_adjustment(Some(1.1)), -6.6); // 1.1 m/km: -6 - (0.1*10*0.6) = -6.6
        assert_approx_eq!(calculate_downhill_adjustment(Some(1.2)), -7.2); // 1.2 m/km: -6 - (0.2*10*0.6) = -7.2
        assert_approx_eq!(calculate_downhill_adjustment(Some(1.5)), -9.0); // 1.5 m/km: -6 - (0.5*10*0.6) = -9.0
        assert_approx_eq!(calculate_downhill_adjustment(Some(2.0)), -12.0); // 2.0 m/km: -6 - (1*10*0.6) = -12.0
        assert_approx_eq!(calculate_downhill_adjustment(Some(3.0)), -18.0); // 3.0 m/km: -6 - (2*10*0.6) = -18.0
    }

    /// Tests the end-to-end `calculate_world_athletics_score` function using a mock coefficient fetcher.
    #[test]
    fn test_calculate_world_athletics_score() {
        // No need to call load_coefficients() here, as we are mocking the dependency.

        // Test case 1: Men's 100m
        let input1 = WorldAthleticsScoreInput {
            gender: Gender::Men,
            event: Event::TrackAndField(TrackAndFieldEvent::M100),
            performance: 10.50, // Example: 10.50 seconds
            wind_speed: Some(0.0),
            net_downhill: None,
            placement_info: None,
        };
        let expected_points1 = 10.50; // 10.50
        let output1 = calculate_world_athletics_score(
            input1,
            mock_result_score_calculator,
            mock_placement_score_calculator,
        )
        .expect("Calculation failed for men's 100m");
        assert_eq!(output1, expected_points1);

        // Test case 2: Women's Long Jump (LJ)
        let input2 = WorldAthleticsScoreInput {
            gender: Gender::Women,
            event: Event::TrackAndField(TrackAndFieldEvent::LJ),
            performance: 6.50,     // Example: 6.50 meters
            wind_speed: Some(0.0), // with no wind we will apply a penalty
            net_downhill: None,
            placement_info: None,
        };
        let expected_points2 = 6.5;
        let output2 = calculate_world_athletics_score(
            input2,
            mock_result_score_calculator,
            mock_placement_score_calculator,
        )
        .expect("Calculation failed for women's LJ");
        assert_eq!(output2, expected_points2);

        // Test case 4: Men's 5000m (using a value that would be in seconds)
        let input4 = WorldAthleticsScoreInput {
            gender: Gender::Men,
            event: Event::TrackAndField(TrackAndFieldEvent::M5000),
            performance: 840.0, // 14 minutes (840 seconds)
            wind_speed: None,
            net_downhill: None,
            placement_info: None,
        };
        let expected_points4 = 840.0;
        let output4 = calculate_world_athletics_score(
            input4,
            mock_result_score_calculator,
            mock_placement_score_calculator,
        )
        .expect("Calculation failed for men's 5000m");
        assert_eq!(output4, expected_points4);

        // Test case 5: Men's 35km Race Walk. Use a winning position in the final. This should add 100 points.
        let input5 = WorldAthleticsScoreInput {
            gender: Gender::Men,
            event: Event::RaceWalking(RaceWalkingEvent::Road35kmW),
            performance: 9415.0, // Example: 2:36:55
            wind_speed: None,
            net_downhill: None,
            placement_info: Some(PlacementInfo {
                competition_category: CompetitionCategory::A,
                round: RoundType::Final,
                place: 1,
                qualified_to_final: true,
                size_of_final: 12,
            }),
        };
        let expected_points5 = 9415.0 + 100.0; // 9415.0 + 100 points for placement
        let output5 = calculate_world_athletics_score(
            input5,
            mock_result_score_calculator,
            mock_placement_score_calculator,
        )
        .expect("Calculation failed for men's 35km Race Walk");
        assert_eq!(output5, expected_points5);

        // Test case 6: Womens LJ with a -3.0 m/s headwind
        let input6 = WorldAthleticsScoreInput {
            gender: Gender::Women,
            event: Event::TrackAndField(TrackAndFieldEvent::LJ),
            performance: 6.50,      // Example: 6.50 meters
            wind_speed: Some(-3.0), // -3.0 m/s headwind
            net_downhill: None,
            placement_info: None,
        };
        let expected_points6 = 6.50 + 18.0; // 6.50 performance + 18.0 points for headwind adjustment
        let output6 = calculate_world_athletics_score(
            input6,
            mock_result_score_calculator,
            mock_placement_score_calculator,
        )
        .expect("Calculation failed for women's LJ with headwind");
        assert_eq!(output6, expected_points6);

        // Test case 7: Road Marathon with a downhill course (1.5 m/km drop)
        let input7 = WorldAthleticsScoreInput {
            gender: Gender::Men,
            event: Event::RoadRunning(RoadRunningEvent::RoadMarathon),
            performance: 7200.0, // Example: 2:00:00
            wind_speed: None,
            net_downhill: Some(1.5), // 1.5 m/km drop (exceeds the 1.0 m/km allowance)
            placement_info: None,
        };
        let expected_points7 = 7200.0 - 9.0; // 7200.0 - 9.0 points for downhill adjustment
        let output7 = calculate_world_athletics_score(
            input7,
            mock_result_score_calculator,
            mock_placement_score_calculator,
        )
        .expect("Calculation failed for men's Road Marathon with downhill course");
        assert_eq!(output7, expected_points7);

        // Test case 8: Road 10km with a significant downhill course (2.5 m/km drop)
        let input8 = WorldAthleticsScoreInput {
            gender: Gender::Women,
            event: Event::RoadRunning(RoadRunningEvent::Road10km),
            performance: 1800.0, // Example: 30:00
            wind_speed: None,
            net_downhill: Some(2.5), // 2.5 m/km drop
            placement_info: None,
        };
        let expected_points8 = 1800.0 - 15.0; // 1800.0 - 15.0 points for downhill adjustment
        let output8 = calculate_world_athletics_score(
            input8,
            mock_result_score_calculator,
            mock_placement_score_calculator,
        )
        .expect("Calculation failed for women's Road 10km with downhill course");
        assert_eq!(output8, expected_points8);
    }
}
