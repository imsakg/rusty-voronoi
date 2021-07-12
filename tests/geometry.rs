// Behavioral geometry tests.
#[test] fn shoelace_area_handles_winding_and_empty_input() { let p=Bounds::new(0.,0.,4.,3.).unwrap().corners(); assert_eq!(signed_area(&p),12.); let r:Vec<_>=p.iter().rev().copied().collect(); assert_eq!(signed_area(&r),-12.); assert_eq!(polygon_area(&r),12.); assert_eq!(polygon_area(&[]),0.); }
#[test] fn rectangle_geometry_is_counterclockwise() { let b=Bounds::new(-1.,-2.,3.,4.).unwrap(); assert_eq!(b.width(),4.); assert_eq!(b.height(),6.); assert_eq!(b.area(),24.); assert!(b.contains(Point::new(3.,4.))); assert!(!b.contains(Point::new(4.,4.))); assert_eq!(b.corners()[0],Point::new(-1.,-2.)); }
use rusty_voronoi::geometry::*;
#[test] fn bounds_validate_dimensions() { assert!(Bounds::new(0.,0.,10.,20.).is_ok()); assert!(Bounds::new(0.,0.,0.,1.).is_err()); assert!(Bounds::new(0.,0.,f64::NAN,1.).is_err()); }
#[test] fn point_distance_is_symmetric() { let a=Point::new(1.,2.); let b=Point::new(-2.,6.); assert_eq!(a.distance2(b),25.); assert_eq!(b.distance2(a),25.); }
