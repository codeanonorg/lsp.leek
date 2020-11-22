use std::ops::Range;
// TODO : add locations
peg::parser! {
  grammar leek_prog() for str {

    rule ws() = quiet!{ ("\t" / " " / "\n")* }

    rule number() -> u32
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule ident() -> String
        = s:$(['a'..='z' | 'A'..='Z']+) { s.to_owned() }

    rule cst() -> EXPR
        = s:position!() ws() n:number() e:position!() { EXPR::Cst(s..e, n) }

    rule var() -> EXPR
        = s:position!() ws() i:ident() e:position!() { EXPR::Var(s..e, i) }

    rule ecall() -> EXPR
        = s:position!() i:ident() ws() "(" l:expr() ** (ws() ",") ws() ")" e:position!() { EXPR::ECall(s..e, i, l) }

    rule expr() -> EXPR
        = cst() / ecall() / var()

    rule affect() -> STMT
        = ws() s:position!() i:ident() ws() "=" ws() e:expr() ws() ";" end:position!() { STMT::Affect(s..end, i, e) }

    rule declr() -> STMT
        = s:position!() "var" ws() i:ident() ws() "=" ws() e:expr() ws() ";" end:position!() { STMT::Declr(s..end, i, e) }

    rule call() -> STMT
        = ws() s:position!() i:ident() "(" ws() l:expr() ** (ws() "," ws()) ws() ")" e:position!() { STMT::Call(s..e, i, l) }

    rule stmt() -> STMT
        = ifElse() / (l:call() ";" { l })

    rule Else() -> Vec<STMT>
        = ws() "else" ws() "{" ws() l:stmt()+ ws() "}" { l }

    rule ifElse() -> STMT
        = ws() s:position!() "if" ws() "(" c:expr() ")" ws() "{" ws() l:stmt()+  ws() "}" e:(Else())? end:position!() { STMT::If(s..end, c, l, e) }

    pub rule list() -> EXPR
        = s:position!() "[" ws() l:expr() ** (ws() "," ws()) "]" e:position!() { EXPR::List(s..e, l) }

    pub rule prog() -> Vec<STMT>
        = ws() l:stmt() ** (ws()) { l }
  }
}

pub fn parse(text: &str) -> Result<Vec<STMT>, peg::error::ParseError<peg::str::LineCol>> {
    leek_prog::prog(text)
}

#[derive(Clone, Debug)]
pub enum EXPR {
    Cst(Range<usize>, u32),
    Var(Range<usize>, String),
    List(Range<usize>, Vec<EXPR>),
    ECall(Range<usize>, String, Vec<EXPR>),
}

impl EXPR {
    pub fn get_range(&self) -> Range<usize> {
        match self {
            Self::Cst(r, ..) => r,
            Self::Var(r, ..) => r,
            Self::ECall(r, ..) => r,
            Self::List(r, ..) => r,
        }.clone()
    }
}

#[derive(Clone, Debug)]
pub enum STMT {
    Declr(Range<usize>, String, EXPR),
    Affect(Range<usize>, String, EXPR),
    If(Range<usize>, EXPR, Vec<STMT>, Option<Vec<STMT>>),
    While(Range<usize>, EXPR, Vec<STMT>),
    Defun(Range<usize>, String, Vec<String>, Vec<STMT>),
    Call(Range<usize>, String, Vec<EXPR>),
    Return(Range<usize>, EXPR),
}

impl STMT {
    pub fn get_range(&self) -> Range<usize> {
        match self {
            Self::Declr(r, ..) => r,
            Self::Affect(r, ..) => r,
            Self::If(r, ..) => r,
            Self::While(r, ..) => r,
            Self::Defun(r, ..) => r,
            Self::Call(r, ..) => r,
            Self::Return(r, ..) => r,
        }.clone()
    }
}

#[test]
fn test_main() {
    println!(
        "{:?}",
        leek_prog::prog("if (1) { print(a); } else { print(b); }")
    );
}
