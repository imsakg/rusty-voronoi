//! Command-line parsing and deterministic data generation.
#[derive(Debug,Clone)]
pub struct Options {pub width:u32,pub height:u32,pub algorithm:String,pub output:String,pub labels:bool,pub dual:bool,pub help:bool}
impl Default for Options {fn default()->Self{Self{width:800,height:800,algorithm:"clipping".into(),output:"voronoi.svg".into(),labels:false,dual:false,help:false}}}
impl Options {
 pub fn parse(args:impl IntoIterator<Item=String>)->Result<Self,String> {
  let mut options=Self::default();let mut args=args.into_iter();while let Some(arg)=args.next(){
   match arg.as_str(){
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
