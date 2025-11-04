/// Build script to auto-discover migration files
/// This runs at compile time and generates the migrations list automatically
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=migrations/");
    
    // Read all .sql files from migrations directory
    let migrations_dir = Path::new("migrations");
    
    if !migrations_dir.exists() {
        panic!("Migrations directory not found!");
    }
    
    let mut migrations: Vec<String> = fs::read_dir(migrations_dir)
        .expect("Failed to read migrations directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            
            // Only include .sql files
            if path.extension()? == "sql" {
                let filename = path.file_name()?.to_str()?.to_string();
                Some(filename)
            } else {
                None
            }
        })
        .collect();
    
    // Sort migrations by filename (ensures correct order)
    migrations.sort();
    
    let migration_count = migrations.len();
    
    // Get absolute path to migrations directory
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations_abs_path = Path::new(&manifest_dir).join("migrations");
    
    // Generate Rust code for the macro with absolute paths
    let mut code = String::from("vec![\n");
    for migration in migrations {
        let full_path = migrations_abs_path.join(&migration);
        let full_path_str = full_path.to_str().unwrap().replace("\\", "\\\\");
        code.push_str(&format!(
            "        (\"{}\", include_str!(r\"{}\")),\n",
            migration, full_path_str
        ));
    }
    code.push_str("    ]");
    
    // Write to a file that will be included
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("migrations.rs");
    fs::write(&dest_path, code).unwrap();
    
    println!("✅ Auto-discovered {} migrations", migration_count);
}
