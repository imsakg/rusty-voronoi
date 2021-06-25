# Voronoi library implementation plan

Goal: evolve the original raster sketch into a tested, bounded Voronoi library and a portable visualizer.

Architecture: double-precision geometry; convex half-plane clipping as the reference; incremental insertion; a Fortune sweep with site and circle events for candidate neighbors; explicit polygon edges; SVG and self-contained HTML output. The vector beach line is intentionally educational, not an O(n log n) claim.

Implementation slices (test first, then implementation):
1. Preserve and fix raster distance/marker regressions.
2. Add Point, Bounds, polygon predicates, clipping and cell measurements in `src/geometry.rs`.
3. Build bounded cells, insertion, topology and queries in `src/diagram.rs`.
4. Add Fortune queue, breakpoints, circle events and checked candidate clipping in `src/fortune.rs`.
5. Add portable SVG/HTML rendering and a deterministic CLI.
6. Exercise degeneracies, differential tests, examples and documentation.

For each behavior, run the focused failing test, implement it, then run the complete library/integration suite. Final gates: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets --all-features`, doctests and real SVG/HTML CLI smoke runs.

History note: the development history in this local reconstruction is deliberately assigned to June 2021–May 2022 at the owner's request. These timestamps are reconstructed metadata, not a claim that these changes were authored during that period. The original source history is preserved in an external Git bundle before rewriting. No push is part of this work.
