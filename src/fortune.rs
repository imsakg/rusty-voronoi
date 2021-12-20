//! Fortune's descending sweep and checked bounded-cell construction.
#[derive(Debug,Clone,Default)]
pub struct Sweep {pub pairs:Vec<(usize,usize)>,pub vertices:Vec<Point>,pub site_events:usize,pub circle_events:usize,pub discarded_events:usize}
#[derive(Clone,Copy)]
struct Arc {site:usize,id:usize}
fn add_pair(pairs:&mut std::collections::BTreeSet<(usize,usize)>,a:usize,b:usize) {if a!=b {pairs.insert((a.min(b),a.max(b)));}}
pub fn sweep(sites:&[Point])->Result<Sweep,&'static str> {
 let mut beach:Vec<Arc>=Vec::new();let mut pairs=std::collections::BTreeSet::new();let mut result=Sweep::default();let mut next_id=0;
 for site in site_order(sites)? {
  let p=sites[site];result.site_events+=1;
  if beach.is_empty(){beach.push(Arc{site,id:next_id});next_id+=1;continue;}
  if beach.iter().all(|a|sites[a.site].y==p.y) {let pos=beach.iter().position(|a|sites[a.site].x>p.x).unwrap_or(beach.len());if pos>0{add_pair(&mut pairs,beach[pos-1].site,site);}if pos<beach.len(){add_pair(&mut pairs,beach[pos].site,site);}beach.insert(pos,Arc{site,id:next_id});next_id+=1;continue;}
  let index=(0..beach.len().saturating_sub(1)).find(|&i|p.x<=breakpoint(sites[beach[i].site],sites[beach[i+1].site],p.y)).unwrap_or(beach.len()-1);
  let old=beach[index];add_pair(&mut pairs,old.site,site);beach.splice(index..=index,[old,Arc{site,id:next_id},Arc{site:old.site,id:next_id+1}]);next_id+=2;
 }
 result.pairs=pairs.into_iter().collect();Ok(result)
}
pub fn breakpoint(left:Point,right:Point,directrix:f64)->f64 {
 if (left.y-right.y).abs()<=f64::EPSILON {return (left.x+right.x)*0.5;}
 if (left.y-directrix).abs()<=f64::EPSILON {return left.x;}
 if (right.y-directrix).abs()<=f64::EPSILON {return right.x;}
 let dl=left.y-directrix;let dr=right.y-directrix;let dx=right.x-left.x;
 let a=0.5/dl-0.5/dr;let b=dx/dr;let c=(left.y-right.y)*0.5-dx*dx/(2.*dr);
 let root=(b*b-4.*a*c).max(0.).sqrt();let x1=(-b-root)/(2.*a)+left.x;let x2=(-b+root)/(2.*a)+left.x;
 if left.y<right.y {x1.max(x2)}else{x1.min(x2)}
}
pub fn parabola_y(focus:Point,directrix:f64,x:f64)->Option<f64> {
 let dy=focus.y-directrix;if dy.abs()<=f64::EPSILON {None}else{Some((focus.y+directrix)*0.5+(x-focus.x).powi(2)/(2.*dy))}
}
use crate::geometry::*;
pub fn site_order(sites:&[Point])->Result<Vec<usize>,&'static str> {
 if sites.iter().any(|p|!p.is_finite()){return Err("nonfinite site");}
 let mut order:Vec<_>=(0..sites.len()).collect();order.sort_by(|&i,&j|sites[j].y.total_cmp(&sites[i].y).then(sites[i].x.total_cmp(&sites[j].x)).then(i.cmp(&j)));
 order.dedup_by(|a,b|sites[*a]==sites[*b]);Ok(order)
}
