# Geometry conventions

Coordinates are finite `f64` values. Bounds have strictly positive width and height. Rectangles and freshly clipped cells are counterclockwise; area and containment also accept reversed winding. The library uses mathematical y-up coordinates; SVG applies a vertical reflection.

Bisectors use `(other - owner) · (vertex - midpoint) <= 0`, avoiding the subtraction of two large squared site norms. Sutherland–Hodgman clipping preserves convexity. Polygon area and centroid calculations translate to the first vertex before summation to reduce cancellation.

Distances are squared until a Euclidean radius or length is explicitly needed. Duplicate sites are identified by exact coordinate equality; near-duplicates remain distinct. Floating-point predicates are not exact arithmetic: nearly collinear and cocircular configurations need conservative fallback checks.
