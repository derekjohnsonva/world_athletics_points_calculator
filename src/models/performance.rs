use serde::{Deserialize, Serialize};

use crate::scoring_logic::placement_score::{PlacementScoreEventGroup, RoundType};

// src/models/performance.rs
/// Represents events typically categorized under Track & Field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackAndFieldEvent {
    // Sprints/Middle Distance/Long Distance
    M50,
    M55,
    M60,
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
    M50m_sh,
    M55m_sh,
    M60m_sh,
    M200m_sh,
    M300m_sh,
    M400m_sh,
    M500m_sh,
    M600m_sh,
    M800m_sh,
    M1000m_sh,
    M1500m_sh,
    M2000m_sh,
    M3000m_sh,
    M5000m_sh,
    Mile_sh,
    M2Miles_sh, // Mile and 2 Miles on short track
    M4x100m_sh,
    M4x200m_sh,
    M4x400m_sh,
    M4x400mix_sh,
}

/// Represents Combined Events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CombinedEvent {
    Dec,     // Decathlon
    Hept,    // Heptathlon
    Hept_sh, // Heptathlon (short track/indoor component)
    Pent_sh, // Pentathlon (short track/indoor component)
}

/// Represents Road Running Events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoadRunningEvent {
    Road5km,
    Road10km,
    Road15km,
    Road20km,
    Road25km,
    Road30km,
    RoadHM,
    RoadMarathon,
    Road10Miles,
    RoadMile,
}

