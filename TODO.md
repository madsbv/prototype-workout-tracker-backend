# Tests
- [x] Test the Strong data import:
- [x]   Turn the current manual deserialization test into an automated, built-in test. 
- [x]   Write tests for the StrongDuration Display and FromStr implementations

# CI/CD
- [ ] Eventually set up CI through Github Actions, see e.g. https://github.com/jonhoo/rust-ci-conf/tree/main/.github for a possible template to use
- [ ] Once this turns web, we want CD

# Features
- [ ] CLI loop to define exercise types
- [ ] Loop to define exercises for every Strong data entry
- [x] Serialization and deserialization of exercises and workouts
- [ ] Facilities to modify exercise types?
- [ ] List of default exercises saved in app
- [ ] Exercise aliases, for searching in app and parsing of outside data sources

# Data structure decisions
- [ ] How do we represent completed workouts vs workouts in progress vs templates?
- [ ] How do we deal with closely related variations, for example different bars used or doing the same machine exercise on different brands of machine?
- [ ] How do we handle names of exercises, and integrating names from different import mechanisms?
- [ ]   TODO: Use something like the nucleo or fzf-wrapped fuzzy matching crates to write a simple TUI loop to match Strong exercise names against em exercise specs. Test how well the matches work, and automate adding aliases to our exercise names using these matches.
- [ ]       Once this is working, try to do a full export from Strong with all exercises, and do matching against that to get a complete exercise list.

