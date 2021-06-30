//! Basic Euclidean geometry used by clipping and the sweep.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point { pub x: f64, pub y: f64 }
impl Point {
    pub const fn new(x: f64, y: f64) -> Self { Self { x, y } }
    pub fn distance2(self, other: Self) -> f64 { (self.x-other.x).powi(2)+(self.y-other.y).powi(2) }
}
