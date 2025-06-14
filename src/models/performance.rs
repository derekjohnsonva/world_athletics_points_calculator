use crate::scoring_logic::placement_score::{PlacementScoreEventGroup, RoundType};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// src/models/performance.rs
/// Represents events typically categorized under Track & Field.
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Default)]
pub enum TrackAndFieldEvent {
    // Sprints/Middle Distance/Long Distance
    M50,
    M55,
    M60,
    #[default]
    M100,
    M200,
    M300,
    M400,
    M500,
    M600,
    M800,
    M1000,
    M1500,
    M2000,
    M3000,
    M5000,
    M10000,
    // Hurdles
    M50H,
    M55H,
    M60H,
    M100H,
    M110H,
    // M300H,
    M400H,
    // Steeplechase
    M2000mSC,
    M3000mSC,
    // Relays
    M4x100m,
    M4x200m,
    M4x400m,
    M4x400mix,
    // Field Events
    LJ,
    TJ,
    HJ,
    PV,
    SP,
    DT,
    HT,
    JT,
    // Indoor/Short Track specific events (often denoted by 'sh' in JSON)
    // M50mSh,
    // M55mSh,
    // M60mSh,
    M200mSh,
    M300mSh,
    M400mSh,
    M500mSh,
    M600mSh,
    M800mSh,
    M1000mSh,
    M1500mSh,
    M2000mSh,
    M3000mSh,
    M5000mSh,
    MileSh,
    M2MilesSh, // Mile and 2 Miles on short track
    // M4x100mSh,
    M4x200mSh,
    M4x400mSh,
    M4x400mixSh,
}

/// Represents Combined Events.
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Default)]
pub enum CombinedEvent {
    #[default]
    Dec, // Decathlon
    Hept,   // Heptathlon
    HeptSh, // Heptathlon (short track/indoor component)
    PentSh, // Pentathlon (short track/indoor component)
}

/// Represents Road Running Events.
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Default)]
pub enum RoadRunningEvent {
    Road5km,
    Road10km,
    Road15km,
    Road20km,
    Road25km,
    Road30km,
    RoadHM,
    #[default]
    RoadMarathon,
    Road10Miles,
    RoadMile,
}

/// Represents Race Walking Events.
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Default)]
pub enum RaceWalkingEvent {
    Road5kmW,
    Road10kmW,
    Road15kmW,
    Road20kmW,
    Road30kmW,
    #[default]
    Road35kmW,
    Road50kmW,
    M3000mW,
    M5000mW,
    // M10000mW,
    M15000mW,
    M20000mW,
    M30000mW,
    M35000mW,
    M50000mW, // Track walks
}

/// Represents Cross Country Events.
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Default)]
pub enum CrossCountryEvent {
    // Add specific Cross Country event variants here as needed.
    // For now, leaving it empty as no specific XC events were in the provided JSON.
    #[default]
    GenericXC, // Placeholder
}

/// A combined enum for all supported events, categorized by World Athletics sections.
/// This will be used in the `WorldAthleticsScoreInput` to specify the event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    TrackAndField(TrackAndFieldEvent),
    CombinedEvents(CombinedEvent),
    RoadRunning(RoadRunningEvent),
    RaceWalking(RaceWalkingEvent),
    CrossCountry(CrossCountryEvent),
}

impl Default for Event {
    fn default() -> Self {
        Event::TrackAndField(TrackAndFieldEvent::M100)
    }
}

impl Event {
    pub fn all_variants() -> Vec<Event> {
        let mut events = Vec::new();
        for track_and_field_event in TrackAndFieldEvent::iter() {
            events.push(Event::TrackAndField(track_and_field_event));
        }
        for combined_event in CombinedEvent::iter() {
            events.push(Event::CombinedEvents(combined_event));
        }
        for road_running_event in RoadRunningEvent::iter() {
            events.push(Event::RoadRunning(road_running_event));
        }
        for race_walking_event in RaceWalkingEvent::iter() {
            events.push(Event::RaceWalking(race_walking_event));
        }
        for cross_country_event in CrossCountryEvent::iter() {
            events.push(Event::CrossCountry(cross_country_event));
        }
        events
    }

