use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::fmt;
use std::str::FromStr;
use time::PrimitiveDateTime;

pub fn parse_strong_csv_to_exercise_data(path: &str) -> anyhow::Result<Vec<StrongData>> {
    let mut strong_rdr = csv::Reader::from_reader(std::fs::File::open(path)?);
    strong_rdr
        .deserialize::<StrongData>()
        // It doesn't seem like this should be necessary
        .map(|s| Ok(s?))
        .collect()
}

// Use serde_with's derives to implement serialization and deserialization with the Display and FromStr traits.
#[derive(SerializeDisplay, DeserializeFromStr, Debug, PartialEq, PartialOrd)]
pub struct StrongDuration {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl fmt::Display for StrongDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts: Vec<String> = vec![];
        if self.hours != 0 {
            parts.push(format!("{}h", self.hours));
        }
        if self.minutes != 0 {
            parts.push(format!("{}m", self.minutes));
        }
        parts.push(format!("{}s", self.seconds));
        let result = parts.join(" ");
        write!(f, "{result}")
    }
}

impl FromStr for StrongDuration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let mut hours = 0;
        let mut minutes = 0;
        let mut seconds = 0;
        match parts.len() {
            3 => {
                hours = parse_suffixed_integer(parts[0], 'h')?;
                minutes = parse_suffixed_integer(parts[1], 'm')?;
                seconds = parse_suffixed_integer(parts[2], 's')?;
            }
            2 => {
                hours = parse_suffixed_integer(parts[0], 'h')?;
                minutes = parse_suffixed_integer(parts[1], 'm')?;
            }
            1 => {
                if let Ok(s) = parse_suffixed_integer(parts[0], 's') {
                    seconds = s;
                } else {
                    minutes = parse_suffixed_integer(parts[0], 'm')?;
                }
            }
            _ => {
                return Err(format!(
                    "The following duration specifier has {} parts, should have 1 or 2.\n {s}",
                    parts.len()
                ));
            }
        };
        if seconds >= 60 {
            let extra_minutes = seconds / 60;
            minutes += extra_minutes;
            seconds -= extra_minutes * 60;
        }
        if minutes >= 60 {
            let extra_hours = minutes / 60;
            hours += extra_hours;
            minutes -= extra_hours * 60;
        }
        Ok(StrongDuration {
            hours,
            minutes,
            seconds,
        })
    }
}

fn parse_suffixed_integer(s: &str, suffix: char) -> Result<u32, <StrongDuration as FromStr>::Err> {
    let chars = s.chars().collect::<Vec<_>>();
    if chars[chars.len() - 1] != suffix {
        return Err(format!("Invalid suffix, should be {suffix}"));
    }
    s[0..chars.len() - 1]
        .parse()
        .map_err(|_| format!("Invalid number before suffix {suffix}"))
}

time::serde::format_description!(
    strong_date,
    PrimitiveDateTime,
    "[year]-[month]-[day] [hour]:[minute]:[second]"
);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StrongData {
    #[serde(with = "strong_date")]
    pub date: PrimitiveDateTime,
    // pub date: String,
    #[serde(rename = "Workout Name")]
    pub workout_name: String,
    pub duration: StrongDuration,
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

    #[test]
    fn test_strong_duration_parsing_and_display() {
        let string = "10h 30m";
        let duration = StrongDuration {
            hours: 10,
            minutes: 30,
            seconds: 0,
        };
        let duration_from_string = StrongDuration::from_str(string).expect("String parses");
        assert_eq!(duration, duration_from_string);
        assert_eq!("10h 30m 0s", duration.to_string());

        let string = "15m";
        let duration = StrongDuration {
            hours: 0,
            minutes: 15,
            seconds: 0,
        };
        let duration_from_string = StrongDuration::from_str(string).expect("String parses");
        assert_eq!(duration, duration_from_string);
        assert_eq!("15m 0s", duration.to_string());

        let string = "26s";
        let duration = StrongDuration {
            hours: 0,
            minutes: 0,
            seconds: 26,
        };
        let duration_from_string = StrongDuration::from_str(string).expect("String parses");
        assert_eq!(duration, duration_from_string);
        assert_eq!("26s", duration.to_string());
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
            date: time::macros::datetime!(2023-07-14 15:30:00),
            workout_name: String::from("Morning Workout "),
            duration: StrongDuration {
                hours: 0,
                minutes: 30,
                seconds: 0,
            },
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
