// TODO : add locations
peg::parser! {
  grammar leek_prog() for str {

    rule ws() = quiet!{ ("\t" / " " / "\n")* }

    rule number() -> u32
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule ident() -> String
        = s:$(['a'..='z' | 'A'..='Z']+) { s.to_owned() }

    rule cst() -> EXPR
        = n:number() { EXPR::Cst(n) }

    rule var() -> EXPR
        = i:ident() { EXPR::Var(i) }

    rule ecall() -> EXPR
        = i:ident() "(" l:expr() ** (ws() "," ws()) ")" { EXPR::ECall(i, l) }

    rule expr() -> EXPR
        = cst() / ecall() / var()

    rule affect() -> STMT
        = i:ident() ws() "=" ws() e:expr() ws() ";" { STMT::Affect(i, e) }

    rule declr() -> STMT
        = "var" ws() i:ident() ws() "=" ws() e:expr() ws() ";" { STMT::Declr(i, e) }

    rule call() -> STMT
        = i:ident() "(" ws() l:expr() ** (ws() "," ws()) ws() ")" { STMT::Call(i, l) }

    rule stmt() -> STMT
        = ifElse() / (l:call() ";" { l })

    rule Else() -> Vec<STMT>
        = ws() "else" ws() "{" ws() l:stmt()+ ws() "}" { l }

    rule ifElse() -> STMT
        = ws() "if" ws() "(" c:expr() ")" ws() "{" ws() l:stmt()+  ws() "}" e:(Else())? { STMT::If(c, l, e) }

    pub rule list() -> Vec<EXPR>
        = "[" ws() l:expr() ** (ws() "," ws()) "]" { l }

    pub rule prog() -> Vec<STMT>
        = ws() l:stmt() ** (ws()) { l }
  }
}

#[derive(Debug)]
pub enum EXPR {
    Cst(u32),
    Var(String),
    ECall(String, Vec<EXPR>),
}

// TODO : add locations
#[derive(Debug)]
pub enum STMT {
    Declr(String, EXPR),
    Affect(String, EXPR),
    If(EXPR, Vec<STMT>, Option<Vec<STMT>>),
    While(EXPR, Vec<STMT>, Vec<STMT>),
    Defun(String, Vec<Box<str>>, Vec<STMT>),
    Call(String, Vec<EXPR>),
    Return(EXPR),
}

#[test]
fn test_main() {
    println!(
        "{:?}",
        leek_prog::prog("if (1) { print(a); } else { print(b); }")
    );
}
