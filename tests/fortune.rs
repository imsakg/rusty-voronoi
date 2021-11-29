// Behavioral fortune tests.
use rusty_voronoi::{fortune::*,geometry::Point};
#[test] fn sites_are_processed_top_down_then_left_to_right() {let p=[Point::new(3.,2.),Point::new(2.,4.),Point::new(1.,4.),Point::new(3.,2.)];assert_eq!(site_order(&p).unwrap(),vec![2,1,0]);assert!(site_order(&[Point::new(f64::NAN,0.)]).is_err());}
