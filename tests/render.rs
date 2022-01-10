// Behavioral render tests.
use rusty_voronoi::{Diagram,geometry::{Point,Bounds},render::*};
fn diagram()->Diagram {Diagram::build(vec![Point::new(2.,3.),Point::new(8.,7.)],Bounds::new(0.,0.,10.,10.).unwrap()).unwrap()}
#[test] fn svg_has_cells_sites_and_a_valid_root() {let svg=svg(&diagram()).unwrap();assert!(svg.starts_with("<svg xmlns="));assert_eq!(svg.matches("<polygon ").count(),2);assert_eq!(svg.matches("<circle ").count(),2);assert!(svg.ends_with("</svg>"));}
