# workout-tracker
This is the (beginnings of) backend code for what will eventually be a workout tracking app.

There are many of these out there already, but the ones I've tried all fall slightly short of being able to follow the workflow I want for my tracking, and the metrics I want to track and have computed and displayed during a workout. So I'm writing my own.

This code will provide backend data handling for the eventual app, some statistics/analysis tools accessible via CLI, and functionality to import workout data from various sources.

# Current features
- Defines internal data structures to keep track of data relevant to defining exercises and workouts, and tracking various metrics.
- Uses [[docs.rs/serde]] to support serializing and deserializing the internal data structures for storage.
- Has a predefined list of exercises to choose from (will later support user-defined exercises).
- Parses data exported from the Strong app on iOS, and has internal functionality to fuzzy search (using [[docs.rs/nucleo_matcher]]) a given list of previously defined exercises to match up exercise names from Strong with the internal exercise data structure.

# Planned features
- Compute and display various statistics from workout data, e.g. personal records, estimated 1 rep max weights.
- TUI loop to import external data, using user input to correctly match the external data to internally defined exercises.
