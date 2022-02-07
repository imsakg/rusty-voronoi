//! Portable SVG, HTML and raster visualization.
pub fn html_frames(frames:&[Diagram],options:&SvgOptions)->Result<String,&'static str> {
 let first=frames.first().ok_or("at least one frame is required")?;
 if frames.iter().any(|d|d.bounds()!=first.bounds()) {return Err("all frames must share bounds");}
 let mut page=html(first,options)?;let mut controls=format!("<label>Insertion step <input id=\"step\" type=\"range\" min=\"0\" max=\"{}\" value=\"0\"></label>",frames.len()-1);
 let mut options=options.clone();options.show_dual=true;options.show_labels=true;options.show_sites=true;
 for(i,diagram) in frames.iter().enumerate(){controls.push_str(&format!("<template id=\"frame-{}\">{}</template>",i,svg_with_options(diagram,&options)?));}
 controls.push_str(r#"<script>document.getElementById('step').addEventListener('input',event=>{const t=document.getElementById('frame-'+event.target.value);document.getElementById('viewer').replaceChildren(t.content.cloneNode(true));document.getElementById('selection').textContent='Insertion step '+event.target.value;layers();});</script>"#);
 page=page.replace("</body>",&format!("{}</body>",controls));Ok(page)
}
pub fn html(diagram:&Diagram,options:&SvgOptions)->Result<String,&'static str> {
 let mut options=options.clone();options.show_dual=true;options.show_labels=true;options.show_sites=true;
 let drawing=svg_with_options(diagram,&options)?;
 Ok(format!(r##"<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{}</title>
<style>body{{font:16px system-ui;margin:2rem auto;max-width:960px;background:#181818;color:#eee;padding:0 1rem}}svg{{width:100%;height:auto;background:#282828;border-radius:8px}}header{{display:flex;flex-wrap:wrap;gap:1rem;align-items:center}}polygon:hover{{filter:brightness(1.2);cursor:pointer}}output{{display:block;padding:1rem 0}}[data-layer="labels"]{{pointer-events:none}}</style>
<body><h1>{}</h1><header><label><input type="checkbox" id="show-dual" checked> Dual edges</label><label><input type="checkbox" id="show-labels" checked> Site IDs</label><label><input type="checkbox" id="show-sites" checked> Sites</label></header><output id="selection" aria-live="polite">Select a cell to inspect its site ID.</output><main id="viewer">{}</main>
<script>
function layers(){{for(const [id,selector] of [['show-dual','[data-layer="dual"]'],['show-labels','[data-layer="labels"]'],['show-sites','circle']]){{for(const el of document.querySelectorAll(selector))el.style.display=document.getElementById(id).checked?'':'none';}}}}
for(const input of document.querySelectorAll('header input'))input.addEventListener('change',layers);
document.addEventListener('click',event=>{{const p=event.target.closest('[data-cell]');if(p)document.getElementById('selection').textContent='Cell '+p.dataset.cell;}});
</script></body></html>"##,escape(&options.title),escape(&options.title),drawing))
}
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
