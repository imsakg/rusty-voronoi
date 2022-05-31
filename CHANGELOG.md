# Changelog

## 0.1.0 — bounded Voronoi library

### Geometry and construction

- Floating-point point/bounds primitives, polygon clipping, orientation, circumcenters, area and centroids.
- Explicit bounded cells, point-location queries, shared edges, neighbor lists and bounded Delaunay adjacency.
- Incremental point insertion and an observable builder; canonical duplicate handling, removal and Lloyd relaxation.
- Educational Fortune site/circle sweep, stale-event rejection, equal-level startup handling and certified bounded-cell clipping.
- Relative numerical predicates, tiny-cell preservation, close-site topology fixes and early rejection of unsupported bounds.

### Visualization and tooling

- SVG export with accessible escaped titles, site IDs and dual overlays.
- Self-contained HTML layer controls and precomputed incremental playback.
- P6 PPM export with explicit allocation limits.
- Reproducible seeded CLI generation, point-file input, error reporting and selectable construction algorithms.
- Runnable basic/incremental/Fortune/timing examples, integration tests, executable API documentation and a three-platform CI definition.

### History provenance

This is a local reconstruction requested by the repository owner. One hundred substantive development commits were added to the original five-commit source history. Author and committer dates are deliberately reassigned to randomized weekday evening times between June 2021 and May 2022; they are reconstructed metadata rather than evidence of contemporaneous authorship. Some test-first commits intentionally precede their implementations and therefore represent a failing test checkpoint.

The original source commit graph was preserved in an external Git bundle before rewriting. The reconstruction uses explicit `git commit --date` values and a root rebase for chronological dating. Original source commit subjects and trees remain in the ancestry, with rewritten identities due to the date change. No push is performed.

See [validation](docs/VALIDATION.md) for what was actually exercised and what remains unverified.
