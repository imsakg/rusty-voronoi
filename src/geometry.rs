//! Basic Euclidean geometry used by clipping and the sweep.
pub fn signed_area(points:&[Point])->f64 {
 if points.len()<3 {return 0.;}
 let o=points[0]; points.iter().zip(points.iter().cycle().skip(1)).take(points.len()).map(|(a,b)|(a.x-o.x)*(b.y-o.y)-(b.x-o.x)*(a.y-o.y)).sum::<f64>()*0.5
}
pub fn polygon_area(points:&[Point])->f64 {signed_area(points).abs()}
impl Bounds {
 pub fn width(self)->f64 {self.max.x-self.min.x}
 pub fn height(self)->f64 {self.max.y-self.min.y}
 pub fn area(self)->f64 {self.width()*self.height()}
 pub fn contains(self,p:Point)->bool { p.x>=self.min.x && p.x<=self.max.x && p.y>=self.min.y && p.y<=self.max.y }
 pub fn corners(self)->Vec<Point> {vec![self.min,Point::new(self.max.x,self.min.y),self.max,Point::new(self.min.x,self.max.y)]}
}
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
