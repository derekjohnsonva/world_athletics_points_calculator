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

/// Retrieves the coefficients for a specific event and gender.
/// Returns `None` if the event or gender is not found.
pub fn get_coefficients(gender: Gender, event_name: &str) -> Option<Coefficients> {
    COEFFICIENTS.get().and_then(|table| {
        let gender_map = match gender {
            Gender::Men => &table.men.events,
            Gender::Women => &table.women.events,
        };
        gender_map
            .get(event_name)
            .map(|raw_coefficients| raw_coefficients.clone().into())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    // A minimal JSON string for testing parsing without relying on the file system
    const TEST_JSON_DATA: &str = r#"{
        "men": {
            "100m": [24.642211664166098, -837.7135408530303, 7119.3125116789015],
            "LJ": [1.931092872960562, 186.73134733641928, -479.70640445759636]
        },
        "women": {
            "100m": [9.927426450685289, -436.6751262119069, 4802.020943877404],
            "HJ": [39.557908744493034, 831.3655724464043, -601.5063267494843]
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

    /// Tests the integration of loading and retrieving coefficients using the public functions.
    #[test]
    fn test_get_coefficients_function_integration() {
        // Ensure the OnceCell is not already set from a previous test run
        // This is important for isolated test execution.
        // In a real test suite, you might use a test harness or clear the static.
        // For simplicity here, we'll just try to set it and ignore if already set.
        let _ = COEFFICIENTS.set(serde_json::from_str(TEST_JSON_DATA).unwrap());

        // Test retrieving men's LJ
        let men_lj_coefficients =
            get_coefficients(Gender::Men, "LJ").expect("Failed to get men's LJ coefficients");
        assert_approx_eq!(men_lj_coefficients.conversion_factor, 1.931092872960562);
        assert_approx_eq!(men_lj_coefficients.result_shift, 186.73134733641928);
        assert_approx_eq!(men_lj_coefficients.point_shift, -479.7064044575963);

        // Test retrieving women's 100m
        let women_100m_coefficients = get_coefficients(Gender::Women, "100m")
            .expect("Failed to get women's 100m coefficients");
        assert_approx_eq!(women_100m_coefficients.conversion_factor, 9.927426450685289);
        assert_approx_eq!(women_100m_coefficients.result_shift, -436.6751262119069);
        assert_approx_eq!(women_100m_coefficients.point_shift, 4802.020943877404);

        // Test a non-existent event for a specific gender
        assert!(get_coefficients(Gender::Men, "NonExistentEvent").is_none());
        assert!(get_coefficients(Gender::Women, "AnotherNonExistent").is_none());
    }
}
