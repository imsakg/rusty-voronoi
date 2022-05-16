use rusty_voronoi::{
    cli, fortune,
    geometry::Bounds,
    render::{self, SvgOptions},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bounds = Bounds::new(0., 0., 800., 600.)?;
    let (diagram, stats) = fortune::build(cli::random_sites(50, 17, bounds), bounds)?;
    diagram.validate(1e-8)?;
    eprintln!("{:?}", stats);
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "fortune.svg".into());
    std::fs::write(
        path,
        render::svg_with_options(
            &diagram,
            &SvgOptions {
                show_dual: true,
                ..Default::default()
            },
        )?,
    )?;
    Ok(())
}
