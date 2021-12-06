//! Fortune's descending sweep and checked bounded-cell construction.
pub fn parabola_y(focus:Point,directrix:f64,x:f64)->Option<f64> {
 let dy=focus.y-directrix;if dy.abs()<=f64::EPSILON {None}else{Some((focus.y+directrix)*0.5+(x-focus.x).powi(2)/(2.*dy))}
}
use crate::geometry::*;
pub fn site_order(sites:&[Point])->Result<Vec<usize>,&'static str> {
 if sites.iter().any(|p|!p.is_finite()){return Err("nonfinite site");}
 let mut order:Vec<_>=(0..sites.len()).collect();order.sort_by(|&i,&j|sites[j].y.total_cmp(&sites[i].y).then(sites[i].x.total_cmp(&sites[j].x)).then(i.cmp(&j)));
 order.dedup_by(|a,b|sites[*a]==sites[*b]);Ok(order)
}
