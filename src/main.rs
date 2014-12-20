extern crate miniparse;
extern crate syntax;

use std::os;
use std::io::{File, FileMode, FileAccess};
use std::io::fs::{PathExtensions, walk_dir};
use syntax::codemap::Span;
use syntax::ast::{Item_, Item, Method_, ImplItem};
use syntax::ptr::P;

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
            print_line(cwd, &mpr, item_ptr.span);
        }
        match_impl_funcs(cwd, &mpr, item_ptr, srch);
    }
}

fn match_impl_funcs(cwd: &Path, mpr: &miniparse::Miniresult, 
                    item_ptr: &P<Item>, srch: &str) {
    if let Item_::ItemImpl(_, _, _, _, ref impitems) = item_ptr.node {
        for iitem in impitems.iter() {
            if let ImplItem::MethodImplItem(ref meth) = *iitem {
                if let Method_::MethDecl(id, _, _, _, _, _, _, _) = meth.node {
                    if id.name.as_str() == srch {
                        print_line(cwd, mpr, meth.span);
                    }
                }
            }
        }
    }
}

fn print_line(cwd: &Path, mpr: &miniparse::Miniresult, spn: Span) {
    let pth = Path::new(mpr.file_map.name.clone());
    let relpath = pth.path_relative_from(cwd).unwrap();
    println!("{}:{}: {}", 
             relpath.display(),
             mpr.get_line_from_span(spn) + 1, // 0-indexed
             mpr.get_line_text_from_span(spn));
}
