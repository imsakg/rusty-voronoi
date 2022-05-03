use rusty_voronoi::{cli,diagram::Builder,fortune,geometry::{Bounds,Point},Diagram};
fn compare(sites:Vec<Point>,bounds:Bounds){
 let reference=Diagram::build(sites.clone(),bounds).unwrap();let(fortune,_)=fortune::build(sites.clone(),bounds).unwrap();let mut builder=Builder::new(bounds);for p in sites{builder.push(p).unwrap();}let incremental=builder.finish();
 for d in [&reference,&fortune,&incremental]{d.validate(1e-7).unwrap();for (a,b) in reference.cells().iter().zip(d.cells()){assert!((a.area()-b.area()).abs()/bounds.area()<1e-7);assert!(b.vertices.iter().all(|v|v.is_finite()));}assert_eq!(reference.delaunay_edges(),d.delaunay_edges());}
 for x in 0..7{for y in 0..7{let p=Point::new(bounds.min().x+(f64::from(x)+0.5)*bounds.width()/7.,bounds.min().y+(f64::from(y)+0.5)*bounds.height()/7.);assert_eq!(reference.locate(p),fortune.locate(p));}}
}
#[test] fn seeded_differential_builds_across_coordinate_scales(){
 for scale in [1e-13,1e-6,1.,1e6]{let b=Bounds::new(-scale,-scale,2.*scale,scale).unwrap();for seed in 0..64{compare(cli::random_sites((seed%29+1) as usize,seed,b),b);}}
}
#[test] fn degenerate_rows_grids_and_cocircular_sites_remain_bounded(){
 let b=Bounds::new(0.,0.,10.,10.).unwrap();
 compare(vec![],b);compare(vec![Point::new(5.,5.);4],b);
 compare((0..=10).map(|x|Point::new(f64::from(x),5.)).collect(),b);
 compare((0..=10).map(|y|Point::new(5.,f64::from(y))).collect(),b);
 compare(vec![Point::new(0.,0.),Point::new(10.,0.),Point::new(10.,10.),Point::new(0.,10.),Point::new(5.,5.)],b);
 compare((0..4).flat_map(|x|(0..4).map(move|y|Point::new(f64::from(x)*3.,f64::from(y)*3.))).collect(),b);
 compare((0..12).map(|i|{let a=f64::from(i)*std::f64::consts::TAU/12.;Point::new(5.+4.*a.cos(),5.+4.*a.sin())}).collect(),b);
}
#[test] fn repeated_insert_remove_preserves_coverage(){let b=Bounds::new(0.,0.,100.,100.).unwrap();let sites=cli::random_sites(30,91,b);let mut d=Diagram::build(vec![],b).unwrap();for p in sites{d.insert(p).unwrap();d.validate(1e-8).unwrap();}while !d.sites().is_empty(){assert!(d.remove(d.sites().len()/2));d.validate(1e-8).unwrap();}}
