use rusty_voronoi::geometry::{Point, Bounds};
#[test] fn bounds_validate_dimensions() { assert!(Bounds::new(0.,0.,10.,20.).is_ok()); assert!(Bounds::new(0.,0.,0.,1.).is_err()); assert!(Bounds::new(0.,0.,f64::NAN,1.).is_err()); }
#[test] fn point_distance_is_symmetric() { let a=Point::new(1.,2.); let b=Point::new(-2.,6.); assert_eq!(a.distance2(b),25.); assert_eq!(b.distance2(a),25.); }
