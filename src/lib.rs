#![warn(clippy::all, clippy::pedantic)]
// #![warn(clippy::cargo)]
#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::struct_excessive_bools
)]

use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

// TODO: Think about how to structure the library and its public API.
pub mod default_exercise_list;
pub mod em_exercise_data;
pub mod fuzzy_exercise_matching;
pub mod strong_data;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Exercise {
    // TODO: How do we encode aliases for exercises, especially in such a way that we can easily change the preferred name?
    name: String,
    // Note: We track exerciseType for calculation purposes, but not for data collection decision making, which is controlled by tracking_config.
    // E.g. a "bodyweight" exercise should take into account the users weight for calculations, and volume computations for barbells vs dumbbells might be different.
    exercise_type: ExerciseType,
    muscles_trained: Vec<Muscle>,
    tracking_config: TrackingConfig,
    pinned_note: Option<String>,
}

// Struct to define which variables the user wants to track for each set of a given exercise.
// This does not hold actual data, but rather holds user-defined configuration for each exercise.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TrackingConfig {
    weight: bool,
    time: bool,
    reps: bool,
    distance: bool,
}

// TODO: Implement default tracking schemes for different exercise types.

// FEAT: It might be useful to have "recursive" exercise types for things like interval/circuit training, or even just supersets. This could be implemented by having a Circuit variant of ExerciseType, which itself holds a list of exercises, and maybe some rest related metadata.
#[derive(Serialize, Deserialize, Debug, PartialEq, EnumIter)]
enum ExerciseType {
    // FEAT: Implement some way of specifying machines within each exercise? That probably shouldn't go in the exercise specification, but rather be part of the data recorded whenever an exercise is done. Or maybe associate to the Machine variant something like a list of Strings with the names of machines, and allow updating? It could just default to empty for no machine-specific tracking.
    Machine,
    Bodyweight,
    // TODO: Do I want to include data about different barbells as type information? It could be done similarly to the Machine feature discussed above. On the other hand, different bar types (meaning straight vs cambered vs EZ bar vs safety squat bar...) are kind of different exercises altogether, so maybe tracking them together is not worth the complexity?
    // It would be good to be able to keep variations of bar type close to each other, but I'm not sure how to encode this. Maybe we just do good search instead.
    Barbell,
    Dumbbell,
    Kettlebell,
    Stretch,
    Cardio,
    // Fallback variant
    Other,
}

impl std::fmt::Display for ExerciseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            ExerciseType::Machine => "Machine",
            ExerciseType::Bodyweight => "Bodyweight",
            ExerciseType::Barbell => "Barbell",
            ExerciseType::Dumbbell => "Dumbbell",
            ExerciseType::Kettlebell => "Kettlebell",
            ExerciseType::Stretch => "Stretch",
            ExerciseType::Cardio => "Cardio",
            ExerciseType::Other => "Other",
        }
        .to_string();
        write!(f, "{result}")
    }
}

impl ExerciseType {
    pub fn default_tracking(&self) -> TrackingConfig {
        match self {
            Self::Machine
            | Self::Bodyweight
            | Self::Barbell
            | Self::Dumbbell
            | Self::Kettlebell
            | Self::Other => TrackingConfig {
                weight: true,
                time: false,
                reps: true,
                distance: false,
            },
            Self::Cardio => TrackingConfig {
                weight: false,
                time: true,
                reps: false,
                distance: true,
            },
            Self::Stretch => TrackingConfig {
                weight: false,
                time: true,
                reps: false,
                distance: false,
            },
        }
    }
}

// Encodes individual muscles very finely grained. If we want to work with coarser data later, we can define a `MuscleGroup` enum implementing From<Muscle>.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, EnumIter, Clone)]
enum Muscle {
    Abs,
    Biceps,
    Calves,
    Chest,
    Forearms,
    FrontDelts,
    Glutes,
    Hamstrings,
    HipAbductors,
    HipAdductors,
    HipFlexors,
    Lats,
    Obliques,
    Quads,
    RearDelts,
    RotatorCuffs,
    SideDelts,
    SpinalErectors,
    Traps,
    Triceps,
}

impl TryFrom<&str> for Muscle {
    type Error = String;

    // Take a best guess at the muscle described by s.
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        for muscle in Muscle::iter() {
            if muscle.get_aliases().contains(&s.to_string()) {
                return Ok(muscle);
            }
        }
        Err(format!("Muscle not found: {s}"))
    }
}

impl Muscle {
    fn get_aliases(&self) -> Vec<String> {
        match self {
            Muscle::Abs => vec!["abs", "abdominals"],
            Muscle::Biceps => vec!["biceps"],
            Muscle::Calves => vec!["calves", "calf"],
            Muscle::Chest => vec!["chest", "pecs", "pectorals"],
            Muscle::Forearms => vec![
                "forearms",
                "fingers",
                "wrists",
                "forearm flexors",
                "forearm extensors",
            ],
            Muscle::FrontDelts => vec!["front deltoid", "front delts"],
            Muscle::Glutes => vec!["glutes"],
            Muscle::Hamstrings => vec!["hamstrings"],
            Muscle::HipAbductors => vec!["abductors", "hip abductors"],
            Muscle::HipAdductors => vec!["adductors", "hip adductors"],
            Muscle::HipFlexors => vec!["hip flexors", "psoas"],
            Muscle::Lats => vec!["lats"],
            Muscle::Obliques => vec!["obliques"],
            Muscle::Quads => vec!["quadriceps", "quads"],
            Muscle::RearDelts => vec!["rear delts", "rear deltoids"],
            Muscle::RotatorCuffs => vec!["rotator cuffs"],
            Muscle::SideDelts => vec!["side delts", "side deltoids"],
            Muscle::SpinalErectors => vec!["spinal erectors", "erector spinae", "lower back"],
            Muscle::Traps => vec!["traps", "trapezius"],
            Muscle::Triceps => vec!["triceps"],
        }
        .iter()
        .map(ToString::to_string)
        .collect()
    }
}

// For recording a completed exercise in a workout, with associated tracking data
// Note: It seems enticing to try to encode the dependence of the ExerciseData on the exercise.tracking_variables in types, but that would probably be bad if we allow for modifying the specification of an exercise (though that has its own potential issues).
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ExerciseWithData {
    exercise: Exercise,
    sets: Vec<ExerciseData>,
    note: Option<String>,
}

// Note: We make these option types to make clear the distinction between "unset" and "set to 0".
// TODO: How do we handle units? Default to metric and convert as needed (with rounding)? We have to make sure that the way we handle rounding doesn't cause weirdness
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ExerciseData {
    weight: Option<f64>,   // May be negative, e.g. for assisted bodyweight exercises
    time: Option<f64>,     // Should always be positive. Can we specify this in types?
    reps: Option<u64>,     // An entirely excessive number of reps
    distance: Option<f64>, // Should always be positive. Can we specify this in types?
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Workout {
    exercises: Vec<ExerciseWithData>,
    note: Option<String>,
}

// TODO: Define a "template" type? This comes later down the line.

// THOUGHTS: Workout creation methods will probably differ a lot by frontend, so maybe don't create functions for that here.
// Instead, work out data analytics functions, how to store data, some kind of data persistence solution, and so on.
