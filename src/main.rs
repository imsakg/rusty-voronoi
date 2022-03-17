use rusty_voronoi::{cli::{self,Options},geometry::Bounds,render::{self,SvgOptions},Diagram};
use std::io::Write;
fn run()->Result<(),Box<dyn std::error::Error>> {
 let options=Options::parse(std::env::args().skip(1))?;if options.help{print!("{}",cli::help());return Ok(());}
 let bounds=Bounds::new(0.,0.,f64::from(options.width),f64::from(options.height))?;
 let sites=if let Some(path)=options.sites.as_ref(){cli::parse_sites(&std::fs::read_to_string(path)?)?}else{cli::random_sites(options.count,options.seed,bounds)};
 if sites.len()>1000 {return Err("at most 1000 input sites are supported by the CLI".into());}
 let diagram=if options.algorithm=="fortune"{let(d,stats)=rusty_voronoi::fortune::build(sites,bounds)?;eprintln!("Fortune: {} site events, {} circle events, {} repaired constraints",stats.site_events,stats.circle_events,stats.repaired_constraints);d}else{Diagram::build(sites,bounds)?};
 diagram.validate(1e-7)?;let svg_options=SvgOptions{width:options.width,height:options.height,show_labels:options.labels,show_dual:options.dual,..Default::default()};
 let extension=std::path::Path::new(&options.output).extension().and_then(|s|s.to_str()).unwrap_or("svg");
 let bytes=match extension {"svg"=>render::svg_with_options(&diagram,&svg_options)?.into_bytes(),"html"=>render::html(&diagram,&svg_options)?.into_bytes(),"ppm"=>render::ppm(&diagram,options.width,options.height)?,_=>return Err("output extension must be svg, html, or ppm".into())};
 if options.output=="-"{std::io::stdout().lock().write_all(&bytes)?;}else{std::fs::write(&options.output,bytes)?;eprintln!("Wrote {} cells to {}",diagram.cells().len(),options.output);}
 Ok(())
}
fn main(){if let Err(error)=run(){eprintln!("error: {}",error);std::process::exit(2);}}
