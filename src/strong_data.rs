use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

pub fn parse_strong_csv_to_exercise_data(path: &str) -> anyhow::Result<Vec<StrongData>> {
    let mut strong_rdr = csv::Reader::from_reader(std::fs::File::open(path)?);
    strong_rdr
        .deserialize::<StrongData>()
        // It doesn't seem like this should be necessary
        .map(|s| Ok(s?))
        .collect()
}

// Relies on humantime_serde to serialize and deserialize timestamps and durations written in human-readable format
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StrongData {
    #[serde(with = "humantime_serde")]
    pub date: SystemTime,
    // pub date: String,
    #[serde(rename = "Workout Name")]
    pub workout_name: String,
    #[serde(with = "humantime_serde")]
    pub duration: Duration,
    // pub duration: String,
    #[serde(rename = "Exercise Name")]
    pub exercise_name: String,
    #[serde(rename = "Set Order")]
    pub set_order: u16,
    pub weight: f64,
    pub reps: u16,
    pub distance: f64,
    pub seconds: u16,
    pub notes: Option<String>,
    #[serde(rename = "Workout Notes")]
    pub workout_notes: Option<String>,
    #[serde(rename = "RPE")]
    pub rpe: Option<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use humantime;

    #[test]
    fn test_duration_parsing_and_display() {
        let string = "10h 30m";
        let duration = Duration::new(3600 * 10 + 60 * 30, 0);
        let duration_from_string = humantime::parse_duration(string).expect("String parses");
        assert_eq!(duration, duration_from_string);
        assert_eq!("10h 30m", humantime::format_duration(duration).to_string());

        let string = "15m";
        let duration = Duration::new(60 * 15, 0);
        let duration_from_string = humantime::parse_duration(string).expect("String parses");
        assert_eq!(duration, duration_from_string);
        assert_eq!("15m", humantime::format_duration(duration).to_string());

        let string = "26s";
        let duration = Duration::new(26, 0);
        let duration_from_string = humantime::parse_duration(string).expect("String parses");
        assert_eq!(duration, duration_from_string);
        assert_eq!("26s", humantime::format_duration(duration).to_string());
    }

    #[test]
    fn test_strong_data_csv_deserializes_correctly() {
        let test_input = r#"
Date,Workout Name,Duration,Exercise Name,Set Order,Weight,Reps,Distance,Seconds,Notes,Workout Notes,RPE
2023-07-14 15:30:00,"Morning Workout ",30m,"Incline Bench Press (Barbell)",1,29.48350405,5,0,0,"","",
"#;
        let mut reader = csv::Reader::from_reader(test_input.as_bytes());
        let mut deserializer = reader.deserialize::<StrongData>();
        let test_case = deserializer
            .next()
            .expect("CSV deserializes correctly")
            .expect("CSV string reads correctly");
        let test_none_case = deserializer.next();

        let data = StrongData {
            date: humantime::parse_rfc3339_weak("2023-07-14 15:30:00")
                .expect("this time format is valid"),
            workout_name: String::from("Morning Workout "),
            duration: Duration::new(30 * 60, 0),
            exercise_name: String::from("Incline Bench Press (Barbell)"),
            set_order: 1,
            weight: 29.483_504_05,
            reps: 5,
            distance: 0.0,
            seconds: 0,
            notes: None,
            workout_notes: None,
            rpe: None,
        };
        println!("{test_case:?}");
        assert_eq!(data, test_case);
        assert!(Option::is_none(&test_none_case));
    }

    #[test]
    fn test_actual_strong_data_deserializes() {
        let _ = parse_strong_csv_to_exercise_data("test_data/strong_test_data.csv")
            .expect("Strong app test data parses to valid StrongData structs");
    }
}
