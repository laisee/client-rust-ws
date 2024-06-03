use std::env;
use std::fs;
use std::path::Path;
use chrono::Utc;

/// .
///
/// # Panics
///
/// Panics if .
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_date.rs");
    let build_date = format!("const BUILD_DATE: &str = \"{}\";", Utc::now().format("%Y-%m-%d"));
    fs::write(&dest_path, build_date).unwrap();
}
