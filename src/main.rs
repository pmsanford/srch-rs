extern crate miniparse;

use std::os;
use std::io::{File, FileMode, FileAccess};
use std::io::fs::{PathExtensions, walk_dir};

fn main() {
    let args: Vec<String> = os::args();
    let p = os::getcwd().unwrap();
    let srch_str = args[1].as_slice();
    for path in walk_dir(&p).unwrap() {
        if path.is_file() && path.as_str().unwrap().ends_with(".rs") {
            try_match(&p, &path, srch_str);
        }
    }
}

fn try_match(cwd: &Path, path: &Path, srch: &str) {
    let mut file = File::open_mode(path, 
                                   FileMode::Open, 
                                   FileAccess::Read).unwrap();
    let source = file.read_to_string().unwrap();
    let path_str = String::from_str(path.as_str().unwrap());
    let mpr = miniparse::parse_crate(source.clone(), path_str);

    for item_ptr in mpr.cr.module.items.iter() {
        if item_ptr.ident.name.as_str() == srch {
            let pth = Path::new(mpr.file_map.name.clone());
            let relpath = pth.path_relative_from(cwd).unwrap();
            println!("{}:{}: {}", 
                     relpath.display(),
                     mpr.get_line_from_span(item_ptr.span) + 1, // 0-indexed
                     mpr.get_line_text_from_span(item_ptr.span));
        }
    }
}
