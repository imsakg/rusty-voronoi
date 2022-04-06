# rusty-voronoi

A pure Rust, bounded 2D Voronoi library and portable visualizer. The original pixel-based sketch has grown into explicit polygon cells, incremental insertion, shared edges, point queries, Lloyd relaxation, and an educational Fortune sweep.

## Try it

```sh
cargo run -- --count 40 --seed 7 --output diagram.svg
cargo run -- --algorithm fortune --count 40 --seed 7 --dual --output diagram.html
cargo run -- --count 20 --seed 7 --steps --output insertion.html
cargo run -- --sites examples/sites.csv --width 10 --height 10 --labels --output sites.svg
cargo run -- --count 24 --relax 3 --output relaxed.svg
cargo run -- --count 10 --width 256 --height 256 --output image.ppm
```

Open SVG/HTML locally in a browser; no web server, JavaScript package manager, or remote service is needed. `--output -` writes SVG to stdout. Errors exit with status 2 and diagnostics go to stderr.

## Use the library

```rust
use rusty_voronoi::{Diagram, geometry::{Bounds, Point}};
let bounds = Bounds::new(0.0, 0.0, 10.0, 10.0)?;
let mut diagram = Diagram::build(vec![Point::new(2.0, 5.0)], bounds)?;
diagram.insert(Point::new(8.0, 5.0))?;
assert_eq!(diagram.cells()[0].area(), 50.0);
assert_eq!(diagram.neighbors(0), vec![1]);
# Ok::<(), &'static str>(())
```

## Algorithms and limits

- Reference construction clips a rectangle against every site's perpendicular bisector.
- Insertion clips existing polygons and adds the new cell without rebuilding all old cells.
- Fortune processes site/circle events using a priority queue and vector beach line. Bounded cells are certified against all sites, repairing missing constraints. This implementation does **not** claim O(n log n) performance or exact predicates.
- Sites must be finite and inside the bounds. Exact duplicates collapse in input order. Removal compacts IDs. Clipped dual adjacency is not a complete unbounded Delaunay triangulation.
- CLI limits: 1000 sites, 200 playback sites, 100 Lloyd iterations, and 16 million PPM pixels.

## Development

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo test --doc
```

See [geometry](docs/GEOMETRY.md), [incremental construction](docs/INCREMENTAL.md), [Fortune](docs/FORTUNE.md), and the [implementation/history note](docs/IMPLEMENTATION.md).

## References

- [Voronoi diagrams](https://en.wikipedia.org/wiki/Voronoi_diagram)
- [Fortune's algorithm](https://en.wikipedia.org/wiki/Fortune%27s_algorithm)
- [Sutherland–Hodgman clipping](https://en.wikipedia.org/wiki/Sutherland%E2%80%93Hodgman_algorithm)
- [Netpbm PPM](https://netpbm.sourceforge.net/doc/ppm.html)

MIT licensed; see [LICENSE](LICENSE).
