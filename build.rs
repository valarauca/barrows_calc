use cfgrammar::yacc::YaccKind;
use lrlex::CTLexerBuilder;
use lrpar::RecoveryKind;


fn main() {
    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Grmtools)
                .recoverer(RecoveryKind::None)
                .grammar_in_src_dir("calc.y")
                .unwrap()
        })
        .lexer_in_src_dir("calc.l")
        .unwrap()
        .build()
        .unwrap();
}
