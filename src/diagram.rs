//! Bounded polygon cells and incremental edits.
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
