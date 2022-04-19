use rusty_voronoi::{cli,diagram::Builder,geometry::Bounds,render};
fn main()->Result<(),Box<dyn std::error::Error>> {
 let bounds=Bounds::new(0.,0.,800.,600.)?;let mut builder=Builder::new(bounds);let mut frames=vec![builder.diagram().clone()];
 for site in cli::random_sites(20,7,bounds){builder.push(site)?;builder.diagram().validate(1e-8)?;frames.push(builder.diagram().clone());}
 let path=std::env::args().nth(1).unwrap_or_else(||"insertion.html".into());
 std::fs::write(&path,render::html_frames(&frames,&Default::default())?)?;println!("wrote {} frames to {}",frames.len(),path);Ok(())
}
