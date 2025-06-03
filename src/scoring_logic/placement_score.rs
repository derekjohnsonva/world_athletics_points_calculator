use crate::models::{CompetitionCategory, Event};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlacementScoreEventGroup {
    TrackAndField,        // Standard track & field events
    Distance5000m3000mSC, // 5000m and 3000mSC
    Distance10000m,       // 10,000m
    Road10km,             // 10km Road Race
    CombinedEvent,
    RoadMarathon,
    HalfMarathon,
    RoadRunning,
    RaceWalking20Km,
    RaceWalking35Km,
    RaceWalking35KmSimilar,
    CrossCountry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoundType {
    Final,
    SemiFinal,
    Other,
}

#[derive(Debug, Deserialize)]
struct PlacementScoreData {
    track_field_final: HashMap<CompetitionCategory, HashMap<i32, i32>>,
    track_field_semi_max9: HashMap<CompetitionCategory, HashMap<i32, i32>>,
    track_field_semi_10plus: HashMap<CompetitionCategory, HashMap<i32, i32>>,
    distance_5000m_3000mSC_final: HashMap<CompetitionCategory, HashMap<i32, i32>>,
    distance_5000m_3000mSC_semi_max9: HashMap<CompetitionCategory, HashMap<i32, i32>>,
    distance_5000m_3000mSC_semi_10plus: HashMap<CompetitionCategory, HashMap<i32, i32>>,
    distance_10000m_final: HashMap<CompetitionCategory, HashMap<i32, i32>>,
    road_10km_final: HashMap<CompetitionCategory, HashMap<i32, i32>>,
    combined_events: HashMap<CompetitionCategory, HashMap<i32, i32>>,
    road_marathon: HashMap<CompetitionCategory, HashMap<i32, i32>>, //TODO: figure out downhill course points
    half_marathon_similar_event: HashMap<CompetitionCategory, HashMap<i32, i32>>,
    road_running_event_group: HashMap<CompetitionCategory, HashMap<i32, i32>>,
    race_walking_20km: HashMap<CompetitionCategory, HashMap<i32, i32>>,
    race_walking_35km: HashMap<CompetitionCategory, HashMap<i32, i32>>,
    race_walking_30km_50km: HashMap<CompetitionCategory, HashMap<i32, i32>>,
    cross_country_finals: HashMap<CompetitionCategory, HashMap<i32, i32>>,
}

pub struct PlacementCalculator {
    data: PlacementScoreData,
}

static CALCULATOR: OnceLock<PlacementCalculator> = OnceLock::new();

pub struct PlacementScoreCalcInput {
    pub event: Event,
    pub competition_category: CompetitionCategory,
    pub round_type: RoundType,
    pub place: i32,
    pub qualified_to_final: bool,
    pub size_of_final: i32,
}

impl PlacementCalculator {
    fn new(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data: PlacementScoreData = serde_json::from_str(json_data)?;
        Ok(PlacementCalculator { data })
    }

    pub fn calculate_placement_score(&self, input: PlacementScoreCalcInput) -> Option<i32> {
        // If the athlete qualifies for the final, they get the same points as all other qualified athletes
        let place = if input.qualified_to_final && input.round_type == RoundType::SemiFinal {
            &1
        } else {
            &input.place
        };
        let event_group = input.event.to_placement_score_event_group();
        match (event_group, input.round_type) {
            (PlacementScoreEventGroup::TrackAndField, RoundType::Final) => self
                .data
                .track_field_final
                .get(&input.competition_category)?
                .get(place)
                .copied(),
            (PlacementScoreEventGroup::TrackAndField, RoundType::SemiFinal) => {
                // check to see which semifinal table to use
                if input.size_of_final <= 9 {
                    self.data
                        .track_field_semi_max9
                        .get(&input.competition_category)?
                        .get(place)
                        .copied()
                } else {
                    self.data
                        .track_field_semi_10plus
                        .get(&input.competition_category)?
                        .get(place)
                        .copied()
                }
            }
            (PlacementScoreEventGroup::Distance5000m3000mSC, RoundType::Final) => self
                .data
                .distance_5000m_3000mSC_final
                .get(&input.competition_category)?
                .get(place)
                .copied(),
            (PlacementScoreEventGroup::Distance5000m3000mSC, RoundType::SemiFinal) => {
                // check to see which semifinal table to use
                if input.size_of_final <= 9 {
                    self.data
                        .distance_5000m_3000mSC_semi_max9
                        .get(&input.competition_category)?
                        .get(place)
                        .copied()
                } else {
                    self.data
                        .distance_5000m_3000mSC_semi_10plus
                        .get(&input.competition_category)?
                        .get(place)
                        .copied()
                }
            }
            (PlacementScoreEventGroup::Distance10000m, RoundType::Final) => self
                .data
                .distance_10000m_final
                .get(&input.competition_category)?
                .get(&place)
                .copied(),
            (PlacementScoreEventGroup::Road10km, RoundType::Final) => self
                .data
                .road_10km_final
                .get(&input.competition_category)?
                .get(&place)
                .copied(),
            (PlacementScoreEventGroup::Distance10000m, RoundType::SemiFinal) => None,
            (PlacementScoreEventGroup::Road10km, RoundType::SemiFinal) => None,
            (PlacementScoreEventGroup::CombinedEvent, RoundType::Final) => self
                .data
                .combined_events
                .get(&input.competition_category)?
                .get(&place)
                .copied(),
            (PlacementScoreEventGroup::RoadMarathon, RoundType::Final) => self
                .data
                .road_marathon
                .get(&input.competition_category)?
                .get(&place)
                .copied(),
            (PlacementScoreEventGroup::HalfMarathon, RoundType::Final) => self
                .data
                .half_marathon_similar_event
                .get(&input.competition_category)?
                .get(&place)
                .copied(),
            (PlacementScoreEventGroup::RoadRunning, RoundType::Final) => self
                .data
                .road_running_event_group
                .get(&input.competition_category)?
                .get(&place)
                .copied(),
            (PlacementScoreEventGroup::RaceWalking20Km, RoundType::Final) => self
                .data
                .race_walking_20km
                .get(&input.competition_category)?
                .get(&place)
                .copied(),
            (PlacementScoreEventGroup::RaceWalking35Km, RoundType::Final) => self
                .data
                .race_walking_35km
                .get(&input.competition_category)?
                .get(&place)
                .copied(),
            (PlacementScoreEventGroup::RaceWalking35KmSimilar, RoundType::Final) => self
                .data
                .race_walking_30km_50km
                .get(&input.competition_category)?
                .get(&place)
                .copied(),
            (PlacementScoreEventGroup::CrossCountry, RoundType::Final) => self
                .data
                .cross_country_finals
                .get(&input.competition_category)?
                .get(&place)
                .copied(),
            (_, RoundType::SemiFinal) => None,
            (_, RoundType::Other) => None,
        }
    }
}

/// Initialize the placement calculator with JSON data
/// This should be called once at application startup
pub fn init_placement_score_calculator() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = include_str!("../../data/track_and_field_placement_scores.json");
    let calculator = PlacementCalculator::new(json_data)?;
    CALCULATOR
        .set(calculator)
        .map_err(|_| "Calculator already initialized")?;
    Ok(())
}

