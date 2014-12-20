extern crate miniparse;

use std::os;
use std::io::{File, FileMode, FileAccess};

fn main() {
    let args: Vec<String> = os::args();
    let p = Path::new(args[1].clone());
    let mut file = File::open_mode(&p, 
                                   FileMode::Open, 
                                   FileAccess::Read).unwrap();
    let source = file.read_to_string().unwrap();
    let mpr = miniparse::parse_crate(source.clone(), args[1].clone());

    for item_ptr in mpr.cr.module.items.iter() {
        if item_ptr.ident.name.as_str() == args[2].as_slice() {
            println!("{}:{}: {}", 
                     mpr.file_map.name,
                     mpr.get_line_from_span(item_ptr.span) + 1, // 0-indexed
                     mpr.get_line_text_from_span(item_ptr.span));
        }
    }
}
