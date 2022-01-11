//! Portable SVG, HTML and raster visualization.
use crate::{Diagram,geometry::Point};
const PALETTE:[&str;7]=["#fb4934","#b8bb26","#fabd2f","#83a598","#d3869b","#8ec07c","#fe8019"];
pub fn svg(diagram:&Diagram)->Result<String,&'static str> {
 let b=diagram.bounds();let screen=|p:Point|Point::new(p.x-b.min().x,b.max().y-p.y);
 let mut out=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" role=\"img\">",b.width(),b.height());
 for cell in diagram.cells(){let points=cell.vertices.iter().map(|&p|{let p=screen(p);format!("{},{}",p.x,p.y)}).collect::<Vec<_>>().join(" ");out.push_str(&format!("<polygon data-cell=\"{}\" points=\"{}\" fill=\"{}\" stroke=\"#282828\" stroke-width=\"{}\"/>",cell.site,points,PALETTE[cell.site%PALETTE.len()],b.width().min(b.height())/500.));}
 for &p in diagram.sites(){let p=screen(p);out.push_str(&format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"#181818\"/>",p.x,p.y,b.width().min(b.height())/150.));}
 out.push_str("</svg>");Ok(out)
}
