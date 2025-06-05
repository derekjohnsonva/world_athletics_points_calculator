use world_athletics_points_calulator::models::*;
use world_athletics_points_calulator::scoring_logic::placement_score::RoundType;

#[cfg(test)]
mod performance_input_integration_tests {
    use super::*;

    #[test]
    fn test_time_based_event_performance_input() {
        // Test 100m event (time-based)
        let event = Event::TrackAndField(TrackAndFieldEvent::M100);
        assert_eq!(event.performance_type(), PerformanceType::Time);
        
        // Test various time formats for 100m
        assert!((Event::parse_time_to_seconds("10.50").unwrap() - 10.50).abs() < 0.001);
        assert!((Event::parse_time_to_seconds("9.58").unwrap() - 9.58).abs() < 0.001);
        
        // Test 1500m event with mm:ss format
        let event_1500 = Event::TrackAndField(TrackAndFieldEvent::M1500);
        assert_eq!(event_1500.performance_type(), PerformanceType::Time);
        assert!((Event::parse_time_to_seconds("3:45.67").unwrap() - 225.67).abs() < 0.001);
        
        // Test marathon with hh:mm:ss format
        let marathon = Event::RoadRunning(RoadRunningEvent::RoadMarathon);
        assert_eq!(marathon.performance_type(), PerformanceType::Time);
        assert!((Event::parse_time_to_seconds("2:15:30.50").unwrap() - 8130.50).abs() < 0.001);
    }

    #[test]
    fn test_distance_based_event_performance_input() {
        // Test field events are distance-based
        let long_jump = Event::TrackAndField(TrackAndFieldEvent::LJ);
        assert_eq!(long_jump.performance_type(), PerformanceType::Distance);
        
        let shot_put = Event::TrackAndField(TrackAndFieldEvent::SP);
        assert_eq!(shot_put.performance_type(), PerformanceType::Distance);
        
        let high_jump = Event::TrackAndField(TrackAndFieldEvent::HJ);
        assert_eq!(high_jump.performance_type(), PerformanceType::Distance);
        
        let pole_vault = Event::TrackAndField(TrackAndFieldEvent::PV);
        assert_eq!(pole_vault.performance_type(), PerformanceType::Distance);
        
        let triple_jump = Event::TrackAndField(TrackAndFieldEvent::TJ);
        assert_eq!(triple_jump.performance_type(), PerformanceType::Distance);
        
        let discus = Event::TrackAndField(TrackAndFieldEvent::DT);
        assert_eq!(discus.performance_type(), PerformanceType::Distance);
        
        let hammer = Event::TrackAndField(TrackAndFieldEvent::HT);
        assert_eq!(hammer.performance_type(), PerformanceType::Distance);
        
        let javelin = Event::TrackAndField(TrackAndFieldEvent::JT);
        assert_eq!(javelin.performance_type(), PerformanceType::Distance);
    }

    #[test]
    fn test_world_athletics_score_input_with_time_parsing() {
        // Test creating a WorldAthleticsScoreInput with parsed time
        let event = Event::TrackAndField(TrackAndFieldEvent::M100);
        let performance_time = "10.50";
        let parsed_performance = Event::parse_time_to_seconds(performance_time).unwrap();
        
        let input = WorldAthleticsScoreInput {
            gender: Gender::Men,
            event,
            performance: parsed_performance,
            wind_speed: Some(1.5),
            net_downhill: None,
            placement_info: None,
        };
        
        assert!((input.performance - 10.50).abs() < 0.001);
        assert_eq!(input.wind_speed, Some(1.5));
    }

    #[test]
    fn test_world_athletics_score_input_with_distance() {
        // Test creating a WorldAthleticsScoreInput with distance measurement
        let event = Event::TrackAndField(TrackAndFieldEvent::LJ);
        let distance_meters = 8.95; // Long jump distance in meters
        
        let input = WorldAthleticsScoreInput {
            gender: Gender::Men,
            event,
            performance: distance_meters,
            wind_speed: Some(0.5), // Wind still matters for long jump
            net_downhill: None,
            placement_info: None,
        };
        
        assert!((input.performance - 8.95).abs() < 0.001);
        assert_eq!(input.wind_speed, Some(0.5));
    }

