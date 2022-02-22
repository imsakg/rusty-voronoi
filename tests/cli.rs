// Behavioral cli tests.
use rusty_voronoi::cli::*;
#[test] fn parse_viewport_algorithm_and_help() {let o=Options::parse(["--width","320","--height","240","--algorithm","fortune","--output","cells.svg","--labels"].into_iter().map(str::to_string)).unwrap();assert_eq!((o.width,o.height),(320,240));assert_eq!(o.algorithm,"fortune");assert!(o.labels);assert!(Options::parse(["--help".into()]).unwrap().help);assert!(help().contains("Usage:"));assert!(Options::parse(["--wat".into()]).is_err());}
