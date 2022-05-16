use rusty_voronoi::{
    geometry::{Bounds, Point},
    Diagram,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut d = Diagram::build(vec![Point::new(2., 5.)], Bounds::new(0., 0., 10., 10.)?)?;
    d.insert(Point::new(8., 5.))?;
    d.validate(1e-8)?;
    for cell in d.cells() {
        println!(
            "site {}: area {}, centroid {:?}",
            cell.site,
            cell.area(),
            cell.centroid()
        );
    }
    println!("shared edges: {:?}", d.delaunay_edges());
    Ok(())
}