    // Convert from string back to enum (for form handling)
    pub fn from_string(s: &str) -> Option<Event> {
        Event::all_variants()
            .into_iter()
            .find(|variant| variant.to_string() == s)
    }

    /// Determines whether this event is measured by time or distance
    pub fn performance_type(&self) -> PerformanceType {
        match self {
            // Field events are measured in meters/distance
            Event::TrackAndField(TrackAndFieldEvent::LJ)
            | Event::TrackAndField(TrackAndFieldEvent::TJ)
            | Event::TrackAndField(TrackAndFieldEvent::HJ)
            | Event::TrackAndField(TrackAndFieldEvent::PV)
            | Event::TrackAndField(TrackAndFieldEvent::SP)
            | Event::TrackAndField(TrackAndFieldEvent::DT)
            | Event::TrackAndField(TrackAndFieldEvent::HT)
            | Event::TrackAndField(TrackAndFieldEvent::JT) => PerformanceType::Distance,

            // All other events are time-based
            _ => PerformanceType::Time,
        }
    }

    pub fn to_placement_score_event_group(&self) -> PlacementScoreEventGroup {
        match self {
            Event::TrackAndField(TrackAndFieldEvent::M5000)
            | Event::TrackAndField(TrackAndFieldEvent::M3000mSC) => {
                PlacementScoreEventGroup::Distance5000m3000mSC
            }

            Event::TrackAndField(TrackAndFieldEvent::M10000) => {
                PlacementScoreEventGroup::Distance10000m
            }
            Event::RoadRunning(RoadRunningEvent::Road10km) => PlacementScoreEventGroup::Road10km,
            Event::RoadRunning(RoadRunningEvent::RoadMarathon) => {
                PlacementScoreEventGroup::RoadMarathon
            }
            Event::RoadRunning(RoadRunningEvent::RoadHM) // TODO: Determine what to do when the half marathon is the Main Event
            | Event::RoadRunning(RoadRunningEvent::Road30km)
            | Event::RoadRunning(RoadRunningEvent::Road25km) => {
                PlacementScoreEventGroup::HalfMarathon
            }
            Event::RaceWalking(RaceWalkingEvent::M20000mW)
            | Event::RaceWalking(RaceWalkingEvent::Road20kmW)
            | Event::RaceWalking(RaceWalkingEvent::Road5kmW)
            | Event::RaceWalking(RaceWalkingEvent::Road10kmW)
            | Event::RaceWalking(RaceWalkingEvent::Road15kmW)
            | Event::RaceWalking(RaceWalkingEvent::M3000mW)
            | Event::RaceWalking(RaceWalkingEvent::M5000mW)
            // | Event::RaceWalking(RaceWalkingEvent::M10000mW)
            | Event::RaceWalking(RaceWalkingEvent::M15000mW) => {
                PlacementScoreEventGroup::RaceWalking20Km
            },
            Event::RaceWalking(RaceWalkingEvent::M35000mW) | Event::RaceWalking(RaceWalkingEvent::Road35kmW) => {
                PlacementScoreEventGroup::RaceWalking35Km
            },
            Event::RaceWalking(_) => PlacementScoreEventGroup::RaceWalking35KmSimilar,
            Event::TrackAndField(_) => PlacementScoreEventGroup::TrackAndField,
            Event::CombinedEvents(_) => PlacementScoreEventGroup::CombinedEvent,
            Event::RoadRunning(_) => PlacementScoreEventGroup::RoadRunning,
            Event::CrossCountry(_) => PlacementScoreEventGroup::CrossCountry,
        }
    }
}

