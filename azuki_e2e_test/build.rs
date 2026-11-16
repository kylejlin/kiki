use azuki::convenience::azuki_to_rust;
use azuki::source_text::FileSource;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    for case in ["balanced_parens", "borrowed_tokens", "lifetime_subsets"] {
        let grammar_path = format!("src/{case}/grammar.azuki");
        println!("cargo::rerun-if-changed={grammar_path}");

        let source = fs::read_to_string(&grammar_path).unwrap();
        // Always regenerate when this script runs: the local generator can change
        // without changing the grammar or Azuki's version.
        let rust_src = match azuki_to_rust(FileSource::new(&source), &grammar_path) {
            Ok(src) => src,
            Err(err) => panic!("Invalid Azuki file {grammar_path}. Error: {err}"),
        };

        // Preserve Azuki's inner attributes when loading the module with include!.
        let rust_src = format!("mod parser {{\n{}\n}}\n", rust_src.raw);
        let rs_path = out_dir.join(format!("{case}.rs"));
        if let Err(err) = fs::write(&rs_path, rust_src) {
            let rs_path = rs_path.display();
            panic!("Cannot write to \"{rs_path}\". Error: {err}");
        }
    }
}
