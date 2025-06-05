// src/scoring_logic/data_tables.rs
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::collections::HashMap;

use crate::models::Gender;

// This struct now represents the three coefficients in the array
#[derive(Debug, Deserialize, Clone)]
pub struct Coefficients {
    // These fields will be populated from the array elements
    pub conversion_factor: f64,
    pub result_shift: f64,
    pub point_shift: f64,
}

// A helper struct to correctly deserialize the [f64, f64, f64] array
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)] // Allows deserializing from multiple types, here specifically from an array
pub enum RawCoefficients {
    Array([f64; 3]), // For when it's an array of 3 floats
}

// Implement conversion from RawCoefficients to Coefficients
impl From<RawCoefficients> for Coefficients {
    fn from(raw: RawCoefficients) -> Self {
        match raw {
            RawCoefficients::Array([cf, rs, ps]) => Coefficients {
                conversion_factor: cf,
                result_shift: rs,
                point_shift: ps,
            },
        }
    }
}

// Represents the coefficients for a single gender (e.g., "men" or "women")
#[derive(Debug, Deserialize, Clone)]
pub struct GenderCoefficients {
    #[serde(flatten)] // This tells Serde to put all top-level keys into the HashMap
    pub events: HashMap<String, RawCoefficients>,
}

// The top-level structure of your JSON
#[derive(Debug, Deserialize, Clone)]
pub struct CoefficientsTable {
    pub men: GenderCoefficients,
    pub women: GenderCoefficients,
}

impl CoefficientsTable {
    /// Retrieves the coefficients for a specific event and gender.
    /// Returns `None` if the event or gender is not found.
    pub fn get_coefficients(&self, gender: Gender, event_name: &str) -> Option<Coefficients> {
        let gender_map = match gender {
            Gender::Men => &self.men.events,
            Gender::Women => &self.women.events,
        };
        gender_map
            .get(event_name)
            .map(|raw_coefficients| raw_coefficients.clone().into())
    }

    /// Calculates the points based on a result and the event-specific coefficients.
    ///
    /// The formula is: `points = floor(conversionFactor * (result + resultShift)^2 + pointShift)`
    ///
    /// # Arguments
    /// * `result` - The performance result in the standard unit (e.g., seconds for track, meters for field).
    /// * 'gender' - The gender of the competitor
    /// * 'event_name' - The events string name
    /// # Returns
    /// The calculated World Athletics points as a floored `f64`.
    pub fn calculate_result_score(
        &self,
        result: f64,
        gender: Gender,
        event_name: &str,
    ) -> Result<f64, String> {
        let coefficients = self.get_coefficients(gender, event_name).ok_or_else(|| {
            format!(
                "Coefficients not found for gender {:?} and event: {}",
                gender.to_string(),
                event_name,
            )
        })?;
        // points = floor(conversionFactor * (result + resultShift)^2 + pointShift)
        // coefficients[0] * x * x + coefficients[1] * x + coefficients[2]
        let raw_points = coefficients.conversion_factor * result * result
            + coefficients.result_shift * result
            + coefficients.point_shift;
        Ok(raw_points.round()) // Ensure the final points are floored
    }
}

pub fn calculate_result_score(
    result: f64,
    gender: Gender,
    event_name: &str,
) -> Result<f64, String> {
    let coefficients = COEFFICIENTS
        .get()
        .ok_or_else(|| "Coefficients not loaded. Call load_coefficients() first.".to_string())?;
    coefficients.calculate_result_score(result, gender, event_name)
}

// Global static for holding the loaded coefficients.
// Using OnceCell ensures it's initialized only once, safely.
static COEFFICIENTS: OnceCell<CoefficientsTable> = OnceCell::new();

