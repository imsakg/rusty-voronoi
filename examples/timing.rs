//! A local timing probe, not a benchmark claim.
use rusty_voronoi::{cli,fortune,geometry::Bounds,Diagram};
use std::time::Instant;
fn main()->Result<(),Box<dyn std::error::Error>> {
 let bounds=Bounds::new(0.,0.,1000.,1000.)?;
 for count in [10,50,100,250]{let sites=cli::random_sites(count,42,bounds);let start=Instant::now();let reference=Diagram::build(sites.clone(),bounds)?;let clipping=start.elapsed();let start=Instant::now();let(d,stats)=fortune::build(sites,bounds)?;let sweep=start.elapsed();d.validate(1e-7)?;reference.validate(1e-7)?;println!("n={count} clipping={clipping:?} checked_fortune={sweep:?} events={} repairs={}",stats.site_events+stats.circle_events,stats.repaired_constraints);}
 Ok(())
}
