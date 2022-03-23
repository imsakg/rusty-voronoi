//! Command-line parsing and deterministic data generation.
pub fn parse_sites(input:&str)->Result<Vec<crate::geometry::Point>,String> {
 let mut sites=Vec::new();for(index,line) in input.lines().enumerate(){let line=line.split('#').next().unwrap().trim();if line.is_empty(){continue;}let words:Vec<_>=line.split(|c:char|c==','||c.is_whitespace()).filter(|s|!s.is_empty()).collect();
  if words.len()!=2{return Err(format!("line {}: expected two coordinates",index+1));}
  let x=words[0].parse::<f64>().map_err(|_|format!("line {}: invalid x",index+1))?;let y=words[1].parse::<f64>().map_err(|_|format!("line {}: invalid y",index+1))?;
  let p=crate::geometry::Point::new(x,y);if !p.is_finite(){return Err(format!("line {}: coordinates must be finite",index+1));}sites.push(p);
 }Ok(sites)
}
pub fn random_sites(count:usize,seed:u64,bounds:crate::geometry::Bounds)->Vec<crate::geometry::Point> {
 use rand::{Rng,SeedableRng};let mut rng=rand::rngs::StdRng::seed_from_u64(seed);
 (0..count).map(|_|crate::geometry::Point::new(rng.gen_range(bounds.min().x..bounds.max().x),rng.gen_range(bounds.min().y..bounds.max().y))).collect()
}
#[derive(Debug,Clone)]
pub struct Options {pub relax:usize,pub sites:Option<String>,pub count:usize,pub seed:u64,pub width:u32,pub height:u32,pub algorithm:String,pub output:String,pub labels:bool,pub dual:bool,pub help:bool}
impl Default for Options {fn default()->Self{Self{relax:0,sites:None,count:32,seed:1,width:800,height:800,algorithm:"clipping".into(),output:"voronoi.svg".into(),labels:false,dual:false,help:false}}}
impl Options {
 pub fn parse(args:impl IntoIterator<Item=String>)->Result<Self,String> {
  let mut options=Self::default();let mut args=args.into_iter();while let Some(arg)=args.next(){
   match arg.as_str(){
    "--relax"=>{options.relax=args.next().ok_or("missing iteration count")?.parse().map_err(|_|"invalid iteration count")?;if options.relax>100{return Err("relax must be <=100".into());}},
    "--sites"=>options.sites=Some(args.next().ok_or("missing sites file")?),
    "--count"=>{options.count=args.next().ok_or("missing count")?.parse().map_err(|_|"invalid count")?;if options.count>1000{return Err("count must be <=1000".into());}},
    "--seed"=>options.seed=args.next().ok_or("missing seed")?.parse().map_err(|_|"invalid seed")?,
    "--help"|"-h"=>options.help=true,"--labels"=>options.labels=true,"--dual"=>options.dual=true,
    "--width"|"--height"=>{let value=args.next().ok_or_else(||format!("missing value for {}",arg))?.parse::<u32>().map_err(|_|format!("invalid dimension for {}",arg))?;if value==0||value>100_000{return Err("dimensions must be 1..=100000".into());}if arg=="--width"{options.width=value;}else{options.height=value;}},
    "--output"|"-o"=>options.output=args.next().ok_or("missing output path")?,
    "--algorithm"=>{options.algorithm=args.next().ok_or("missing algorithm")?;if !["clipping","fortune"].contains(&options.algorithm.as_str()){return Err("algorithm must be clipping or fortune".into());}},
    _=>return Err(format!("unknown option: {}",arg)),
   }
  }Ok(options)
 }
}
pub fn help()->&'static str {"Usage: rusty-voronoi [OPTIONS]\n  --width N --height N       Viewport dimensions (default 800)\n  --algorithm clipping|fortune\n  --output PATH             SVG, HTML or PPM path; - for stdout\n  --labels --dual           Show IDs and Delaunay overlay\n  --help                    Print help\n"}
