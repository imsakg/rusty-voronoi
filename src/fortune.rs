//! Fortune's descending sweep and checked bounded-cell construction.
use crate::geometry::*;
pub fn site_order(sites:&[Point])->Result<Vec<usize>,&'static str> {
 if sites.iter().any(|p|!p.is_finite()){return Err("nonfinite site");}
 let mut order:Vec<_>=(0..sites.len()).collect();order.sort_by(|&i,&j|sites[j].y.total_cmp(&sites[i].y).then(sites[i].x.total_cmp(&sites[j].x)).then(i.cmp(&j)));
 order.dedup_by(|a,b|sites[*a]==sites[*b]);Ok(order)
}
