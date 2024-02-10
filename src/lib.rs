// TODO: Tighten up linting once csv parsing has been solved.
#![allow(dead_code)]
#![allow(unused_imports)]

struct Exercise {
    name: String,
    // Note: We track exerciseType for calculation purposes, but not for data collection decision making, which is controlled by tracking_config.
    // E.g. a "bodyweight" exercise should take into account the users weight for calculations, and volume computations for barbells vs dumbbells might be different.
    exercise_type: ExerciseType,
    // FEAT: Consider changing the data type to be some kind of list instead, to allow specifying multiple muscles, or even no muscles if desired.
    primary_muscle: Muscle,
    tracking_config: TrackingConfig,
    pinned_note: Option<String>,
}

// Struct to define which variables the user wants to track for each set of a given exercise.
// This does not hold actual data, but rather holds user-defined configuration for each exercise.
struct TrackingConfig {
    weight: bool,
    time: bool,
    reps: bool,
}

// FEAT: It might be useful to have "recursive" exercise types for things like interval/circuit training, or even just supersets. This could be implemented by having a Circuit variant of ExerciseType, which itself holds a list of exercises, and maybe some rest related metadata.
enum ExerciseType {
    // FEAT: Implement some way of specifying machines within each exercise? That probably shouldn't go in the exercise specification, but rather be part of the data recorded whenever an exercise is done. Or maybe associate to the Machine variant something like a list of Strings with the names of machines, and allow updating? It could just default to empty for no machine-specific tracking.
    Machine,
    Bodyweight,
    // TODO: Do I want to include data about different barbells as type information? It could be done similarly to the Machine feature discussed above. On the other hand, different bar types (meaning straight vs cambered vs EZ bar vs safety squat bar...) are kind of different exercises altogether, so maybe tracking them together is not worth the complexity?
    Barbell,
    Dumbbell,
}

// TODO: Fill out
enum Muscle {
    Bicep,
    Tricep,
}

// For recording a completed exercise in a workout, with associated tracking data
// Note: It seems enticing to try to encode the dependence of the ExerciseData on the exercise.tracking_variables in types, but that would probably be bad if we allow for modifying the specification of an exercise (though that has its own potential issues).
struct ExerciseWithData {
    exercise: Exercise,
    sets: Vec<ExerciseData>,
    note: Option<String>,
}

// Note: We make these option types to make clear the distinction between "unset" and "set to 0".
struct ExerciseData {
    weight: Option<f64>, // May be negative, e.g. for assisted bodyweight exercises
    time: Option<f64>,   // Should always be positive. Can we specify this in types?
    reps: Option<u64>,   // An entirely excessive number of reps
}

struct Workout {
    exercises: Vec<ExerciseWithData>,
    note: Option<String>,
}

// TODO: Define a "template" type? This comes later down the line.

// THOUGHTS: Workout creation methods will probably differ a lot by frontend, so maybe don't create functions for that here.
// Instead, work out data analytics functions, how to store data, some kind of data persistence solution, and so on.
// We could also create parsing libraries to import Strong data, for example.

// TODO: Move the Strong data parsing/import logic into a separate module. Export the StrongData type, then do the conversion to our format in a different module.
//
// Strong data import: The exported data does not contain exercise specifications. Write an import facility with user input that has the user specify each new exercise, then converts to internal format.
// For import purposes, we might also want an aliasing facility to let the names from apps differ from internal names.

use serde::Deserialize;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{fmt, str::FromStr};
use time::{OffsetDateTime, PrimitiveDateTime, Time};

// Use serde_with's derives to implement serialization and deserialization with the Display and FromStr traits.
#[derive(SerializeDisplay, DeserializeFromStr, Debug)]
pub struct StrongDuration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl fmt::Display for StrongDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        if self.hours != 0 {
            result += &format!("{}h", self.hours);
        }
        if self.minutes != 0 {
            result += &format!("{}m", self.minutes);
        }
        result += &format!("{}s", self.seconds);
        write!(f, "{}", result)
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
    Ok(s[0..chars.len() - 1]
        .parse()
        .map_err(|_| format!("Invalid number before suffix {suffix}"))?)
}

time::serde::format_description!(
    strong_date,
    PrimitiveDateTime,
    "[year]-[month]-[day] [hour]:[minute]:[second]"
);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StrongData {
    #[serde(with = "strong_date")]
    pub date: PrimitiveDateTime,
    // pub date: String,
    #[serde(rename = "Workout Name")]
    pub workout_name: String,
    // TODO: Parsing to Time doesn't work, fix
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
