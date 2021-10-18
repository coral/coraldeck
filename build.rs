extern crate codegen;
use codegen::Scope;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let paths = fs::read_dir("src/modules").unwrap();

    let mut scope = Scope::new();

    for path in paths {
        let path = path.unwrap().path();
        let name = path.file_stem().unwrap().to_string_lossy();
        scope.raw(&format!(
            "#[path = \"../../../../../src/modules/{}.rs\"] mod {};",
            name, name
        ));
    }
    let output = scope.to_string();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("modimports.rs");
    std::fs::write(dest_path, output.as_bytes()).unwrap();
}
