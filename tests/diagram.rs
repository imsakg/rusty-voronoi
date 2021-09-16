// Behavioral diagram tests.
#[test] fn duplicate_sites_do_not_double_count_area() {let a=Point::new(2.,5.);let d=Diagram::build(vec![a,Point::new(8.,5.),a],bounds()).unwrap();assert_eq!(d.sites().len(),2);assert_eq!(d.cells().iter().map(|c|c.area()).sum::<f64>(),100.);}
#[test] fn invalid_sites_are_rejected() {assert!(Diagram::build(vec![Point::new(f64::NAN,0.)],bounds()).is_err());assert!(Diagram::build(vec![Point::new(-1.,0.)],bounds()).is_err());}
use rusty_voronoi::{Diagram,geometry::{Point,Bounds}};
fn bounds()->Bounds {Bounds::new(0.,0.,10.,10.).unwrap()}
#[test] fn one_site_owns_the_entire_viewport() {let d=Diagram::build(vec![Point::new(3.,4.)],bounds()).unwrap();assert_eq!(d.cells().len(),1);assert_eq!(d.cells()[0].area(),100.);assert_eq!(d.cells()[0].site,0);assert_eq!(d.bounds(),bounds());}
#[test] fn empty_diagram_has_no_cells() {assert!(Diagram::build(vec![],bounds()).unwrap().cells().is_empty());}
