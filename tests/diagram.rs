// Behavioral diagram tests.
use rusty_voronoi::{Diagram,geometry::{Point,Bounds}};
fn bounds()->Bounds {Bounds::new(0.,0.,10.,10.).unwrap()}
#[test] fn one_site_owns_the_entire_viewport() {let d=Diagram::build(vec![Point::new(3.,4.)],bounds()).unwrap();assert_eq!(d.cells().len(),1);assert_eq!(d.cells()[0].area(),100.);assert_eq!(d.cells()[0].site,0);assert_eq!(d.bounds(),bounds());}
#[test] fn empty_diagram_has_no_cells() {assert!(Diagram::build(vec![],bounds()).unwrap().cells().is_empty());}
