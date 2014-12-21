extern crate miniparse;
extern crate syntax;
extern crate getopts;

use getopts::{optflag, getopts, OptGroup};
use std::os;
use std::io::{File, FileMode, FileAccess};
use std::io::fs::{PathExtensions, walk_dir};
use syntax::codemap::Span;
use syntax::ast::{
    Item_, Item, Method_, ImplItem, TraitItem, StructFieldKind
};
use syntax::ptr::P;

fn main() {
    let args: Vec<String> = os::args();
    let program = args[0].clone();

    let opts = &[
        optflag("a", "", "Match all"),
        optflag("x", "", "Top-level items"),
        optflag("m", "", "Methods (fns in impls)"),
        optflag("t", "", "Trait methods (fns in traits)"),
        optflag("f", "", "Struct fields"),
        ];

    let opt_strs = ["a", "x", "m", "t", "f"];
    let opt_strings: Vec<String> = 
        opt_strs.iter().map(|&x| x.to_string()).collect();

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()); }
    };

    let term = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(program.as_slice(), opts);
        return;
    };

    let match_all = 
        matches.opt_present("a") || 
        !matches.opts_present(opt_strings.as_slice());

    let tl = matches.opt_present("x") || match_all;
    let m = matches.opt_present("m") || match_all;
    let t = matches.opt_present("t") || match_all;
    let f = matches.opt_present("f") || match_all;

    let p = os::getcwd().unwrap();
    let srch_str = term.as_slice();
    for path in walk_dir(&p).unwrap() {
        if path.is_file() && path.as_str().unwrap().ends_with(".rs") {
            try_match(&p, &path, srch_str,
                     tl, m, t, f);
        }
    }
}

fn try_match(cwd: &Path, path: &Path, srch: &str,
             tl: bool, m: bool, t: bool, f: bool) {
    let mut file = File::open_mode(path, 
                                   FileMode::Open, 
                                   FileAccess::Read).unwrap();
    let source = file.read_to_string().unwrap();
    let path_str = String::from_str(path.as_str().unwrap());
    let mpr = miniparse::parse_crate(source.clone(), path_str);

    for item_ptr in mpr.cr.module.items.iter() {
        if item_ptr.ident.name.as_str().contains(srch) && tl {
            print_line(cwd, &mpr, item_ptr.span);
        }
        if m {
            match_impl_funcs(cwd, &mpr, item_ptr, srch);
        }
        if t {
            match_trait_funcs(cwd, &mpr, item_ptr, srch);
        }
        if f {
            match_struct_members(cwd, &mpr, item_ptr, srch);
        }
    }
}

fn match_struct_members(cwd: &Path, mpr: &miniparse::Miniresult,
                        item_ptr: &P<Item>, srch: &str) {
    if let Item_::ItemStruct(ref sdef, _) = item_ptr.node {
        for sitem in sdef.fields.iter() {
            if let StructFieldKind::NamedField(ref id, _) = sitem.node.kind {
                if id.name.as_str().contains(srch) {
                    print_line(cwd, mpr, sitem.span);
                }
            }
        }
    }
}

fn match_trait_funcs(cwd: &Path, mpr: &miniparse::Miniresult,
                     item_ptr: &P<Item>, srch: &str) {
    if let Item_::ItemTrait(_, _, _, _, ref traititems) = item_ptr.node {
        for titem in traititems.iter() {
            match *titem {
                TraitItem::RequiredMethod(ref tymethod) => {
                    if tymethod.ident.name.as_str().contains(srch) {
                        print_line(cwd, mpr, tymethod.span);
                    }
                },
                TraitItem::ProvidedMethod(ref ptymethod) => {
                    if let Method_::MethDecl(id, _, _, _, _, _, _, _) =
                        ptymethod.node {
                            if id.name.as_str().contains(srch) {
                                print_line(cwd, mpr, ptymethod.span);
                            }
                        }
                    },
                _ => {}
            }
        }
    }
}

fn match_impl_funcs(cwd: &Path, mpr: &miniparse::Miniresult, 
                    item_ptr: &P<Item>, srch: &str) {
    if let Item_::ItemImpl(_, _, _, _, ref impitems) = item_ptr.node {
        for iitem in impitems.iter() {
            if let ImplItem::MethodImplItem(ref meth) = *iitem {
                if let Method_::MethDecl(id, _, _, _, _, _, _, _) = meth.node {
                    if id.name.as_str().contains(srch) {
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

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} search_term", program);
}
