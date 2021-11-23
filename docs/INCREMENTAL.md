# Incremental diagrams

`Diagram::insert(point)` clips each existing cell against the new site's bisector and constructs the new cell against all existing sites. Existing site IDs stay stable; exact duplicate insertion returns its original ID. Rejected points leave the diagram unchanged.

`Builder` starts empty and exposes `diagram()` after each `push()`. This makes incremental construction observable without retaining a second competing state. Insertions are not a Fortune sweep and do not claim logarithmic complexity.

Removal must restore portions previously clipped away, so `remove(index)` rebuilds from the remaining sites. IDs above the removed index shift down by one. `relax(iterations)` returns a new diagram whose sites move to area-weighted cell centroids; it never mutates the original.

Point location breaks exact distance ties by canonical site ID. Dual edges are bounded-cell adjacencies: a corner-only contact contributes no positive-length edge and no arbitrary triangulation diagonal.
