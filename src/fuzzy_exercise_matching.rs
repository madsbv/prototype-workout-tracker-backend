use super::*;
use itertools::Itertools;
use nucleo_matcher::{pattern::*, Matcher};

/// Search `exercise_list` for exercises whose name and exercise type together match `name`. Return list of possible matches according to fuzzy finder (nucleo), sorted with the best match first.
pub fn search_exercises<'a, 'b>(
    name: &'a str,
    exercise_list: &'b Vec<Exercise>,
) -> Vec<&'b Exercise> {
    let mut matcher = Matcher::new(nucleo_matcher::Config::DEFAULT);
    let pattern = Pattern::parse(
        &simplify_exercise_name(name),
        CaseMatching::Ignore,
        Normalization::Smart,
    );

    let mut buf = Vec::<char>::with_capacity(64);
    let mut exercises_with_scores: Vec<(&Exercise, u32)> = exercise_list
        .iter()
        .filter_map(|ex| {
            Some((
                ex,
                pattern.score(
                    nucleo_matcher::Utf32Str::new(
                        &simplify_exercise_name(&format!("{} {}", ex.name, ex.exercise_type)),
                        &mut buf,
                    ),
                    &mut matcher,
                )?,
            ))
        })
        .collect();

    exercises_with_scores.sort_unstable_by_key(|&(_, score)| -(score as i64));
    exercises_with_scores.iter().map(|&(ex, _)| ex).collect()
}

/// Get only the best match, if any.
pub fn identify_exercise<'a, 'b>(
    name: &'a str,
    exercise_list: &'b Vec<Exercise>,
) -> Option<&'b Exercise> {
    Some(search_exercises(name, exercise_list).get(0)?)
}

// Try to strip out symbols and standardize certain words to improve search results
// XXX: This would be a natural place to implement exercise name aliases as well.
fn simplify_exercise_name(name: &str) -> String {
    let symbols = [
        ',', '.', '(', ')', '[', ']', '{', '}', ';', ':', ' ', '-', '_',
    ];
    name.split(symbols).join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_exercise() {
        let mut strong_rdr = csv::Reader::from_reader(
            std::fs::File::open("test_data/strong_test_data.csv")
                .expect("File test_data/strong_test_data.csv exists and is readable"),
        );
        let strong_records: Vec<strong_data::StrongData> = strong_rdr.deserialize().map(|r| r.expect("Strong app test data deserializes correctly (i.e. test_actual_strong_data_deserializes passes)")).collect();

        let mut em_rdr = csv::Reader::from_reader(
            std::fs::File::open("test_data/em_exercise_specs.csv").expect("File is readable"),
        );
        let em_exercises: Vec<Exercise> = em_rdr.deserialize::<em_exercise_data::EmExerciseSpecification>().map(|em| em.expect("Exercise specifications test data parse correctly (i.e. test_em_exercise_specs_parse passes)").into()).collect();

        // A selection of records that we know matches some exercise in em_exercise_specs.
        // This lets us test the functionality of identify_exercise against a known good list of inputs.
        let curated_strong_records = [
            &strong_records[20],
            &strong_records[50],
            &strong_records[90],
        ];

        // Easily see which records we're testing with `cargo test -- --nocapture`
        for sr in dbg!(curated_strong_records) {
            let best_match = identify_exercise(&sr.exercise_name, &em_exercises);
            if Option::is_none(&best_match) {
                eprintln!("{sr:#?}");
            }
            assert!(Option::is_some(&best_match));
        }
        // TODO: We should eventually have enough pre-populated exercises to match all known exercises from Strong.
        // Once we have a more stable notion of our ultimate source of exercises, we should also run a version of this test against that source, and require that ALL exercises from Strong match against something.
    }
}
