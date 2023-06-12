extern crate kiki;
extern crate sha256;
extern crate walkdir;

use kiki::RustSrcRef;
use walkdir::WalkDir;

use std::fs;
use std::path::Path;

fn main() {
    for entry in WalkDir::new("./src") {
        let entry = entry.unwrap();
        if entry.path().ends_with(".kiki") {
            let file_contents = fs::read_to_string(entry.path()).unwrap();
            let file_hash = sha256::digest(&*file_contents);

            let rs_path = Path::new(entry.path().file_stem().unwrap()).join(".rs");
            if let Ok(rs_contents) = fs::read_to_string(&rs_path) {
                let rs_contents = RustSrcRef(&rs_contents);
                if kiki::get_grammar_hash(rs_contents) == Some(&file_hash) {
                    // The .kiki file has not changed.
                    // Therefore, we don't need to regenerate the .rs file.
                    continue;
                }
            }

            let rust_src = match kiki::generate(&file_contents) {
                Ok(s) => s,
                Err(err) => {
                    let file_path = entry.path().display();
                    panic!("Invalid Kiki file {file_path}. Error: {err:#?}");
                }
            };
            fs::write(rs_path, &rust_src.0).unwrap();
        }
    }
}
