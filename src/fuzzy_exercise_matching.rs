use super::Exercise;
use itertools::Itertools;
use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Matcher,
};

/// Search `exercise_list` for exercises whose name and exercise type together match `name`. Return list of possible matches according to fuzzy finder (nucleo), sorted with the best match first.
pub fn search_exercises<'b>(name: &str, exercise_list: &'b [Exercise]) -> Vec<&'b Exercise> {
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

    exercises_with_scores.sort_unstable_by_key(|&(_, score)| -i64::from(score));
    exercises_with_scores.iter().map(|&(ex, _)| ex).collect()
}

/// Get only the best match, if any.
pub fn identify_exercise<'b>(name: &str, exercise_list: &'b [Exercise]) -> Option<&'b Exercise> {
    Some(search_exercises(name, exercise_list).first()?)
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
    use super::identify_exercise;
    use crate::{em_exercise_data, strong_data};

    #[test]
    fn test_identify_exercise() {
        let strong_records =
            strong_data::parse_strong_csv_to_exercise_data("test_data/strong_test_data.csv")
                .expect("Strong app test data parses to valid StrongData structs");

        let em_exercises =
            em_exercise_data::parse_em_spec_csv_to_exercises("test_data/em_exercise_specs.csv")
                .expect("test exercise specifications parse correctly");

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
