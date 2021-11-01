//! Bounded polygon cells and incremental edits.
impl Diagram {
 pub fn validate(&self,tolerance:f64)->Result<(),&'static str> {
  if !tolerance.is_finite()||tolerance<0. {return Err("invalid tolerance");}
  if self.sites.is_empty(){return Ok(());}
  let scale=self.bounds.area().max(1.);
  if (self.cells.iter().map(Cell::area).sum::<f64>()-self.bounds.area()).abs()>tolerance*scale {return Err("cell area does not cover bounds");}
  for cell in &self.cells {for &v in &cell.vertices {
   if !v.is_finite() {return Err("nonfinite cell vertex");}
   let margin=tolerance*self.bounds.width().max(self.bounds.height()).max(1.);
   if v.x<self.bounds.min().x-margin||v.y<self.bounds.min().y-margin||v.x>self.bounds.max().x+margin||v.y>self.bounds.max().y+margin {return Err("cell vertex outside bounds");}
   let owner=self.sites[cell.site];if self.sites.iter().any(|&p|v.distance2(owner)>v.distance2(p)+tolerance*scale){return Err("vertex violates nearest-site ownership");}
  }}Ok(())
 }
}
impl Diagram {
 pub fn neighbors(&self,index:usize)->Vec<usize> {let mut out=Vec::new();for e in self.edges(){if let Some(r)=e.right {if e.left==index {out.push(r);}else if r==index{out.push(e.left);}}}out.sort_unstable();out.dedup();out}
 /// Bounded dual adjacency, excluding zero-length cocircular contacts.
 pub fn delaunay_edges(&self)->Vec<(usize,usize)> {let mut pairs:Vec<_>=self.edges().into_iter().filter_map(|e|e.right.map(|r|(e.left,r))).collect();pairs.sort_unstable();pairs.dedup();pairs}
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Edge {pub start:Point,pub end:Point,pub left:usize,pub right:Option<usize>}
impl Edge {pub fn length(self)->f64 {self.start.distance2(self.end).sqrt()}}
impl Diagram {
 pub fn edges(&self)->Vec<Edge> {
  let tolerance=1e-8*self.bounds.width().hypot(self.bounds.height()).powi(2).max(1.);
  let mut edges=Vec::new();for cell in &self.cells {
   for (&start,&end) in cell.vertices.iter().zip(cell.vertices.iter().cycle().skip(1)).take(cell.vertices.len()) {
    if start.distance2(end)<=1e-24 {continue;}
    let mid=start.lerp(end,0.5);let owner=self.sites[cell.site];
    let right=self.sites.iter().enumerate().filter(|(j,_)|*j!=cell.site).find(|(_,p)|(mid.distance2(**p)-mid.distance2(owner)).abs()<=tolerance).map(|(j,_)|j);
    if right.is_none_or(|j|cell.site<j) {edges.push(Edge{start,end,left:cell.site,right});}
   }
  }edges
 }
}
impl Diagram {
 pub fn locate(&self,point:Point)->Option<usize> {
  if !point.is_finite()||!self.bounds.contains(point){return None;}
  self.sites.iter().enumerate().min_by(|(i,a),(j,b)|a.distance2(point).total_cmp(&b.distance2(point)).then(i.cmp(j))).map(|(i,_)|i)
 }
}
impl Diagram {
 /// Remove a canonical site; higher site indices are compacted.
 pub fn remove(&mut self,index:usize)->bool {
  if index>=self.sites.len(){return false;}
  let mut sites=self.sites.clone();sites.remove(index);*self=Self::build(sites,self.bounds).expect("retained sites were validated");true
 }
}
impl Diagram {
 pub fn insert(&mut self,point:Point)->Result<usize,&'static str> {
  if !point.is_finite()||!self.bounds.contains(point) {return Err("site must be finite and inside the bounds");}
  if let Some(i)=self.sites.iter().position(|p|*p==point){return Ok(i);}
  let id=self.sites.len();let mut vertices=self.bounds.corners();
  for cell in &mut self.cells {let other=self.sites[cell.site];cell.vertices=clip_bisector(&cell.vertices,other,point);vertices=clip_bisector(&vertices,point,other);}
  self.sites.push(point);self.cells.push(Cell{site:id,vertices});Ok(id)
 }
}
use crate::geometry::*;
#[derive(Debug,Clone,PartialEq)]
pub struct Cell {pub site:usize,pub vertices:Vec<Point>}
impl Cell {pub fn area(&self)->f64 {polygon_area(&self.vertices)}}
#[derive(Debug,Clone)]
pub struct Diagram {sites:Vec<Point>,cells:Vec<Cell>,bounds:Bounds}
impl Diagram {
 pub fn build(sites:Vec<Point>,bounds:Bounds)->Result<Self,&'static str> {
  if sites.iter().any(|p|!p.is_finite()||!bounds.contains(*p)) {return Err("sites must be finite and inside the bounds");}
  let mut unique=Vec::new(); for p in sites {if !unique.contains(&p){unique.push(p);}} let sites=unique;
  let cells=sites.iter().enumerate().map(|(i,&owner)|{let mut vertices=bounds.corners();for (j,&other) in sites.iter().enumerate(){if i!=j {vertices=clip_bisector(&vertices,owner,other);}}Cell{site:i,vertices}}).collect();
  Ok(Self{sites,cells,bounds})
 }
 pub fn cells(&self)->&[Cell] {&self.cells}
 pub fn sites(&self)->&[Point] {&self.sites}
 pub fn bounds(&self)->Bounds {self.bounds}
}
