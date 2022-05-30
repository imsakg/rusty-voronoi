# Validation and review

## Reproducible gates

```sh
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo test --locked --doc
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps
```

The local verification run completed all five gates successfully on macOS using Rust 1.99.0. There are 48 integration tests and one executable documentation test. The seeded differential test covers 256 datasets across four coordinate scales (`1e-13`, `1e-6`, `1`, `1e6`) and compares batch clipping, incremental construction and checked Fortune cells. Other tests cover equal-height and vertical rows, grids, cocircular points, duplicate and signed-zero sites, insertion/removal, close distinct sites, tiny polygons, raster limits and invalid CLI inputs.

## Real artifact checks

The CLI and examples were actually run to write SVG, offline HTML, insertion-playback HTML and P6 PPM files outside the repository. The exported SVG parsed successfully as XML. The 64-by-48 raster contained its P6 header and exactly three bytes per pixel. The real executable tests exercise all export formats, point-file input, playback templates and error paths that must not create output files.

The insertion visualizer loaded successfully in a local browser, showing the three layer controls and the insertion slider; browser console collection reported zero JavaScript errors. Automated interactive manipulation was blocked by the browser action-approval layer, so a full click/slider UI acceptance pass is **not** claimed. The frame/layer generation and executable exports are covered by integration tests.

## Independent diagnostic review

An isolated source-snapshot reviewer reproduced arithmetic overflow on huge bounds, absolute-epsilon collapse of tiny cells, scale-dependent containment, false midpoint-based adjacency and signed-zero event duplication. Each finding received a regression test and a targeted fix. Randomized checks and the complete suite passed after these fixes. This was a diagnostic review of the earlier snapshot, not an independent certification of the final HEAD.

## Not claimed

- Hosted CI execution: the three-platform workflow is configured but has not been pushed or run remotely.
- Rust 1.82 execution: this is the declared minimum from the APIs in use and dependency metadata; the local gates ran on the installed Rust 1.99.0 toolchain.
- Exact arithmetic or optimal asymptotic Fortune performance.
- Benchmarks portable across machines: the timing example measures only its current host.
