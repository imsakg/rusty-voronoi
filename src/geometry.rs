//! Basic Euclidean geometry used by clipping and the sweep.
pub fn circumcircle(a:Point,b:Point,c:Point)->Option<(Point,f64)> {
 let bx=b.x-a.x;let by=b.y-a.y;let cx=c.x-a.x;let cy=c.y-a.y;
 let d=2.*(bx*cy-by*cx); if d.abs()<=f64::EPSILON*(bx.abs()+by.abs())*(cx.abs()+cy.abs()) {return None;}
 let bb=bx*bx+by*by;let cc=cx*cx+cy*cy;
 let center=Point::new(a.x+(cy*bb-by*cc)/d,a.y+(bx*cc-cx*bb)/d);
 let r=center.distance2(a).sqrt(); if center.is_finite()&&r.is_finite() {Some((center,r))}else{None}
}
pub fn clip_bisector(polygon:&[Point],owner:Point,other:Point)->Vec<Point> {
 let n=Point::new(other.x-owner.x,other.y-owner.y);let mid=owner.lerp(other,0.5);
 clip_signed(polygon,|p|n.x*(p.x-mid.x)+n.y*(p.y-mid.y))
}
pub fn clip_halfplane(polygon:&[Point],normal:Point,offset:f64)->Vec<Point> {
 clip_signed(polygon,|p|normal.x*p.x+normal.y*p.y-offset)
}
fn clip_signed(polygon:&[Point],signed:impl Fn(Point)->f64)->Vec<Point> {
 let mut out=Vec::new(); if polygon.is_empty() {return out;}
 let mut previous=*polygon.last().unwrap(); let mut dp=signed(previous);
 for &current in polygon {let dc=signed(current);
  if (dp<=0.)!=(dc<=0.) {out.push(previous.lerp(current,dp/(dp-dc)));}
  if dc<=0. {out.push(current);}
  previous=current;dp=dc;
 }
 out.dedup();
 if out.len()>1 && out[0]==*out.last().unwrap() {out.pop();}
 out
}
pub fn convex_contains(points:&[Point],p:Point,epsilon:f64)->bool {
 if points.len()<3 || !p.is_finite() {return false;}
 let mut positive=false; let mut negative=false;
 for (a,b) in points.iter().zip(points.iter().cycle().skip(1)).take(points.len()) {let c=orientation(*a,*b,p); positive|=c>epsilon; negative|=c< -epsilon;}
 !(positive&&negative)
}
pub fn centroid(points:&[Point])->Option<Point> {
 let area=signed_area(points); if area==0. {return None;}
 let o=points[0]; let mut x=0.; let mut y=0.;
 for (a,b) in points.iter().zip(points.iter().cycle().skip(1)).take(points.len()) {
  let cross=(a.x-o.x)*(b.y-o.y)-(b.x-o.x)*(a.y-o.y);
  x+=((a.x-o.x)+(b.x-o.x))*cross; y+=((a.y-o.y)+(b.y-o.y))*cross;
 }
 Some(Point::new(o.x+x/(6.*area),o.y+y/(6.*area)))
}
pub fn orientation(a:Point,b:Point,c:Point)->f64 {(b.x-a.x)*(c.y-a.y)-(b.y-a.y)*(c.x-a.x)}
impl Point {
 pub fn is_finite(self)->bool {self.x.is_finite()&&self.y.is_finite()}
 pub fn lerp(self,other:Self,t:f64)->Self {Self::new(self.x+(other.x-self.x)*t,self.y+(other.y-self.y)*t)}
}
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
        if ![x0,y0,x1,y1].iter().all(|v|v.is_finite()&&v.abs()<=1e100) || x1-x0<1e-100 || y1-y0<1e-100 { return Err("bounds require coordinates <=1e100 and extents >=1e-100"); }
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
