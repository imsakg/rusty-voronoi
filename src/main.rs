use rusty_voronoi::{
    cli::{self, Options},
    geometry::Bounds,
    render::{self, SvgOptions},
    Diagram,
};
use std::io::Write;
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse(std::env::args().skip(1))?;
    if options.help {
        print!("{}", cli::help());
        return Ok(());
    }
    let bounds = Bounds::new(0., 0., f64::from(options.width), f64::from(options.height))?;
    let sites = if let Some(path) = options.sites.as_ref() {
        cli::parse_sites(&std::fs::read_to_string(path)?)?
    } else {
        cli::random_sites(options.count, options.seed, bounds)
    };
    if sites.len() > 1000 {
        return Err("at most 1000 input sites are supported by the CLI".into());
    }
    let playback_sites = sites.clone();
    let diagram = if options.algorithm == "fortune" {
        let (d, stats) = rusty_voronoi::fortune::build(sites, bounds)?;
        eprintln!(
            "Fortune: {} site events, {} circle events, {} repaired constraints",
            stats.site_events, stats.circle_events, stats.repaired_constraints
        );
        d
    } else {
        Diagram::build(sites, bounds)?
    };
    let diagram = diagram.relax(options.relax)?;
    diagram.validate(1e-7)?;
    let svg_options = SvgOptions {
        width: options.width,
        height: options.height,
        show_labels: options.labels,
        show_dual: options.dual,
        ..Default::default()
    };
    let extension = std::path::Path::new(&options.output)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("svg");
    if options.steps && extension != "html" {
        return Err("--steps requires an HTML output path".into());
    }
    if options.steps && options.relax > 0 {
        return Err("--steps cannot be combined with --relax".into());
    }
    let mut frames = Vec::new();
    if options.steps {
        if playback_sites.len() > 200 {
            return Err("playback is limited to 200 sites".into());
        }
        let mut builder = rusty_voronoi::diagram::Builder::new(bounds);
        frames.push(builder.diagram().clone());
        for point in playback_sites {
            builder.push(point)?;
            frames.push(builder.diagram().clone());
        }
    }
    let bytes = match extension {
        "svg" => render::svg_with_options(&diagram, &svg_options)?.into_bytes(),
        "html" => {
            if options.steps {
                render::html_frames(&frames, &svg_options)?.into_bytes()
            } else {
                render::html(&diagram, &svg_options)?.into_bytes()
            }
        }
        "ppm" => render::ppm(&diagram, options.width, options.height)?,
        _ => return Err("output extension must be svg, html, or ppm".into()),
    };
    if options.output == "-" {
        std::io::stdout().lock().write_all(&bytes)?;
    } else {
        std::fs::write(&options.output, bytes)?;
        eprintln!(
            "Wrote {} cells to {}",
            diagram.cells().len(),
            options.output
        );
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("error: {}", error);
        std::process::exit(2);
    }
}