/// Calculate placement score for given parameters
/// Returns None if no score is available for the given combination
pub fn calculate_placement_score(input: PlacementScoreCalcInput) -> Option<i32> {
    CALCULATOR.get()?.calculate_placement_score(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{RoadRunningEvent, TrackAndFieldEvent};

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
            "distance_5000m_3000mSC_final": {
                "OW": {
                    "1": 305,
                    "2": 270,
                    "3": 240
                }
            },
            "distance_5000m_3000mSC_semi_max9": {},
            "distance_5000m_3000mSC_semi_10plus": {},
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
            },
            "combined_events":{},
            "road_marathon":{},
            "half_marathon_similar_event":{},
            "road_running_event_group": {},
            "race_walking_20km": {},
            "race_walking_35km":{} ,
            "race_walking_30km_50km": {},
            "cross_country_finals": {}
        }"#
    }

    #[test]
    fn test_calculator_initialization() {
        let json_data = get_test_json();
        let calculator = PlacementCalculator::new(json_data).unwrap();

        // Test track field final score
        assert_eq!(
            calculator.calculate_placement_score(PlacementScoreCalcInput {
                event: Event::TrackAndField(TrackAndFieldEvent::M100),
                competition_category: CompetitionCategory::OW,
                round_type: RoundType::Final,
                place: 1,
                qualified_to_final: true,
                size_of_final: 8,
            }),
            Some(375)
        );
        // Test a random placement score
        assert_eq!(
            calculator.calculate_placement_score(PlacementScoreCalcInput {
                event: Event::RoadRunning(RoadRunningEvent::Road10km),
                competition_category: CompetitionCategory::OW,
                round_type: RoundType::Final,
                place: 3,
                qualified_to_final: true,
                size_of_final: 32,
            }),
            Some(75)
        );
        // Test a semifinal score that does not advance to the final
        assert_eq!(
            calculator.calculate_placement_score(PlacementScoreCalcInput {
                event: Event::TrackAndField(TrackAndFieldEvent::M100),
                competition_category: CompetitionCategory::DF,
                round_type: RoundType::SemiFinal,
                place: 11,
                qualified_to_final: false,
                size_of_final: 10,
            }),
            Some(85)
        );
        // Test a semifinal score where the athlete advances to the final
        assert_eq!(
            calculator.calculate_placement_score(PlacementScoreCalcInput {
                event: Event::TrackAndField(TrackAndFieldEvent::M100),
                competition_category: CompetitionCategory::DF,
                round_type: RoundType::SemiFinal,
                place: 11,
                qualified_to_final: true,
                size_of_final: 11,
            }),
            Some(90)
        );
        // Test a semifinal score where the athlete advances to the final in an 8-person final
        assert_eq!(
            calculator.calculate_placement_score(PlacementScoreCalcInput {
                event: Event::TrackAndField(TrackAndFieldEvent::M100),
                competition_category: CompetitionCategory::OW,
                round_type: RoundType::SemiFinal,
                place: 2,
                qualified_to_final: true,
                size_of_final: 8,
            }),
            Some(140)
        );
    }
}
