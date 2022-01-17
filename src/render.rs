//! Portable SVG, HTML and raster visualization.
#[derive(Debug,Clone)]
pub struct SvgOptions {pub width:u32,pub height:u32,pub title:String,pub show_sites:bool,pub show_labels:bool,pub show_dual:bool}
impl Default for SvgOptions {fn default()->Self{Self{width:800,height:800,title:"Voronoi diagram".into(),show_sites:true,show_labels:false,show_dual:false}}}
fn escape(text:&str)->String {text.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;").replace('"',"&quot;").replace('\'',"&apos;")}
pub fn svg_with_options(diagram:&Diagram,options:&SvgOptions)->Result<String,&'static str> {
 if options.width==0||options.height==0{return Err("image dimensions must be positive");}
 let mut s=svg(diagram)?;let b=diagram.bounds();
 s=s.replacen("role=\"img\"",&format!("role=\"img\" width=\"{}\" height=\"{}\" aria-labelledby=\"diagram-title\"",options.width,options.height),1);
 let title=format!("<title id=\"diagram-title\">{}</title>",escape(&options.title));let end=s.find('>').unwrap()+1;s.insert_str(end,&title);
 if !options.show_sites {while let Some(start)=s.find("<circle "){let end=start+s[start..].find("/>").unwrap()+2;s.replace_range(start..end,"");}}
 let screen=|p:Point|Point::new(p.x-b.min().x,b.max().y-p.y);let mut layers=String::new();
 if options.show_dual {layers.push_str("<g data-layer=\"dual\" stroke=\"#181818\" stroke-dasharray=\"0.1 0.1\">");for(a,c) in diagram.delaunay_edges(){let p=screen(diagram.sites()[a]);let q=screen(diagram.sites()[c]);layers.push_str(&format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke-width=\"{}\"/>",p.x,p.y,q.x,q.y,b.width()/600.));}layers.push_str("</g>");}
 if options.show_labels {layers.push_str("<g data-layer=\"labels\">");for(i,&p) in diagram.sites().iter().enumerate(){let p=screen(p);layers.push_str(&format!("<text x=\"{}\" y=\"{}\" font-size=\"{}\">{}</text>",p.x,p.y,b.width()/45.,i));}layers.push_str("</g>");}
 s.insert_str(s.len()-6,&layers);Ok(s)
}
use crate::{Diagram,geometry::Point};
const PALETTE:[&str;7]=["#fb4934","#b8bb26","#fabd2f","#83a598","#d3869b","#8ec07c","#fe8019"];
pub fn svg(diagram:&Diagram)->Result<String,&'static str> {
 let b=diagram.bounds();let screen=|p:Point|Point::new(p.x-b.min().x,b.max().y-p.y);
 let mut out=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" role=\"img\">",b.width(),b.height());
 for cell in diagram.cells(){let points=cell.vertices.iter().map(|&p|{let p=screen(p);format!("{},{}",p.x,p.y)}).collect::<Vec<_>>().join(" ");out.push_str(&format!("<polygon data-cell=\"{}\" points=\"{}\" fill=\"{}\" stroke=\"#282828\" stroke-width=\"{}\"/>",cell.site,points,PALETTE[cell.site%PALETTE.len()],b.width().min(b.height())/500.));}
 for &p in diagram.sites(){let p=screen(p);out.push_str(&format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"#181818\"/>",p.x,p.y,b.width().min(b.height())/150.));}
 out.push_str("</svg>");Ok(out)
}
