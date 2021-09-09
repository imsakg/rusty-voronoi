//! Bounded polygon cells and incremental edits.
use crate::geometry::*;
#[derive(Debug,Clone,PartialEq)]
pub struct Cell {pub site:usize,pub vertices:Vec<Point>}
impl Cell {pub fn area(&self)->f64 {polygon_area(&self.vertices)}}
#[derive(Debug,Clone)]
pub struct Diagram {sites:Vec<Point>,cells:Vec<Cell>,bounds:Bounds}
impl Diagram {
 pub fn build(sites:Vec<Point>,bounds:Bounds)->Result<Self,&'static str> {
  if sites.iter().any(|p|!p.is_finite()||!bounds.contains(*p)) {return Err("sites must be finite and inside the bounds");}
  let cells=sites.iter().enumerate().map(|(i,&owner)|{let mut vertices=bounds.corners();for (j,&other) in sites.iter().enumerate(){if i!=j {vertices=clip_bisector(&vertices,owner,other);}}Cell{site:i,vertices}}).collect();
  Ok(Self{sites,cells,bounds})
 }
 pub fn cells(&self)->&[Cell] {&self.cells}
 pub fn sites(&self)->&[Point] {&self.sites}
 pub fn bounds(&self)->Bounds {self.bounds}
}
