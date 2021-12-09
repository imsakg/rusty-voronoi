// Behavioral fortune tests.
#[test] fn breakpoint_handles_equal_heights_and_new_arcs() {let a=Point::new(0.,4.);let b=Point::new(4.,4.);assert_eq!(breakpoint(a,b,0.),2.);let c=Point::new(2.,2.);assert_eq!(breakpoint(a,c,2.),2.);let x=breakpoint(a,c,0.);assert!((parabola_y(a,0.,x).unwrap()-parabola_y(c,0.,x).unwrap()).abs()<1e-9);assert!(x<2.);}
#[test] fn parabola_satisfies_focus_directrix_distance() {let f=Point::new(2.,4.);let y=parabola_y(f,0.,5.).unwrap();let p=Point::new(5.,y);assert!((p.distance2(f)-y*y).abs()<1e-12);assert!(parabola_y(f,4.,2.).is_none());}
use rusty_voronoi::{fortune::*,geometry::Point};
#[test] fn sites_are_processed_top_down_then_left_to_right() {let p=[Point::new(3.,2.),Point::new(2.,4.),Point::new(1.,4.),Point::new(3.,2.)];assert_eq!(site_order(&p).unwrap(),vec![2,1,0]);assert!(site_order(&[Point::new(f64::NAN,0.)]).is_err());}
