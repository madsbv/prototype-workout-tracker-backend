# CI/CD
- [ ] Eventually set up CI through Github Actions, see e.g. https://github.com/jonhoo/rust-ci-conf/tree/main/.github for a possible template to use
- [ ] Once this turns web, we want CD

# Features
- [ ] CLI interface for importing and exporting data, statistics,...
- [ ]   Decide storage file for internal data via CLI flag (default to stdio if not set)
- [ ]   Print stats for all/selected workouts (how to select?)
- [ ]   CLI loop/REPL for importing external data and matching externally described exercises against internal ones/defining new internal ones
- [ ]       Once we start supporting multiple externals, turn "external data" into a trait and have the REPL take that trait as input.
- [ ]   Operation to get estimated RM stats based on time limited range of inputs, or only on sets with a certain range of reps
- [ ] List of default exercises saved in app
- [ ]   Should this just be some const Strings embedded in a separate Rust file, instead of reading and parsing CSV's on every load?
- [ ] Exercise aliases, for searching in app and parsing of outside data sources

# Data structure decisions
- [ ] How do we represent completed workouts vs workouts in progress vs templates?
- [ ] How do we deal with closely related variations, for example different bars used or doing the same machine exercise on different brands of machine?

