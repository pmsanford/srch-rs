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
    let crt = miniparse::get_crate(source.clone(), args[1].clone());

    for item_ptr in crt.module.items.iter() {
        if item_ptr.ident.name.as_str() == args[2].as_slice() {
            println!("I found it. Somewhere.");
        }
    }
}
