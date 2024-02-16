use super::{Exercise, ExerciseType, Muscle};
use inflector::Inflector;
use itertools::Itertools;
use serde::{de, Deserialize};

impl From<EmExerciseSpecification> for Exercise {
    fn from(em: EmExerciseSpecification) -> Exercise {
        let name = em.exercise.to_sentence_case();
        let exercise_type = match &em.exercise_type as &str {
            "stretch" => ExerciseType::Stretch,
            "cardio" => ExerciseType::Cardio,
            "bodyweight" => ExerciseType::Bodyweight,
            "lift" => {
                if em.equipment_type.contains("dumbbell") {
                    ExerciseType::Dumbbell
                } else if em.equipment_type.contains("barbell") {
                    ExerciseType::Barbell
                } else if em.equipment_type.contains("kettlebell") {
                    ExerciseType::Kettlebell
                } else if em.equipment_type.contains("machine") {
                    ExerciseType::Machine
                } else {
                    // There's some stuff in the data that doesn't match our categories perfectly, e.g. back extensions with a weight plate. That's ok.
                    ExerciseType::Other
                }
            }
            _ => ExerciseType::Other,
        };

        let muscles_trained = em
            .muscle_groups
            .split(", ")
            .filter_map(|s| Muscle::try_from(s).ok())
            .unique()
            .collect();

        Exercise {
            name,
            tracking_config: exercise_type.default_tracking(),
            exercise_type,
            muscles_trained,
            pinned_note: None,
        }
    }
}

// Parse the relevant parts of data/clean_exercise_data.csv
#[derive(Deserialize, Debug, PartialEq)]
pub struct EmExerciseSpecification {
    exercise: String,
    #[serde(rename = "two sided")]
    #[serde(deserialize_with = "deserialize_bool_from_yes_no")]
    two_sided: bool,
    #[serde(rename = "exercise type")]
    exercise_type: String,
    #[serde(rename = "equipment")]
    #[serde(deserialize_with = "deserialize_bool_from_yes_no")]
    uses_equipment: bool,
    #[serde(rename = "equipment type")]
    equipment_type: String,
    #[serde(rename = "muscle groups")]
    muscle_groups: String,
}

fn deserialize_bool_from_yes_no<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["SI", "NO"])),
    }
}
