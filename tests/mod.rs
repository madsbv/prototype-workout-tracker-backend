use std::str::FromStr;
use workout_tracker::{StrongData, StrongDuration};

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

#[test]
fn test_actual_strong_data_deserializes() {
    let mut rdr = csv::Reader::from_reader(
        std::fs::File::open("tests/strong_test_data.csv")
            .expect("File tests/strong_test_data.csv exists and is readable"),
    );
    for result in rdr.deserialize() {
        let _: StrongData =
            result.expect("Actual CSV exported from Strong app deserializes validly");
    }
}