/// Loads the World Athletics coefficients from the embedded JSON string.
/// This function should be called once at application startup.
pub fn load_coefficients() -> Result<(), String> {
    // The path assumes your JSON file is at the project root in a 'data' folder.
    // Ensure 'data/world_athletics_constants.json' exists relative to your Cargo.toml.
    let json_data = include_str!("../../data/world_athletics_constants_2025.json");

    let table: CoefficientsTable = serde_json::from_str(json_data)
        .map_err(|e| format!("Failed to parse coefficients JSON: {}", e))?;

    COEFFICIENTS
        .set(table)
        .map_err(|_| "Coefficients already loaded.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    // A minimal JSON string for testing parsing without relying on the file system
    const TEST_JSON_DATA: &str = r#"{
        "men": {
            "100m": [24.642211664166098, -837.7135408530303, 7119.3125116789015],
            "LJ": [1.931092872960562, 186.73134733641928, -479.70640445759636],
            "5000m": [0.002777997945427213,  -8.000608112196687,5760.418712362531]
        },
        "women": {
            "100m": [9.927426450685289, -436.6751262119069, 4802.020943877404],
            "HJ": [39.557908744493034, 831.3655724464043, -601.5063267494843],
            "LJ": [1.958114032649064, 193.69548254413166,-233.98988652729167]
        }
    }"#;

    /// Tests the direct parsing of a JSON string into the CoefficientsTable struct.
    #[test]
    fn test_json_parsing_direct() {
        let table: CoefficientsTable =
            serde_json::from_str(TEST_JSON_DATA).expect("Failed to parse test JSON data");

        // Test men's 100m
        let men_100m = table
            .men
            .events
            .get("100m")
            .expect("Men's 100m coefficients not found");
        let men_100m_coefficients: Coefficients = men_100m.clone().into();
        assert_approx_eq!(men_100m_coefficients.conversion_factor, 24.642211664166098);
        assert_approx_eq!(men_100m_coefficients.result_shift, -837.7135408530303);
        assert_approx_eq!(men_100m_coefficients.point_shift, 7119.3125116789015);

        // Test women's HJ
        let women_hj = table
            .women
            .events
            .get("HJ")
            .expect("Women's HJ coefficients not found");
        let women_hj_coefficients: Coefficients = women_hj.clone().into();
        assert_approx_eq!(women_hj_coefficients.conversion_factor, 39.557908744493034);
        assert_approx_eq!(women_hj_coefficients.result_shift, 831.3655724464043);
        assert_approx_eq!(women_hj_coefficients.point_shift, -601.5063267494843);

        // Test a non-existent event
        assert!(table.men.events.get("NonExistentEvent").is_none());
    }

    #[test]
    fn test_get_coefficients_function_integration() {
        let table: CoefficientsTable =
            serde_json::from_str(TEST_JSON_DATA).expect("Failed to parse test JSON data");

        // Test retrieving men's LJ
        let men_lj_coefficients = table
            .get_coefficients(Gender::Men, "LJ")
            .expect("Failed to get men's LJ coefficients");
        assert_approx_eq!(men_lj_coefficients.conversion_factor, 1.931092872960562);
        assert_approx_eq!(men_lj_coefficients.result_shift, 186.73134733641928);
        assert_approx_eq!(men_lj_coefficients.point_shift, -479.7064044575963);

        // Test retrieving women's 100m
        let women_100m_coefficients = table
            .get_coefficients(Gender::Women, "100m")
            .expect("Failed to get women's 100m coefficients");
        assert_approx_eq!(women_100m_coefficients.conversion_factor, 9.927426450685289);
        assert_approx_eq!(women_100m_coefficients.result_shift, -436.6751262119069);
        assert_approx_eq!(women_100m_coefficients.point_shift, 4802.020943877404);

        // Test a non-existent event for a specific gender
        assert!(table
            .get_coefficients(Gender::Men, "NonExistentEvent")
            .is_none());
        assert!(table
            .get_coefficients(Gender::Women, "AnotherNonExistent")
            .is_none());
    }

    #[test]
    fn test_calculate_placement_score() {
        let table: CoefficientsTable =
            serde_json::from_str(TEST_JSON_DATA).expect("Failed to parse test JSON data");

        // A Men's 100m result of 10.5 seconds should yield 1040.0 points
        let points = table.calculate_result_score(10.5, Gender::Men, "100m");
        assert!(points.is_ok());
        let points = points.unwrap();
        assert_approx_eq!(points, 1040.0);

        // A womens long jump of 6.5 meters should result in 1108.0 points
        let points = table.calculate_result_score(6.5, Gender::Women, "LJ");
        assert!(points.is_ok());
        let points = points.unwrap();
        assert_approx_eq!(points, 1108.0);

        // Test with a non-existent event
        let points = table.calculate_result_score(10.0, Gender::Men, "NonExistentEvent");
        assert!(points.is_err());

        // Test with a 5k value of 14 minutes (840 seconds) that should yield 1000.0 points
        let points = table.calculate_result_score(840.0, Gender::Men, "5000m");
        assert!(points.is_ok());
        let points = points.unwrap();
        assert_approx_eq!(points, 1000.0);
    }
}
