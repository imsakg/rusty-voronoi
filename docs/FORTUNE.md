# Fortune sweep

`fortune::sweep` processes site events from high y to low y. A binary heap interleaves circle events. The beach line is an ordered vector of arcs; site events split an arc into old/new/old, while valid circle events remove the disappearing middle arc and record its circumcenter. Events carry the identities of all three arcs, so outdated triples are discarded lazily. Equal-height startup rows are handled without singular parabolas.

`fortune::build` normalizes the sweep coordinates, extracts candidate site pairs, and clips their bounded cells. It then checks every remaining site inequality at the polygon vertices and clips missing constraints. This is a real Fortune event implementation coupled to a conservative reference-style certificate, not an unqualified optimal-complexity implementation. Cocircular or nearly collinear data may require repairs; `BuildStats::repaired_constraints` reports them.

The beach line uses vector searches, and scheduling rescans neighboring triples. Certification checks all sites. **No O(n log n) performance claim is made.** The bounded output is the supported API; raw sweep pairs are candidates and can contain zero-length dual contacts. Use `Diagram::delaunay_edges()` for actual positive-length bounded adjacency.
