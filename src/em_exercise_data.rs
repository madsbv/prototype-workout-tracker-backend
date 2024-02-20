use super::{Exercise, ExerciseType, Muscle};
use inflector::Inflector;
use itertools::Itertools;
use serde::{de, Deserialize};

pub fn parse_em_spec_csv_to_exercises(path: &str) -> anyhow::Result<Vec<Exercise>> {
    let mut em_rdr = csv::Reader::from_reader(std::fs::File::open(path)?);
    em_rdr
        .deserialize::<EmExerciseSpecification>()
        // It doesn't seem like this should be necessary
        .map(|em| Ok(em?.into()))
        .collect()
}

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

// Struct into which to parse the relevant parts of data/em_exercise_specs.csv
#[derive(Deserialize, Debug, PartialEq)]
struct EmExerciseSpecification {
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

#[cfg(test)]
mod tests {
    // use super::super::Exercise;
    use super::*;

    #[test]
    fn test_em_exercise_specs_parse() {
        for record in parse_em_spec_csv_to_exercises("test_data/em_exercise_specs.csv")
            .expect("test exercise specifications parse correctly")
        {
            // Every parsed exercise should train some muscle
            assert!(!record.muscles_trained.is_empty());
            // For every exercise type, we should track *something*
            assert!(
                record.tracking_config.weight
                    || record.tracking_config.time
                    || record.tracking_config.reps
                    || record.tracking_config.distance
            );
            // Every exercise should have a non-empty name
            assert!(!record.name.is_empty());
            // We shouldn't have any pinned notes on directly parsed data
            assert!(Option::is_none(&record.pinned_note));
        }
    }
}
