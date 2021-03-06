extern crate syntax;
extern crate rustc;

use std::rc::Rc;
use rustc::session::config::{CrateType, Options};
use rustc::session;
use rustc::session::Session;
use syntax::ast::{Crate, CrateConfig};
use syntax::codemap::{Span, FileMap};
use syntax::diagnostics::registry::Registry;
use syntax::parse;
use syntax::parse::lexer::StringReader;
use syntax::parse::ParseSess;
use syntax::parse::parser::Parser;

fn get_session_config_options() -> Options {
    Options {
        crate_types: vec![CrateType::CrateTypeRlib],
        .. session::config::basic_options()
    }
}

fn get_registry() -> Registry {
    Registry::new(&rustc::DIAGNOSTICS)
}

fn get_rustc_session(opt: Options, reg: Registry) -> Session {
    session::build_session(opt, None, reg)
}

fn create_rustc_config(session: &Session) -> CrateConfig {
    session::config::build_configuration(session)
}

fn get_rustc_config() -> CrateConfig {
    let opt = get_session_config_options();
    let reg = get_registry();
    let session = get_rustc_session(opt, reg);
    create_rustc_config(&session)
}

pub fn get_parse_sess() -> ParseSess {
    parse::new_parse_sess()
}

fn get_filemap(parse_session: &ParseSess, 
               source: String, 
               path: String) -> Rc<FileMap> {
    parse::string_to_filemap(parse_session, source, path)
}

fn get_lexer(parse_session: &ParseSess,
             fm: Rc<FileMap>) -> StringReader {
    StringReader::new(&parse_session.span_diagnostic, fm)
}

pub fn get_parser(parse_session: &ParseSess, 
              source: String, path: String) -> Parser {
    let fm = get_filemap(parse_session, source, path);
    let lexer = get_lexer(parse_session, fm);
    let cfg = get_rustc_config();
    Parser::new(parse_session, cfg, box lexer)
}

pub fn get_crate_from_session(parse_session: &ParseSess,
                              source: String,
                              path: String) -> Crate {
    let mut parser = get_parser(parse_session, source, path);
    parser.parse_crate_mod()
}

pub fn get_crate(source: String, path: String) -> Crate {
    let parse_session = get_parse_sess();
    get_crate_from_session(&parse_session, source, path)
}

pub fn parse_crate(source: String, path: String) -> Miniresult {
    Miniresult::new(source, path)
}

pub struct Miniresult {
    pub session: ParseSess,
    pub file_map: Rc<FileMap>,
    pub cr: Crate
}

impl Miniresult {
    pub fn new(source: String, path: String) -> Miniresult {
        let parse_session = get_parse_sess();
        let fm = get_filemap(&parse_session, source, path);
        let lexer = get_lexer(&parse_session, fm.clone());
        let cfg = get_rustc_config();
        let mut parser = Parser::new(&parse_session, cfg, box lexer);

        Miniresult {
            session: get_parse_sess(),
            file_map: fm,
            cr: parser.parse_crate_mod()
        }
    }

    pub fn get_line_text_from_span(&self, spn: Span) -> String {
        let no = self.get_line_from_span(spn);

        self.file_map.get_line(no).unwrap()
    }

    pub fn get_line_from_span(&self, spn: Span) -> uint {
        let pos = spn.lo;
        let mut a = 0u;
        {
            let lines = self.file_map.lines.borrow();
            let mut b = lines.len();
            while b - a > 1u {
                let m = (a + b) / 2u;
                if (*lines)[m] > pos { b = m; } else { a = m; }
            }
        }
        a
    }
}
