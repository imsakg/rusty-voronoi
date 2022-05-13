//! Bounded two-dimensional Voronoi diagrams.
//!
//! ```
//! use rusty_voronoi::{Diagram, geometry::{Bounds, Point}};
//! let bounds = Bounds::new(0.0, 0.0, 10.0, 10.0)?;
//! let mut diagram = Diagram::build(vec![Point::new(2.0, 5.0)], bounds)?;
//! diagram.insert(Point::new(8.0, 5.0))?;
//! assert_eq!(diagram.cells()[0].area(), 50.0);
//! assert_eq!(diagram.neighbors(0), vec![1]);
//! diagram.validate(1e-8)?;
//! # Ok::<(), &'static str>(())
//! ```
pub mod geometry;
pub mod cli;
pub mod render;
pub mod fortune;
pub mod diagram;
pub use diagram::{Diagram,Cell};
