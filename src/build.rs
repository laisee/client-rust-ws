use std::env;
use std::fs;
use std::path::Path;
use chrono::Utc;

/// Generates a build date file at compile time
///
/// # Panics
///
/// Panics if unable to write to the output directory
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_date.rs");
    
    // Change to a function instead of a const declaration
    let build_date = format!("pub fn build_date() -> &'static str {{ \"{}\" }}", 
                            Utc::now().format("%Y-%m-%d"));
    
    fs::write(dest_path, build_date).unwrap();
}
