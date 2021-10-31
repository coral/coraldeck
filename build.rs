extern crate codegen;
use codegen::{Block, Scope};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // let entries = fs::read_dir("src/modules").unwrap();

    // let mut scope = Scope::new();
    // scope.import("std", "fmt");

    // let names: Vec<String> = entries
    //     .map(|entry| {
    //         entry
    //             .unwrap()
    //             .path()
    //             .file_stem()
    //             .unwrap()
    //             .to_string_lossy()
    //             .to_string()
    //     })
    //     .collect();

    // //Import modules
    // for name in &names {
    //     scope.raw(&format!(
    //         "#[path = \"../../../../../src/modules/{}.rs\"] mod {};",
    //         name, name
    //     ));
    // }

    // //instantiate_all function block
    // {
    //     let mut fnb = scope
    //         .new_fn("instantiate_all")
    //         .vis("pub")
    //         .set_async(true)
    //         .ret(&format!(
    //             "Result<[(String, DynModule); {}], Error>",
    //             names.len()
    //         ));

    //     fnb.line("Ok([");
    //     for name in &names {
    //         fnb.line(&format!(
    //             r#"("{}".to_string(), {}::instantiate().await?),"#,
    //             name, name
    //         ));
    //     }
    //     fnb.line("])");
    // }

    // //instantiate function block
    // {
    //     let mut fnb = scope
    //         .new_fn("instantiate")
    //         .vis("pub")
    //         .arg("module_name", "&str")
    //         .set_async(true)
    //         .ret("Result<DynModule, Error>");

    //     let mut match_block = Block::new("match module_name");

    //     for name in &names {
    //         match_block.line(&format!(
    //             r#""{}" => Ok({}::instantiate().await?),"#,
    //             name, name
    //         ));
    //     }
    //     match_block.line("_ => { return Err(Error::ModuleNotFound(module_name.to_string()))}");

    //     fnb.push_block(match_block);
    // }

    // let output = scope.to_string();

    // let out_dir = env::var_os("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir).join("modimports.rs");
    // std::fs::write(dest_path, output.as_bytes()).unwrap();
}