impl fmt::Display for Event {
    /// Converts the Event enum variant into its string representation
    /// which matches the keys in your JSON constants table.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Event::TrackAndField(e) => match e {
                TrackAndFieldEvent::M50 => "50m",
                TrackAndFieldEvent::M55 => "55m",
                TrackAndFieldEvent::M60 => "60m",
                TrackAndFieldEvent::M100 => "100m",
                TrackAndFieldEvent::M200 => "200m",
                TrackAndFieldEvent::M300 => "300m",
                TrackAndFieldEvent::M400 => "400m",
                TrackAndFieldEvent::M500 => "500m",
                TrackAndFieldEvent::M600 => "600m",
                TrackAndFieldEvent::M800 => "800m",
                TrackAndFieldEvent::M1000 => "1000m",
                TrackAndFieldEvent::M1500 => "1500m",
                TrackAndFieldEvent::M2000 => "2000m",
                TrackAndFieldEvent::M3000 => "3000m",
                TrackAndFieldEvent::M5000 => "5000m",
                TrackAndFieldEvent::M10000 => "10000m",
                TrackAndFieldEvent::M50H => "50m Hurdle",
                TrackAndFieldEvent::M55H => "55m Hurdle",
                TrackAndFieldEvent::M60H => "60m Hurdle",
                TrackAndFieldEvent::M100H => "100m Hurdle", // Women's 100mH
                TrackAndFieldEvent::M110H => "110m Hurdle", // Men's 110mH
                // TrackAndFieldEvent::M300H => "300m Hurdle",
                TrackAndFieldEvent::M400H => "400m Hurdle",
                TrackAndFieldEvent::M2000mSC => "2000m SC",
                TrackAndFieldEvent::M3000mSC => "3000m SC",
                TrackAndFieldEvent::M4x100m => "4x100m",
                TrackAndFieldEvent::M4x200m => "4x200m",
                TrackAndFieldEvent::M4x400m => "4x400m",
                TrackAndFieldEvent::M4x400mix => "4x400mix",
                TrackAndFieldEvent::LJ => "Long Jump",
                TrackAndFieldEvent::TJ => "Triple Jump",
                TrackAndFieldEvent::HJ => "High Jump",
                TrackAndFieldEvent::PV => "Pole Vault",
                TrackAndFieldEvent::SP => "Shot Put",
                TrackAndFieldEvent::DT => "Discus Throw",
                TrackAndFieldEvent::HT => "Hammer Throw",
                TrackAndFieldEvent::JT => "Javelin Throw",
                // TrackAndFieldEvent::M50mSh => "50m short track",
                // TrackAndFieldEvent::M55mSh => "55m short track",
                // TrackAndFieldEvent::M60mSh => "60m short track",
                TrackAndFieldEvent::M200mSh => "200m short track",
                TrackAndFieldEvent::M300mSh => "300m short track",
                TrackAndFieldEvent::M400mSh => "400m short track",
                TrackAndFieldEvent::M500mSh => "500m short track",
                TrackAndFieldEvent::M600mSh => "600m short track",
                TrackAndFieldEvent::M800mSh => "800m short track",
                TrackAndFieldEvent::M1000mSh => "1000m short track",
                TrackAndFieldEvent::M1500mSh => "1500m short track",
                TrackAndFieldEvent::M2000mSh => "2000m short track",
                TrackAndFieldEvent::M3000mSh => "3000m short track",
                TrackAndFieldEvent::M5000mSh => "5000m short track",
                TrackAndFieldEvent::MileSh => "Mile short track",
                TrackAndFieldEvent::M2MilesSh => "2 Miles short track",
                // TrackAndFieldEvent::M4x100mSh => "4x100m short track",
                TrackAndFieldEvent::M4x200mSh => "4x200m short track",
                TrackAndFieldEvent::M4x400mSh => "4x400m short track",
                TrackAndFieldEvent::M4x400mixSh => "4x400mix short track",
            },
            Event::CombinedEvents(e) => match e {
                CombinedEvent::Dec => "Dec.",
                CombinedEvent::HeptSh => "Hept. short track",
                CombinedEvent::PentSh => "Pent. short track",
                CombinedEvent::Hept => "Hept.",
            },
            Event::RoadRunning(e) => match e {
                RoadRunningEvent::Road5km => "Road 5 km",
                RoadRunningEvent::Road10km => "Road 10 km",
                RoadRunningEvent::Road15km => "Road 15 km",
                RoadRunningEvent::Road20km => "Road 20 km",
                RoadRunningEvent::Road25km => "Road 25 km",
                RoadRunningEvent::Road30km => "Road 30 km",
                RoadRunningEvent::RoadHM => "Road HM",
                RoadRunningEvent::RoadMarathon => "Road Marathon",
                RoadRunningEvent::Road10Miles => "Road 10 Miles",
                RoadRunningEvent::RoadMile => "Road Mile",
            },
            Event::RaceWalking(e) => match e {
                RaceWalkingEvent::Road5kmW => "Road 5km Walk",
                RaceWalkingEvent::Road10kmW => "Road 10km Walk",
                RaceWalkingEvent::Road15kmW => "Road 15km Walk",
                RaceWalkingEvent::Road20kmW => "Road 20km Walk",
                RaceWalkingEvent::Road30kmW => "Road 30km Walk",
                RaceWalkingEvent::Road35kmW => "Road 35km Walk",
                RaceWalkingEvent::Road50kmW => "Road 50km Walk",
                RaceWalkingEvent::M3000mW => "3000m Walk",
                RaceWalkingEvent::M5000mW => "5000m Walk",
                // RaceWalkingEvent::M10000mW => "10000m Walk",
                RaceWalkingEvent::M15000mW => "15,000m Walk",
                RaceWalkingEvent::M20000mW => "20,000m Walk",
                RaceWalkingEvent::M30000mW => "30,000m Walk",
                RaceWalkingEvent::M35000mW => "35,000m Walk",
                RaceWalkingEvent::M50000mW => "50,000m Walk",
            },
            Event::CrossCountry(e) => match e {
                CrossCountryEvent::GenericXC => "GenericXC", // Placeholder for now
            },
        };
        write!(f, "{}", s)
    }
}

