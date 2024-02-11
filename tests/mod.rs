use std::str::FromStr;
use workout_tracker::{StrongData, StrongDuration};

#[test]
fn test_strong_duration_parsing() {
    let string = "10h 30m";
    let duration = StrongDuration {
        hours: 10,
        minutes: 30,
        seconds: 0,
    };
    assert_eq!(Ok(duration), StrongDuration::from_str(string));
    let string = "15m";
    let duration = StrongDuration {
        hours: 0,
        minutes: 15,
        seconds: 0,
    };
    assert_eq!(Ok(duration), StrongDuration::from_str(string));
    let string = "26s";
    let duration = StrongDuration {
        hours: 0,
        minutes: 0,
        seconds: 26,
    };
    assert_eq!(Ok(duration), StrongDuration::from_str(string));
}

#[test]
fn test_strong_data_csv_deserialize() {
    let test_input = r#"
Date,Workout Name,Duration,Exercise Name,Set Order,Weight,Reps,Distance,Seconds,Notes,Workout Notes,RPE
2023-07-14 15:30:00,"Morning Workout ",30m,"Incline Bench Press (Barbell)",1,29.48350405,5,0,0,"","",
"#;
    let mut reader = csv::Reader::from_reader(test_input.as_bytes());
    let mut deserializer = reader.deserialize::<StrongData>();
    let test_case = deserializer
        .next()
        .expect("CSV deserializing failed")
        .expect("CSV reading failed");
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
        weight: 29.48350405,
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
