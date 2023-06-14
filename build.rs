extern crate kiki;
extern crate sha256;
extern crate walkdir;

use kiki::RustSrcRef;
use walkdir::WalkDir;

use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn main() {
    for entry in WalkDir::new("./src").follow_links(true) {
        let entry = entry.unwrap();

        if is_ignored(entry.path()) {
            continue;
        }

        if entry.path().extension() == Some(OsStr::new("kiki")) {
            let file_contents = fs::read_to_string(entry.path()).unwrap();
            let file_hash = sha256::digest(&*file_contents);

            let rs_path = entry
                .path()
                .parent()
                .unwrap()
                .join(Path::new(entry.path().file_stem().unwrap()).with_extension("rs"));
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
            if let Err(err) = fs::write(&rs_path, &rust_src.0) {
                let rs_path = rs_path.display();
                panic!("Cannot write to \"{rs_path}\". Error: {err:#?}")
            };
        }
    }
}

const IGNORE_LIST: [&str; 1] = ["./src/examples/should_fail"];

fn is_ignored(path: &Path) -> bool {
    IGNORE_LIST.iter().any(|ignored| path.starts_with(ignored))
}
