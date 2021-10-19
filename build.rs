extern crate codegen;
use codegen::Scope;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let entries = fs::read_dir("src/modules").unwrap();

    let mut scope = Scope::new();

    let names: Vec<String> = entries.map(|entry|
            entry.unwrap()
            .path().file_stem().unwrap()
            .to_string_lossy().to_string()).collect();

    for name in &names {
        scope.raw(&format!(
            "#[path = \"../../../../../src/modules/{}.rs\"] mod {};",
            name, name
        ));
    }
    scope.raw(&format!("
        pub async fn instantiate_all() -> Result<[(String, DynModule); {}], Error> {{
            Ok([", names.len()));
    for name in &names {
        scope.raw(&format!(r#"("{}".to_string(), {}::instantiate().await?),"#, name, name));
    }
    scope.raw("
            ])
        }
    ");
    let output = scope.to_string();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("modimports.rs");
    std::fs::write(dest_path, output.as_bytes()).unwrap();
}