/// Represents Race Walking Events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RaceWalkingEvent {
    Road5kmW,
    Road10kmW,
    Road15kmW,
    Road20kmW,
    Road30kmW,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrossCountryEvent {
    // Add specific Cross Country event variants here as needed.
    // For now, leaving it empty as no specific XC events were in the provided JSON.
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

impl Event {
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

impl ToString for Event {
    /// Converts the Event enum variant into its string representation
    /// which matches the keys in your JSON constants table.
    fn to_string(&self) -> String {
        match self {
            Event::TrackAndField(e) => match e {
                TrackAndFieldEvent::M50 => "50m".to_string(),
                TrackAndFieldEvent::M55 => "55m".to_string(),
                TrackAndFieldEvent::M60 => "60m".to_string(),
                TrackAndFieldEvent::M100 => "100m".to_string(),
                TrackAndFieldEvent::M200 => "200m".to_string(),
                TrackAndFieldEvent::M300 => "300m".to_string(),
                TrackAndFieldEvent::M400 => "400m".to_string(),
                TrackAndFieldEvent::M500 => "500m".to_string(),
                TrackAndFieldEvent::M600 => "600m".to_string(),
                TrackAndFieldEvent::M800 => "800m".to_string(),
                TrackAndFieldEvent::M1000 => "1000m".to_string(),
                TrackAndFieldEvent::M1500 => "1500m".to_string(),
                TrackAndFieldEvent::M2000 => "2000m".to_string(),
                TrackAndFieldEvent::M3000 => "3000m".to_string(),
                TrackAndFieldEvent::M5000 => "5000m".to_string(),
                TrackAndFieldEvent::M10000 => "10000m".to_string(),
                TrackAndFieldEvent::M50H => "50mH".to_string(),
                TrackAndFieldEvent::M55H => "55mH".to_string(),
                TrackAndFieldEvent::M60H => "60mH".to_string(),
                TrackAndFieldEvent::M100H => "100mH".to_string(), // Women's 100mH
                TrackAndFieldEvent::M110H => "110mH".to_string(), // Men's 110mH
                TrackAndFieldEvent::M300H => "300mH".to_string(),
                TrackAndFieldEvent::M400H => "400mH".to_string(),
                TrackAndFieldEvent::M1500mSC => "1500m sh".to_string(), // Assuming this is short track steeplechase
                TrackAndFieldEvent::M2000mSC => "2000m SC".to_string(),
                TrackAndFieldEvent::M3000mSC => "3000m SC".to_string(),
                TrackAndFieldEvent::M4x100m => "4x100m".to_string(),
                TrackAndFieldEvent::M4x200m => "4x200m".to_string(),
                TrackAndFieldEvent::M4x400m => "4x400m".to_string(),
                TrackAndFieldEvent::M4x400mix => "4x400mix".to_string(),
                TrackAndFieldEvent::LJ => "LJ".to_string(),
                TrackAndFieldEvent::TJ => "TJ".to_string(),
                TrackAndFieldEvent::HJ => "HJ".to_string(),
                TrackAndFieldEvent::PV => "PV".to_string(),
                TrackAndFieldEvent::SP => "SP".to_string(),
                TrackAndFieldEvent::DT => "DT".to_string(),
                TrackAndFieldEvent::HT => "HT".to_string(),
                TrackAndFieldEvent::JT => "JT".to_string(),
                TrackAndFieldEvent::M50m_sh => "50m sh".to_string(),
                TrackAndFieldEvent::M55m_sh => "55m sh".to_string(),
                TrackAndFieldEvent::M60m_sh => "60m sh".to_string(),
                TrackAndFieldEvent::M200m_sh => "200m sh".to_string(),
                TrackAndFieldEvent::M300m_sh => "300m sh".to_string(),
                TrackAndFieldEvent::M400m_sh => "400m sh".to_string(),
                TrackAndFieldEvent::M500m_sh => "500m sh".to_string(),
                TrackAndFieldEvent::M600m_sh => "600m sh".to_string(),
                TrackAndFieldEvent::M800m_sh => "800m sh".to_string(),
                TrackAndFieldEvent::M1000m_sh => "1000m sh".to_string(),
                TrackAndFieldEvent::M1500m_sh => "1500m sh".to_string(),
                TrackAndFieldEvent::M2000m_sh => "2000m sh".to_string(),
                TrackAndFieldEvent::M3000m_sh => "3000m sh".to_string(),
                TrackAndFieldEvent::M5000m_sh => "5000m sh".to_string(),
                TrackAndFieldEvent::Mile_sh => "Mile sh".to_string(),
                TrackAndFieldEvent::M2Miles_sh => "2 Miles sh".to_string(),
                TrackAndFieldEvent::M4x100m_sh => "4x100m sh".to_string(),
                TrackAndFieldEvent::M4x200m_sh => "4x200m sh".to_string(),
                TrackAndFieldEvent::M4x400m_sh => "4x400m sh".to_string(),
                TrackAndFieldEvent::M4x400mix_sh => "4x400mix sh".to_string(),
            },
            Event::CombinedEvents(e) => match e {
                CombinedEvent::Dec => "Dec.".to_string(),
                CombinedEvent::Hept_sh => "Hept. sh".to_string(),
                CombinedEvent::Pent_sh => "Pent. sh".to_string(),
                CombinedEvent::Hept => "Hept.".to_string(),
            },
            Event::RoadRunning(e) => match e {
                RoadRunningEvent::Road5km => "Road 5 km".to_string(),
                RoadRunningEvent::Road10km => "Road 10 km".to_string(),
                RoadRunningEvent::Road15km => "Road 15 km".to_string(),
                RoadRunningEvent::Road20km => "Road 20 km".to_string(),
                RoadRunningEvent::Road25km => "Road 25 km".to_string(),
                RoadRunningEvent::Road30km => "Road 30 km".to_string(),
                RoadRunningEvent::RoadHM => "Road HM".to_string(),
                RoadRunningEvent::RoadMarathon => "Road Marathon".to_string(),
                RoadRunningEvent::Road10Miles => "Road 10 Miles".to_string(),
                RoadRunningEvent::RoadMile => "Road Mile".to_string(),
            },
            Event::RaceWalking(e) => match e {
                RaceWalkingEvent::Road5kmW => "Road 5kmW".to_string(),
                RaceWalkingEvent::Road10kmW => "Road 10kmW".to_string(),
                RaceWalkingEvent::Road15kmW => "Road 15kmW".to_string(),
                RaceWalkingEvent::Road20kmW => "Road 20kmW".to_string(),
                RaceWalkingEvent::Road30kmW => "Road 30kmW".to_string(),
                RaceWalkingEvent::Road35kmW => "Road 35kmW".to_string(),
                RaceWalkingEvent::Road50kmW => "Road 50kmW".to_string(),
                RaceWalkingEvent::M3000mW => "3000mW".to_string(),
                RaceWalkingEvent::M5000mW => "5000mW".to_string(),
                RaceWalkingEvent::M10000mW => "10000mW".to_string(),
                RaceWalkingEvent::M15000mW => "15,000mW".to_string(),
                RaceWalkingEvent::M20000mW => "20,000mW".to_string(),
                RaceWalkingEvent::M30000mW => "30,000mW".to_string(),
                RaceWalkingEvent::M35000mW => "35,000mW".to_string(),
                RaceWalkingEvent::M50000mW => "50,000mW".to_string(),
            },
            Event::CrossCountry(e) => match e {
                CrossCountryEvent::GenericXC => "GenericXC".to_string(), // Placeholder for now
            },
        }
    }
}

/// Enum to represent gender for clearer function signatures and data access.
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // Added Copy for easier use in arguments
pub enum Gender {
    Men,
    Women,
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        match self {
            Gender::Men => "men".to_string(),
            Gender::Women => "women".to_string(),
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CompetitionCategory {
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
    pub placement_info: Option<PlacementInfo>,
}
