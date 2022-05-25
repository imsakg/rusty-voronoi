use std::{path::PathBuf, process::Command};
struct Sandbox(PathBuf);
impl Sandbox {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!("rusty-voronoi-exports-{}", std::process::id()));
        std::fs::create_dir_all(&p).unwrap();
        Self(p)
    }
}
impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rusty-voronoi"))
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn export_real_files_and_reject_invalid_inputs_without_writing() {
    let sandbox = Sandbox::new();
    for extension in ["svg", "html", "ppm"] {
        let path = sandbox.0.join(format!("diagram.{extension}"));
        let output = run(&[
            "--count",
            "6",
            "--seed",
            "9",
            "--width",
            "24",
            "--height",
            "16",
            "--algorithm",
            "fortune",
            "--output",
            path.to_str().unwrap(),
        ]);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let bytes = std::fs::read(path).unwrap();
        match extension {
            "svg" => assert!(bytes.starts_with(b"<svg ")),
            "html" => assert!(bytes.starts_with(b"<!doctype html>")),
            _ => assert_eq!(bytes.len(), b"P6\n24 16\n255\n".len() + 24 * 16 * 3),
        }
    }
    let points = sandbox.0.join("sites.csv");
    std::fs::write(&points, "2,2\n8,2\n5,8\n").unwrap();
    let path = sandbox.0.join("steps.html");
    let output = run(&[
        "--sites",
        points.to_str().unwrap(),
        "--width",
        "10",
        "--height",
        "10",
        "--steps",
        "--output",
        path.to_str().unwrap(),
    ]);
    assert!(output.status.success());
    let page = std::fs::read_to_string(path).unwrap();
    assert_eq!(page.matches("<template ").count(), 4);
    let rejected = sandbox.0.join("must-not-exist.svg");
    std::fs::write(&points, "11,2\n").unwrap();
    let bad = run(&[
        "--sites",
        points.to_str().unwrap(),
        "--width",
        "10",
        "--height",
        "10",
        "--output",
        rejected.to_str().unwrap(),
    ]);
    assert_eq!(bad.status.code(), Some(2));
    assert!(!rejected.exists());
    for args in [
        vec!["--width", "0"],
        vec!["--count", "1001"],
        vec!["--seed"],
        vec!["--algorithm", "bogus"],
        vec!["--output", "result.txt"],
        vec!["--count", "0", "--output", "-"],
    ] {
        let output = run(&args);
        if args.contains(&"0") && args.contains(&"--count") {
            assert!(output.status.success());
        } else {
            assert_eq!(output.status.code(), Some(2), "{args:?}");
            assert!(output.stdout.is_empty());
        }
    }
}
