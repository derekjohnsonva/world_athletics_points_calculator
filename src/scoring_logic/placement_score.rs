use crate::models::CompetitionCategory;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventGroup {
    TrackAndField,        // Standard track & field events
    Distance5000m3000mSC, // 5000m and 3000mSC
    Distance10000m,       // 10,000m
    Road10km,             // 10km Road Race
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoundType {
    Final,
    SemiFinal,
    Other,
}

#[derive(Debug, Deserialize)]
struct PlacementScoreData {
    track_field_final: HashMap<CompetitionCategory, HashMap<u8, u16>>,
    track_field_semi_max9: HashMap<CompetitionCategory, HashMap<u8, u16>>,
    track_field_semi_10plus: HashMap<CompetitionCategory, HashMap<u8, u16>>,
    distance_5000m_final: HashMap<CompetitionCategory, HashMap<u8, u16>>,
    distance_5000m_semi_max9: HashMap<CompetitionCategory, HashMap<u8, u16>>,
    distance_5000m_semi_10plus: HashMap<CompetitionCategory, HashMap<u8, u16>>,
    distance_10000m_final: HashMap<CompetitionCategory, HashMap<u8, u16>>,
    road_10km_final: HashMap<CompetitionCategory, HashMap<u8, u16>>,
}

pub struct PlacementCalculator {
    data: PlacementScoreData,
}

static CALCULATOR: OnceLock<PlacementCalculator> = OnceLock::new();

impl PlacementCalculator {
    fn new(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data: PlacementScoreData = serde_json::from_str(json_data)?;
        Ok(PlacementCalculator { data })
    }

    pub fn calculate_placement_score(
        &self,
        event_group: EventGroup,
        competition_category: CompetitionCategory,
        round_type: RoundType,
        place: u8,
        size_of_final: u8,
    ) -> Option<u16> {
        // If the athlete qualifies for the final, they get the same points as all other qualified athletes
        let place = if place <= size_of_final && round_type == RoundType::SemiFinal {
            &1
        } else {
            &place
        };

        match (event_group, round_type) {
            (EventGroup::TrackAndField, RoundType::Final) => self
                .data
                .track_field_final
                .get(&competition_category)?
                .get(place)
                .copied(),
            (EventGroup::TrackAndField, RoundType::SemiFinal) => {
                // check to see which semifinal table to use
                if size_of_final <= 9 {
                    self.data
                        .track_field_semi_max9
                        .get(&competition_category)?
                        .get(place)
                        .copied()
                } else {
                    self.data
                        .track_field_semi_10plus
                        .get(&competition_category)?
                        .get(place)
                        .copied()
                }
            }
            (EventGroup::Distance5000m3000mSC, RoundType::Final) => self
                .data
                .distance_5000m_final
                .get(&competition_category)?
                .get(place)
                .copied(),
            (EventGroup::Distance5000m3000mSC, RoundType::SemiFinal) => {
                // check to see which semifinal table to use
                if size_of_final <= 9 {
                    self.data
                        .distance_5000m_semi_max9
                        .get(&competition_category)?
                        .get(place)
                        .copied()
                } else {
                    self.data
                        .distance_5000m_semi_10plus
                        .get(&competition_category)?
                        .get(place)
                        .copied()
                }
            }
            (EventGroup::Distance10000m, RoundType::Final) => self
                .data
                .distance_10000m_final
                .get(&competition_category)?
                .get(&place)
                .copied(),
            (EventGroup::Road10km, RoundType::Final) => self
                .data
                .road_10km_final
                .get(&competition_category)?
                .get(&place)
                .copied(),
            (_, RoundType::Other) => None,
            (EventGroup::Distance10000m, RoundType::SemiFinal) => None,
            (EventGroup::Road10km, RoundType::SemiFinal) => None,
        }
    }
}

/// Initialize the placement calculator with JSON data
/// This should be called once at application startup
pub fn init_placement_score_calculator(json_data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let calculator = PlacementCalculator::new(json_data)?;
    CALCULATOR
        .set(calculator)
        .map_err(|_| "Calculator already initialized")?;
    Ok(())
}

/// Calculate placement score for given parameters
/// Returns None if no score is available for the given combination
pub fn calculate_placement_score(
    event_group: EventGroup,
    competition_category: CompetitionCategory,
    round_type: RoundType,
    place: u8,
    qualified_to_final: bool,
    size_of_final: u8,
) -> Option<u16> {
    CALCULATOR.get()?.calculate_placement_score(
        event_group,
        competition_category,
        round_type,
        place,
        size_of_final,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_json() -> &'static str {
        r#"{
            "track_field_final": {
                "OW": {
                    "1": 375,
                    "2": 330,
                    "3": 300,
                    "4": 270,
                    "5": 250,
                    "6": 230,
                    "7": 215,
                    "8": 200,
                    "9": 130,
                    "10": 120,
                    "11": 110,
                    "12": 100,
                    "13": 95,
                    "14": 90,
                    "15": 85,
                    "16": 80
                },
                "DF": {
                    "1": 240,
                    "2": 210,
                    "3": 185,
                    "4": 170,
                    "5": 155,
                    "6": 145,
                    "7": 135,
                    "8": 125,
                    "9": 90,
                    "10": 80,
                    "11": 70,
                    "12": 60
                },
                "F": {
                    "1": 15,
                    "2": 10,
                    "3": 5
                }
            },
            "track_field_semi_max9": {
                "OW": {
                    "1": 140,
                    "9": 130,
                    "10": 120,
                    "11": 110,
                    "12": 100,
                    "13": 85,
                    "14": 80,
                    "15": 75,
                    "16": 70
                },
                "DF": {
                    "1": 95,
                    "9": 90,
                    "10": 80,
                    "11": 70,
                    "12": 60
                }
            },
            "track_field_semi_10plus": {
                "DF": {
                    "1": 90,
                    "11": 85,
                    "12": 60
                    }
                },
            "distance_5000m_final": {
                "OW": {
                    "1": 305,
                    "2": 270,
                    "3": 240
                }
            },
            "distance_5000m_semi_max9": {},
            "distance_5000m_semi_10plus": {},
            "distance_10000m_final": {
                "OW": {
                    "1": 280,
                    "2": 250,
                    "3": 225
                }
            },
            "road_10km_final": {
                "OW": {
                    "1": 95,
                    "2": 85,
                    "3": 75
                }
            }
        }"#
    }

    #[test]
    fn test_calculator_initialization() {
        let json_data = get_test_json();
        let calculator = PlacementCalculator::new(json_data).unwrap();

        // Test track field final score
        assert_eq!(
            calculator.calculate_placement_score(
                EventGroup::TrackAndField,
                CompetitionCategory::OW,
                RoundType::Final,
                1,
                8
            ),
            Some(375)
        );
        // Test a random placement score
        assert_eq!(
            calculator.calculate_placement_score(
                EventGroup::Road10km,
                CompetitionCategory::OW,
                RoundType::Final,
                3,
                32,
            ),
            Some(75)
        );
        // test a semifinal score that does not advance to the final
        assert_eq!(
            calculator.calculate_placement_score(
                EventGroup::TrackAndField,
                CompetitionCategory::DF,
                RoundType::SemiFinal,
                11,
                10,
            ),
            Some(85)
        );
        // test a semifinal score where the athlete advances to the final
        assert_eq!(
            calculator.calculate_placement_score(
                EventGroup::TrackAndField,
                CompetitionCategory::DF,
                RoundType::SemiFinal,
                11,
                11,
            ),
            Some(90)
        );
        // test a semifinal score where the athlete advances to the final in an 8 person final
        assert_eq!(
            calculator.calculate_placement_score(
                EventGroup::TrackAndField,
                CompetitionCategory::OW,
                RoundType::SemiFinal,
                2,
                8,
            ),
            Some(140)
        );
    }
}
