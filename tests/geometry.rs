use rusty_voronoi::geometry::Point;
#[test] fn point_distance_is_symmetric() { let a=Point::new(1.,2.); let b=Point::new(-2.,6.); assert_eq!(a.distance2(b),25.); assert_eq!(b.distance2(a),25.); }
