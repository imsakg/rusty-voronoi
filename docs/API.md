# Library guide

- `geometry::Bounds::new(x0, y0, x1, y1)` validates a rectangular viewport.
- `Diagram::build(sites, bounds)` constructs reference clipped cells.
- `fortune::build(sites, bounds)` returns `(Diagram, BuildStats)` using a real Fortune sweep followed by conservative certification.
- `diagram.cells()` and `diagram.sites()` expose read-only slices in canonical insertion order. Cells carry their canonical site ID and counterclockwise vertices.
- `diagram.insert(point)` returns an existing ID for an exact duplicate; errors do not alter the diagram. `diagram::Builder` exposes every incremental state.
- `diagram.remove(index)` rebuilds and compacts IDs. `diagram.relax(count)` returns a new centroidal diagram.
- `diagram.locate(point)` returns the nearest site ID, with stable tie-breaking, or `None` outside the bounds or for an empty diagram.
- `Cell::{area,centroid,perimeter,contains}` measure and query a cell. Degenerate polygon centroids are `None`.
- `diagram.edges()` returns each positive-length shared edge once, plus boundary segments. `Edge::right == None` denotes a boundary edge. `left`/`right` name canonical incident sites, not a directional half-edge/DCEL orientation contract.
- `diagram.neighbors(id)` and `diagram.delaunay_edges()` exclude corner-only contacts. These describe bounded cells, not every edge of an unbounded triangulation.
- `diagram.validate(relative_tolerance)` checks area coverage, finite vertices, bounds and nearest-site inequalities. Empty diagrams have no coverage obligation.
- `render::{svg,svg_with_options,html,html_frames,ppm}` provide dependency-free export. HTML is an offline view: controls toggle layers, selecting a polygon shows its ID, and frame playback shows precomputed insertion states. It does not edit the Rust diagram in-browser.

## Numerical contract

Bounds reject nonfinite coordinates, coordinate magnitudes above `1e100`, and extents below `1e-100` before construction. Exact duplicate coordinates collapse; arbitrarily close sites remain distinct, subject to normal `f64` rounding. Shared edges are classified using both endpoint bisector residuals relative to the site separation and viewport size. Small polygons are not deduplicated using an absolute world-space threshold.

These are floating-point geometric predicates, not a formally exact kernel. Extremely skinny cells, unresolved coordinate separations, and adversarial near-degeneracies can still lose precision. Call `validate` when numerical quality matters, and rescale pathological data toward a unit-sized viewport. Raw geometry helper functions expect finite, numerically supported inputs; they are not independent sanitizing constructors.

## Complexity and repeatability

Reference clipping and incremental updates are intentionally direct. Fortune uses a vector beach line and globally checks clipping constraints, so optimal asymptotic performance is not claimed. Repair statistics count conservative extra clipping passes, including residual roundoff, not just unique missing Delaunay neighbors.

`cli::random_sites` is repeatable for the same seed and locked dependency versions. It is not cryptographic randomness and the sequence is not promised stable across dependency upgrades. `cargo run --release --example timing` measures local construction time; it is not a portable benchmark claim.
