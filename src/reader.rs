
use std::fmt::Write;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("calc.l");
lrpar_mod!("calc.y");

use crate::clog::Predicate;

pub fn parse(s: &str) -> Result<Predicate,String> {
    let lexerdef = calc_l::lexerdef();
    let lexer = lexerdef.lexer(s);
    let (res, errs) = calc_y::parse(&lexer);
    let mut s = String::new();
    for e in errs {
        s.write_fmt(format_args!("{:?}\n", e)).unwrap();
    }
    if !s.is_empty() {
        return Err(s);
    }

    match res {
        Option::Some(Err(())) |
        Option::None => Err(format!("nothing returned to parsed, invalid input")),
        Option::Some(Ok(p)) => Ok(p)
    }
}
