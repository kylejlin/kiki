use azuki::convenience::azuki_to_rust;
use azuki::source_text::FileSource;
use walkdir::WalkDir;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    println!("cargo::rerun-if-changed=src");

    for entry in WalkDir::new("./src").follow_links(true) {
        let entry = entry.unwrap();

        if entry.path().extension() == Some(OsStr::new("azuki")) {
            let file_contents = fs::read_to_string(entry.path()).unwrap();

            let rs_path = out_dir
                .join(entry.path().strip_prefix("./src").unwrap())
                .with_extension("rs");
            fs::create_dir_all(rs_path.parent().unwrap()).unwrap();

            let file_path = entry.path().display().to_string();
            let rust_src = match azuki_to_rust(FileSource::new(&file_contents), &file_path) {
                Ok(s) => s,
                Err(err) => {
                    panic!("Invalid Azuki file {file_path}. Error: {err}");
                }
            };

            // Preserve Azuki's inner attributes when loading the module with include!.
            let module_name = entry.path().file_stem().unwrap().to_str().unwrap();
            let rust_src = format!("mod {module_name} {{\n{}\n}}\n", rust_src.raw);
            if let Err(err) = fs::write(&rs_path, &rust_src) {
                let rs_path = rs_path.display();
                panic!("Cannot write to \"{rs_path}\". Error: {err:#?}")
            };
        }
    }
}