/// Enum to represent the type of performance measurement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceType {
    /// Time-based events (running, hurdles, etc.) measured in seconds
    Time,
    /// Distance/height-based field events measured in meters
    Distance,
}

/// Enum to represent gender for clearer function signatures and data access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)] // Added Copy for easier use in arguments
pub enum Gender {
    Men,
    Women,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gender::Men => write!(f, "men"),
            Gender::Women => write!(f, "women"),
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default, EnumIter)]
pub enum CompetitionCategory {
    #[default]
    /// Other competitions
    F,
    /// International Matches
    E,
    /// World Athletics Continental Tour Challenger series
    D,
    /// World Athletics Continental Tour Bronze Meetings
    C,
    /// World Athletics Continental Tour Silver Meetings
    B,
    /// Major Games and Gold Meetings
    A,
    /// Area Senior Outdoor Championships
    GL,
    /// Minor Championships
    GW,
    /// Diamond League Finals*
    DF,
    /// Worlds and Olympics
    OW,
}

impl CompetitionCategory {
    pub fn from_string(s: &str) -> Option<CompetitionCategory> {
        CompetitionCategory::iter().find(|variant| variant.to_string() == s)
    }
}

impl fmt::Display for CompetitionCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompetitionCategory::F => write!(f, "F"),
            CompetitionCategory::E => write!(f, "E"),
            CompetitionCategory::D => write!(f, "D"),
            CompetitionCategory::C => write!(f, "C"),
            CompetitionCategory::B => write!(f, "B"),
            CompetitionCategory::A => write!(f, "A"),
            CompetitionCategory::GL => write!(f, "GL"),
            CompetitionCategory::GW => write!(f, "GW"),
            CompetitionCategory::DF => write!(f, "DF"),
            CompetitionCategory::OW => write!(f, "OW"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlacementInfo {
    pub competition_category: CompetitionCategory,
    pub place: i32,
    pub round: RoundType,
    /// The size of the final impacts how the prelim is scored
    pub size_of_final: i32,
    pub qualified_to_final: bool,
}
/// Represents the input data required to calculate a World Athletics Score.
#[derive(Debug, Clone)]
pub struct WorldAthleticsScoreInput {
    pub gender: Gender,
    pub event: Event,
    pub performance: f64,
    /// For events affected by wind (e.g., sprints, jumps)
    pub wind_speed: Option<f64>,
    /// For road running events, net elevation drop in m/km (if > 1.0 m/km)
    pub net_downhill: Option<f64>,
    pub placement_info: Option<PlacementInfo>,
}

/// Utility functions for time parsing and conversion
impl Event {
    /// Parse time string in various formats (hh:mm:ss.mmm, mm:ss.mmm, ss.mmm) to seconds
    pub fn parse_time_to_seconds(time_str: &str) -> Result<f64, String> {
        let time_str = time_str.trim();

        // Split by colons to determine format
        let parts: Vec<&str> = time_str.split(':').collect();

        match parts.len() {
            // Format: ss.mmm or ss
            1 => parts[0]
                .parse::<f64>()
                .map_err(|_| format!("Invalid seconds format: {}", time_str)),
            // Format: mm:ss.mmm or mm:ss
            2 => {
                let minutes = parts[0]
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid minutes: {}", parts[0]))?;
                let seconds = parts[1]
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid seconds: {}", parts[1]))?;
                Ok(minutes * 60.0 + seconds)
            }
            // Format: hh:mm:ss.mmm or hh:mm:ss
            3 => {
                let hours = parts[0]
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid hours: {}", parts[0]))?;
                let minutes = parts[1]
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid minutes: {}", parts[1]))?;
                let seconds = parts[2]
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid seconds: {}", parts[2]))?;
                Ok(hours * 3600.0 + minutes * 60.0 + seconds)
            }
            _ => Err(format!(
                "Invalid time format: {}. Expected formats: ss.mmm, mm:ss.mmm, or hh:mm:ss.mmm",
                time_str
            )),
        }
    }

    /// Convert seconds back to time string format (mm:ss.mmm or hh:mm:ss.mmm)
    pub fn seconds_to_time_string(seconds: f64) -> String {
        if seconds < 3600.0 {
            // Less than an hour, use mm:ss.mmm format
            let minutes = (seconds / 60.0).floor();
            let remaining_seconds = seconds - (minutes * 60.0);
            format!("{:02.0}:{:06.3}", minutes, remaining_seconds)
        } else {
            // Hour or more, use hh:mm:ss.mmm format
            let hours = (seconds / 3600.0).floor();
            let remaining_minutes = ((seconds - (hours * 3600.0)) / 60.0).floor();
            let remaining_seconds = seconds - (hours * 3600.0) - (remaining_minutes * 60.0);
            format!(
                "{:02.0}:{:02.0}:{:06.3}",
                hours, remaining_minutes, remaining_seconds
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_parse_time_to_seconds() {
        // Test seconds only
        assert!((Event::parse_time_to_seconds("10.50").unwrap() - 10.50).abs() < 0.001);
        assert!((Event::parse_time_to_seconds("9.58").unwrap() - 9.58).abs() < 0.001);

        // Test mm:ss format
        assert!((Event::parse_time_to_seconds("1:30.25").unwrap() - 90.25).abs() < 0.001);
        assert!((Event::parse_time_to_seconds("3:45.67").unwrap() - 225.67).abs() < 0.001);

        // Test hh:mm:ss format
        assert!((Event::parse_time_to_seconds("2:15:30.50").unwrap() - 8130.50).abs() < 0.001);
        assert!((Event::parse_time_to_seconds("1:00:00.00").unwrap() - 3600.00).abs() < 0.001);

        // Test error cases
        assert!(Event::parse_time_to_seconds("invalid").is_err());
        assert!(Event::parse_time_to_seconds("1:2:3:4").is_err());
        assert!(Event::parse_time_to_seconds("").is_err());
    }

    #[test]
    fn test_seconds_to_time_string() {
        // Test less than an hour
        assert_eq!(Event::seconds_to_time_string(10.50), "00:10.500");
        assert_eq!(Event::seconds_to_time_string(90.25), "01:30.250");
        assert_eq!(Event::seconds_to_time_string(225.67), "03:45.670");

        // Test an hour or more
        assert_eq!(Event::seconds_to_time_string(3600.0), "01:00:00.000");
        assert_eq!(Event::seconds_to_time_string(8130.50), "02:15:30.500");
    }

    #[test]
    fn test_performance_type() {
        // Test field events return Distance
        assert_eq!(
            Event::TrackAndField(TrackAndFieldEvent::LJ).performance_type(),
            PerformanceType::Distance
        );
        assert_eq!(
            Event::TrackAndField(TrackAndFieldEvent::SP).performance_type(),
            PerformanceType::Distance
        );
        assert_eq!(
            Event::TrackAndField(TrackAndFieldEvent::HJ).performance_type(),
            PerformanceType::Distance
        );

        // Test track events return Time
        assert_eq!(
            Event::TrackAndField(TrackAndFieldEvent::M100).performance_type(),
            PerformanceType::Time
        );
        assert_eq!(
            Event::TrackAndField(TrackAndFieldEvent::M400H).performance_type(),
            PerformanceType::Time
        );
        assert_eq!(
            Event::RoadRunning(RoadRunningEvent::RoadMarathon).performance_type(),
            PerformanceType::Time
        );
    }

    #[test]
    fn test_all_enum_events_must_exist_in_json() {
        // This test ensures ALL events defined in enums exist in JSON constants
        let json_content = include_str!("../../data/world_athletics_constants_2025.json");
        let json_data: Value =
            serde_json::from_str(json_content).expect("Failed to parse JSON constants file");

        let men_events = json_data["men"]
            .as_object()
            .expect("Men's section not found");
        let women_events = json_data["women"]
            .as_object()
            .expect("Women's section not found");

        // Get all enum events and check each one
        let all_events = Event::all_variants();
        let mut missing_events = Vec::new();

        for event in all_events {
            let event_string = event.to_string();

            // Skip cross country events as they might be placeholders
            if matches!(event, Event::CrossCountry(_)) {
                continue;
            }

            // Determine expected gender availability based on event type
            let should_be_in_men = match event_string.as_str() {
                "100m Hurdle" | "Hept." | "Pent. short track" => false, // Women only
                _ => true, // Should be in men's constants
            };

            let should_be_in_women = match event_string.as_str() {
                "110m Hurdle" | "Dec." | "Hept. short track" => false, // Men only
                _ => true, // Should be in women's constants
            };

            let in_men = men_events.contains_key(&event_string);
            let in_women = women_events.contains_key(&event_string);

            if should_be_in_men && !in_men {
                missing_events.push(format!("Missing from men's constants: {}", event_string));
            }
            if should_be_in_women && !in_women {
                missing_events.push(format!("Missing from women's constants: {}", event_string));
            }
        }

        // Fail the test if any events are missing
        if !missing_events.is_empty() {
            panic!(
                "The following events are defined in enums but missing from JSON constants:\n{}\n\
                All enum events must have corresponding entries in world_athletics_constants_2025.json",
                missing_events.join("\n")
            );
        }
    }
}
