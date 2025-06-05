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
    M300H,
    M400H,
    // Steeplechase
    M1500mSC,
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
    M50mSh,
    M55mSh,
    M60mSh,
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
    M4x100mSh,
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
    M10000mW,
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
            | Event::RaceWalking(RaceWalkingEvent::M10000mW)
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
                TrackAndFieldEvent::M50H => "50mH",
                TrackAndFieldEvent::M55H => "55mH",
                TrackAndFieldEvent::M60H => "60mH",
                TrackAndFieldEvent::M100H => "100mH", // Women's 100mH
                TrackAndFieldEvent::M110H => "110mH", // Men's 110mH
                TrackAndFieldEvent::M300H => "300mH",
                TrackAndFieldEvent::M400H => "400mH",
                TrackAndFieldEvent::M1500mSC => "1500m sh", // Assuming this is short track steeplechase
                TrackAndFieldEvent::M2000mSC => "2000m SC",
                TrackAndFieldEvent::M3000mSC => "3000m SC",
                TrackAndFieldEvent::M4x100m => "4x100m",
                TrackAndFieldEvent::M4x200m => "4x200m",
                TrackAndFieldEvent::M4x400m => "4x400m",
                TrackAndFieldEvent::M4x400mix => "4x400mix",
                TrackAndFieldEvent::LJ => "LJ",
                TrackAndFieldEvent::TJ => "TJ",
                TrackAndFieldEvent::HJ => "HJ",
                TrackAndFieldEvent::PV => "PV",
                TrackAndFieldEvent::SP => "SP",
                TrackAndFieldEvent::DT => "DT",
                TrackAndFieldEvent::HT => "HT",
                TrackAndFieldEvent::JT => "JT",
                TrackAndFieldEvent::M50mSh => "50m sh",
                TrackAndFieldEvent::M55mSh => "55m sh",
                TrackAndFieldEvent::M60mSh => "60m sh",
                TrackAndFieldEvent::M200mSh => "200m sh",
                TrackAndFieldEvent::M300mSh => "300m sh",
                TrackAndFieldEvent::M400mSh => "400m sh",
                TrackAndFieldEvent::M500mSh => "500m sh",
                TrackAndFieldEvent::M600mSh => "600m sh",
                TrackAndFieldEvent::M800mSh => "800m sh",
                TrackAndFieldEvent::M1000mSh => "1000m sh",
                TrackAndFieldEvent::M1500mSh => "1500m sh",
                TrackAndFieldEvent::M2000mSh => "2000m sh",
                TrackAndFieldEvent::M3000mSh => "3000m sh",
                TrackAndFieldEvent::M5000mSh => "5000m sh",
                TrackAndFieldEvent::MileSh => "Mile sh",
                TrackAndFieldEvent::M2MilesSh => "2 Miles sh",
                TrackAndFieldEvent::M4x100mSh => "4x100m sh",
                TrackAndFieldEvent::M4x200mSh => "4x200m sh",
                TrackAndFieldEvent::M4x400mSh => "4x400m sh",
                TrackAndFieldEvent::M4x400mixSh => "4x400mix sh",
            },
            Event::CombinedEvents(e) => match e {
                CombinedEvent::Dec => "Dec.",
                CombinedEvent::HeptSh => "Hept. sh",
                CombinedEvent::PentSh => "Pent. sh",
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
                RaceWalkingEvent::Road5kmW => "Road 5kmW",
                RaceWalkingEvent::Road10kmW => "Road 10kmW",
                RaceWalkingEvent::Road15kmW => "Road 15kmW",
                RaceWalkingEvent::Road20kmW => "Road 20kmW",
                RaceWalkingEvent::Road30kmW => "Road 30kmW",
                RaceWalkingEvent::Road35kmW => "Road 35kmW",
                RaceWalkingEvent::Road50kmW => "Road 50kmW",
                RaceWalkingEvent::M3000mW => "3000mW",
                RaceWalkingEvent::M5000mW => "5000mW",
                RaceWalkingEvent::M10000mW => "10000mW",
                RaceWalkingEvent::M15000mW => "15,000mW",
                RaceWalkingEvent::M20000mW => "20,000mW",
                RaceWalkingEvent::M30000mW => "30,000mW",
                RaceWalkingEvent::M35000mW => "35,000mW",
                RaceWalkingEvent::M50000mW => "50,000mW",
            },
            Event::CrossCountry(e) => match e {
                CrossCountryEvent::GenericXC => "GenericXC", // Placeholder for now
            },
        };
        write!(f, "{}", s)
    }
}

/// Enum to represent gender for clearer function signatures and data access.
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // Added Copy for easier use in arguments
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