    #[test]
    fn test_time_string_conversion_roundtrip() {
        // Test that we can convert time to string and back
        let original_seconds = 8130.50; // 2:15:30.50
        let time_string = Event::seconds_to_time_string(original_seconds);
        let parsed_back = Event::parse_time_to_seconds(&time_string).unwrap();
        
        assert!((original_seconds - parsed_back).abs() < 0.001);
    }

    #[test]
    fn test_various_event_types_performance_classification() {
        // Verify all track events are time-based
        assert_eq!(Event::TrackAndField(TrackAndFieldEvent::M200).performance_type(), PerformanceType::Time);
        assert_eq!(Event::TrackAndField(TrackAndFieldEvent::M400H).performance_type(), PerformanceType::Time);
        assert_eq!(Event::TrackAndField(TrackAndFieldEvent::M3000mSC).performance_type(), PerformanceType::Time);
        assert_eq!(Event::TrackAndField(TrackAndFieldEvent::M4x100m).performance_type(), PerformanceType::Time);
        
        // Verify road running events are time-based
        assert_eq!(Event::RoadRunning(RoadRunningEvent::Road10km).performance_type(), PerformanceType::Time);
        assert_eq!(Event::RoadRunning(RoadRunningEvent::RoadHM).performance_type(), PerformanceType::Time);
        
        // Verify race walking events are time-based
        assert_eq!(Event::RaceWalking(RaceWalkingEvent::Road20kmW).performance_type(), PerformanceType::Time);
        assert_eq!(Event::RaceWalking(RaceWalkingEvent::M10000mW).performance_type(), PerformanceType::Time);
        
        // Verify combined events are time-based (individual events within would vary, but the overall scoring is points)
        assert_eq!(Event::CombinedEvents(CombinedEvent::Dec).performance_type(), PerformanceType::Time);
        assert_eq!(Event::CombinedEvents(CombinedEvent::Hept).performance_type(), PerformanceType::Time);
    }

    #[test]
    fn test_invalid_time_formats() {
        // Test various invalid time formats
        assert!(Event::parse_time_to_seconds("").is_err());
        assert!(Event::parse_time_to_seconds("invalid").is_err());
        assert!(Event::parse_time_to_seconds("1:2:3:4").is_err());
        assert!(Event::parse_time_to_seconds("ab:cd").is_err());
        assert!(Event::parse_time_to_seconds("1:ab:cd").is_err());
    }

    #[test]
    fn test_placement_info_toggle() {
        // Test creating WorldAthleticsScoreInput with placement info
        let event = Event::TrackAndField(TrackAndFieldEvent::M100);
        let performance = 10.50;
        
        let input_with_placement = WorldAthleticsScoreInput {
            gender: Gender::Men,
            event: event.clone(),
            performance,
            wind_speed: Some(1.5),
            net_downhill: None,
            placement_info: Some(PlacementInfo {
                competition_category: CompetitionCategory::A,
                place: 1,
                round: RoundType::Final,
                size_of_final: 8,
                qualified_to_final: true,
            }),
        };
        
        // Test creating WorldAthleticsScoreInput without placement info
        let input_without_placement = WorldAthleticsScoreInput {
            gender: Gender::Men,
            event,
            performance,
            wind_speed: Some(1.5),
            net_downhill: None,
            placement_info: None,
        };
        
        // Verify placement info is present/absent as expected
        assert!(input_with_placement.placement_info.is_some());
        assert!(input_without_placement.placement_info.is_none());
        
        // Verify other fields are the same
        assert_eq!(input_with_placement.gender, input_without_placement.gender);
        assert_eq!(input_with_placement.event, input_without_placement.event);
        assert!((input_with_placement.performance - input_without_placement.performance).abs() < 0.001);
        assert_eq!(input_with_placement.wind_speed, input_without_placement.wind_speed);
        assert_eq!(input_with_placement.net_downhill, input_without_placement.net_downhill);
    }
}