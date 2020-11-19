pub mod ast;
use ast::*;

peg::parser! {
  grammar leek_prog() for str {

    rule _() = quiet!{ ("\t" / " " / "\n")* }

    rule number() -> u32
        = n:$(['0'..='9']+)
        { n.parse().unwrap() }

    rule ident() -> String
        = s:$(['a'..='z' | 'A'..='Z']+)
        { s.to_owned() }

    rule cst() -> EXPR
        = _ p:position!() n:number()
        { unsafe { EXPR::Cst(p, n) } }

    rule var() -> EXPR
        = _ p:position!() i:ident()
        { unsafe { EXPR::Var(p, i) } }

    rule ecall() -> EXPR
        = p:position!() i:ident() "(" l:expr() ** (_ "," _) ")"
        { unsafe { EXPR::ECall(p, i, l) } }

    rule expr() -> EXPR
        = cst() / ecall() / var()

    rule affect() -> STMT
        = _ p:position!() i:ident() _ "=" _ e:expr() _ ";"
        { unsafe { STMT::Affect(p, i, e) } }

    rule declr() -> STMT
        = _ p:position!() "var" _ i:ident() _ "=" _ e:expr() _ ";"
        { unsafe { STMT::Declr(p, i, e) } }

    rule call() -> STMT
        = _ p:position!() i:ident() "(" _ l:expr() ** (_ "," _) _ ")"
        { unsafe { STMT::Call(p, i, l) } }

    rule stmt() -> STMT
        = ifElse() / declr() / affect() / (l:call() ";" { l })

    rule Else() -> Vec<STMT>
        = _ "else" _ "{" _ l:stmt()+ _ "}"
        { l }

    rule ifElse() -> STMT
        = _ p:position!() "if" _ "(" c:expr() ")" _ "{" _ l:stmt()+  _ "}" e:(Else())?
        { unsafe { STMT::If(p, c, l, e) } }

    pub rule list() -> Vec<EXPR>
        = "[" _ l:expr() ** (_ "," _) "]"
        { l }

    pub rule prog() -> Vec<STMT>
        = _ l:stmt() ** _
        { l }
  }
}

#[test]
fn test_main() {
    let inp = "var a = 3;\nvar b = 3;\nif (a) { print(a); }";
    let res: Result<Vec<STMT>, _> = leek_prog::prog(inp);
    match res {
        Ok(ast) => {
            for s in ast.iter() {
                println!("{}", SPROG::new(String::from(inp), s))
            }
        }
        Err(err) => println!("{:?}", err),
    }
}
