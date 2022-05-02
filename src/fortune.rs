//! Fortune's descending sweep and checked bounded-cell construction.
#[derive(Debug,Clone,Default)]
pub struct BuildStats {pub site_events:usize,pub circle_events:usize,pub discarded_events:usize,pub candidate_edges:usize,pub repaired_constraints:usize}
/// Build bounded cells using sweep candidates plus certified missing constraints.
pub fn build(sites:Vec<Point>,bounds:Bounds)->Result<(crate::Diagram,BuildStats),&'static str> {
 if sites.iter().any(|p|!p.is_finite()||!bounds.contains(*p)){return Err("sites must be finite and inside bounds");}
 let mut unique=Vec::new();for p in sites{if !unique.contains(&p){unique.push(p);}}
 let scale=bounds.width().max(bounds.height());let origin=bounds.min();
 let normalized:Vec<_>=unique.iter().map(|p|Point::new((p.x-origin.x)/scale,(p.y-origin.y)/scale)).collect();
 let sweep=sweep(&normalized)?;let (diagram,repairs)=crate::Diagram::checked_candidates(unique,bounds,&sweep.pairs)?;
 let stats=BuildStats{site_events:sweep.site_events,circle_events:sweep.circle_events,discarded_events:sweep.discarded_events,candidate_edges:sweep.pairs.len(),repaired_constraints:repairs};Ok((diagram,stats))
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
 let canonical=|x:f64|if x==0.{0.}else{x};
 let mut order:Vec<_>=(0..sites.len()).collect();order.sort_by(|&i,&j|canonical(sites[j].y).total_cmp(&canonical(sites[i].y)).then(canonical(sites[i].x).total_cmp(&canonical(sites[j].x))).then(i.cmp(&j)));
 order.dedup_by(|a,b|sites[*a]==sites[*b]);Ok(order)
}

#[derive(Debug,Clone,Default)]
pub struct Sweep {pub pairs:Vec<(usize,usize)>,pub vertices:Vec<Point>,pub site_events:usize,pub circle_events:usize,pub discarded_events:usize}
#[derive(Clone,Copy)]
struct Arc {site:usize,id:usize}
fn add_pair(pairs:&mut std::collections::BTreeSet<(usize,usize)>,a:usize,b:usize) {if a!=b {pairs.insert((a.min(b),a.max(b)));}}
#[derive(Clone,Copy)]
enum Kind {Site(usize),Circle{triple:[usize;3],center:Point}}
#[derive(Clone,Copy)]
struct Event {y:f64,x:f64,serial:usize,kind:Kind}
impl PartialEq for Event {fn eq(&self,other:&Self)->bool {self.serial==other.serial}}
impl Eq for Event {}
impl PartialOrd for Event {fn partial_cmp(&self,other:&Self)->Option<std::cmp::Ordering>{Some(self.cmp(other))}}
impl Ord for Event {fn cmp(&self,other:&Self)->std::cmp::Ordering {
 self.y.total_cmp(&other.y).then_with(||matches!(self.kind,Kind::Site(_)).cmp(&matches!(other.kind,Kind::Site(_)))).then_with(||other.x.total_cmp(&self.x)).then_with(||other.serial.cmp(&self.serial))
}}
fn schedule(beach:&[Arc],sites:&[Point],level:f64,queue:&mut std::collections::BinaryHeap<Event>,scheduled:&mut std::collections::HashSet<[usize;3]>,serial:&mut usize) {
 for arcs in beach.windows(3) {
  let a=sites[arcs[0].site];let b=sites[arcs[1].site];let c=sites[arcs[2].site];
  if orientation(a,b,c)>=0. {continue;}
  if let Some((center,radius))=circumcircle(a,b,c) {
   let y=center.y-radius;let triple=[arcs[0].id,arcs[1].id,arcs[2].id];
   if y<=level+1e-12 && y.is_finite() && scheduled.insert(triple) {queue.push(Event{y,x:center.x,serial:*serial,kind:Kind::Circle{triple,center}});*serial+=1;}
  }
 }
}
/// Educational Fortune sweep: vector beach line, lazy stale-event rejection.
pub fn sweep(sites:&[Point])->Result<Sweep,&'static str> {
 let mut queue=std::collections::BinaryHeap::new();let mut serial=0;
 for site in site_order(sites)? {queue.push(Event{y:sites[site].y,x:sites[site].x,serial,kind:Kind::Site(site)});serial+=1;}
 let mut beach:Vec<Arc>=Vec::new();let mut pairs=std::collections::BTreeSet::new();let mut result=Sweep::default();let mut next_id=0;let mut scheduled=std::collections::HashSet::new();
 while let Some(event)=queue.pop() {
  match event.kind {
   Kind::Site(site)=>{
    let p=sites[site];result.site_events+=1;
    if beach.is_empty(){beach.push(Arc{site,id:next_id});next_id+=1;}
    else if beach.iter().all(|a|sites[a.site].y==p.y) {
     let pos=beach.iter().position(|a|sites[a.site].x>p.x).unwrap_or(beach.len());
     if pos>0{add_pair(&mut pairs,beach[pos-1].site,site);}if pos<beach.len(){add_pair(&mut pairs,beach[pos].site,site);}
     beach.insert(pos,Arc{site,id:next_id});next_id+=1;
    }else{
     let index=(0..beach.len().saturating_sub(1)).find(|&i|p.x<=breakpoint(sites[beach[i].site],sites[beach[i+1].site],p.y)).unwrap_or(beach.len()-1);
     let old=beach[index];add_pair(&mut pairs,old.site,site);beach.splice(index..=index,[old,Arc{site,id:next_id},Arc{site:old.site,id:next_id+1}]);next_id+=2;
    }
   }
   Kind::Circle{triple,center}=>{
    if let Some(index)=beach.windows(3).position(|a|[a[0].id,a[1].id,a[2].id]==triple) {
     add_pair(&mut pairs,beach[index].site,beach[index+2].site);beach.remove(index+1);result.vertices.push(center);result.circle_events+=1;
    }else{result.discarded_events+=1;continue;}
   }
  }
  schedule(&beach,sites,event.y,&mut queue,&mut scheduled,&mut serial);
 }
 result.pairs=pairs.into_iter().collect();Ok(result)
}
