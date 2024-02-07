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
