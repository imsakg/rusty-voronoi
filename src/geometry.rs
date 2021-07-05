//! Basic Euclidean geometry used by clipping and the sweep.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds { min: Point, max: Point }
impl Bounds {
    pub fn new(x0:f64,y0:f64,x1:f64,y1:f64)->Result<Self,&'static str> {
        if ![x0,y0,x1,y1].iter().all(|v|v.is_finite()) || x1<=x0 || y1<=y0 { return Err("bounds must be finite with positive dimensions"); }
        Ok(Self { min:Point::new(x0,y0),max:Point::new(x1,y1) })
    }
    pub fn min(self)->Point { self.min }
    pub fn max(self)->Point { self.max }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point { pub x: f64, pub y: f64 }
impl Point {
    pub const fn new(x: f64, y: f64) -> Self { Self { x, y } }
    pub fn distance2(self, other: Self) -> f64 { (self.x-other.x).powi(2)+(self.y-other.y).powi(2) }
}
