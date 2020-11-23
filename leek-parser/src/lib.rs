use std::borrow::Cow;
use std::ops::Range;

peg::parser! {
  grammar leek_prog() for str {

    rule ws() = quiet!{ "//" (!"\n" [_])* / ("\t" / " " / "\n")* }

    rule number() -> u32 = quiet! {
            n:$(['0'..='9']+) {? n.parse().map_err(|_| "cannot parse integer") }
        } / expected!("number")

    rule ident() -> String = quiet!{
            s:$(['a'..='z' | 'A'..='Z']+) { s.to_owned() }
        } / expected!("identifier")

    rule cst() -> EXPR
        = s:position!() ws() n:number() e:position!() { EXPR::Cst(s..e, n) }

    rule var() -> EXPR
        = s:position!() ws() i:ident() e:position!() { EXPR::Var(s..e, i) }

    rule ecall() -> EXPR
        = s:position!() i:ident() ws() "(" l:expr() ** (ws() ",") ws() ")" e:position!() { EXPR::ECall(s..e, i, l) }

    rule list() -> EXPR
        = s:position!() "[" ws() l:expr() ** (ws() "," ws()) "]" e:position!() { EXPR::List(s..e, l) }

    rule atom() -> EXPR
            = list() / cst() / ecall() / var()

    rule expr() -> EXPR = precedence!{
        /*s:position!() x:(@) ws() "-" y:@ end:position!() { EXPR::Infix(s..end, Box::new(x), Op::Subtract, Box::new(y)) }
        s:position!() x:(@) ws() "+" y:@ end:position!() { EXPR::Infix(s..end, Box::new(x), Op::Add, Box::new(y)) }
        --
        s:position!() x:(@) ws() "/" y:@ end:position!() { EXPR::Infix(s..end, Box::new(x), Op::Divide, Box::new(y)) }
        s:position!() x:(@) ws() "*" y:@ end:position!() { EXPR::Infix(s..end, Box::new(x), Op::Multiply, Box::new(y)) }
        --
        s:position!() x:@ ws() "**" y:(@) end:position!() { EXPR::Infix(s..end, Box::new(x), Op::Power, Box::new(y)) }
        --*/
        ws() s:position!() "!" e:expr() end:position!() { EXPR::Prefix(s..end, PrefixOp::Not, Box::new(e)) }
        --
        a:atom() { a }
        "(" e:expr() ")" { e }
    } / expected!("expression")

    rule affect() -> STMT
        = ws() s:position!() i:ident() ws() "=" ws() e:expr() ws() ";" end:position!() { STMT::Affect(s..end, i, e) }

    rule declr() -> STMT
        = s:position!() "var" ws() i:ident() ws() "=" ws() e:expr() ws() ";" end:position!() { STMT::Declr(s..end, i, e) }

    rule call() -> STMT
        = ws() s:position!() i:ident() "(" ws() l:expr() ** (ws() "," ws()) ws() ")" e:position!() { STMT::Call(s..e, i, l) }

    rule while_() -> STMT
        = ws() s:position!() "while" ws() "(" e:expr() ws() ")" b:block() end:position!() { STMT::While(s..end, e, b) }

    rule block() -> Vec<STMT>
        = ws() "{" p:stmts() ws() "}" { p }
        / s:stmt() { vec![s] }
        / expected!("block")

    rule stmt() -> STMT
        = declr() / while_() / ifElse() / (l:call() ";" { l }) / expected!("statement")

    rule Else() -> Vec<STMT>
        = ws() "else" ws() l:block() { l }

    rule ifElse() -> STMT
        = ws() s:position!() "if" ws() "(" c:expr() ")" ws() l:block() e:(Else())? end:position!() { STMT::If(s..end, c, l, e) }

    rule stmts() -> Vec<STMT>
        = ws() l:stmt() ** (ws()) { l }

    rule eof() = ws() quiet!{ ![_] }

    pub rule prog() -> Vec<STMT> = s:stmts() eof() { s }
  }
}

pub fn parse(text: &str) -> Result<Vec<STMT>, peg::error::ParseError<peg::str::LineCol>> {
    leek_prog::prog(text)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeQuery<'a> {
    Statement(&'a STMT),
    Expression(&'a EXPR),
    NotFound,
}

impl<'a> From<Option<&'a EXPR>> for NodeQuery<'a> {
    fn from(o: Option<&'a EXPR>) -> Self {
        match o {
            Some(e) => Self::Expression(e),
            None => Self::NotFound,
        }
    }
}

impl<'a> From<Option<&'a STMT>> for NodeQuery<'a> {
    fn from(o: Option<&'a STMT>) -> Self {
        match o {
            Some(s) => Self::Statement(s),
            None => Self::NotFound,
        }
    }
}

impl<'a> NodeQuery<'a> {
    pub fn map<T, FE: FnOnce(&'a EXPR) -> T, FS: FnOnce(&'a STMT) -> T>(
        self,
        map_expr: FE,
        map_stmt: FS,
    ) -> Option<T> {
        match self {
            Self::Statement(s) => Some(map_stmt(s)),
            Self::Expression(e) => Some(map_expr(e)),
            Self::NotFound => None,
        }
    }

    pub fn is_statement(&self) -> bool {
        if let Self::Statement(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_expression(&self) -> bool {
        if let Self::Expression(_) = self {
            true
        } else {
            false
        }
    }

    pub fn not_found(&self) -> bool {
        if let Self::NotFound = self {
            true
        } else {
            false
        }
    }

    pub fn statement(self) -> Option<&'a STMT> {
        match self {
            Self::Statement(s) => Some(s),
            _ => None,
        }
    }

    pub fn expression(self) -> Option<&'a EXPR> {
        match self {
            Self::Expression(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PrefixOp {
    Not,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EXPR {
    Cst(Range<usize>, u32),
    Var(Range<usize>, String),
    List(Range<usize>, Vec<EXPR>),
    ECall(Range<usize>, String, Vec<EXPR>),
    Prefix(Range<usize>, PrefixOp, Box<EXPR>),
    Infix(Range<usize>, Box<EXPR>, Op, Box<EXPR>),
}

impl EXPR {
    pub(crate) fn get_innermost(&self, p: usize) -> Option<&Self> {
        if self.get_range().contains(&p) {
            match self {
                Self::List(_, es) => get_innermost_exprs(es, p),
                Self::ECall(_, _, es) => get_innermost_exprs(es, p),
                Self::Prefix(_, _, e) => e.get_innermost(p),
                Self::Infix(_, e1, _, e2) => e1.get_innermost(p).or_else(|| e2.get_innermost(p)),
                x => Some(x),
            }
        } else {
            None
        }
    }
}

impl EXPR {
    pub fn get_range(&self) -> Range<usize> {
        match self {
            Self::Cst(r, ..) => r,
            Self::Var(r, ..) => r,
            Self::ECall(r, ..) => r,
            Self::List(r, ..) => r,
            Self::Prefix(r, ..) => r,
            Self::Infix(r, ..) => r,
        }
        .clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum STMT {
    Declr(Range<usize>, String, EXPR),
    Affect(Range<usize>, String, EXPR),
    If(Range<usize>, EXPR, Vec<STMT>, Option<Vec<STMT>>),
    While(Range<usize>, EXPR, Vec<STMT>),
    Defun(Range<usize>, String, Vec<String>, Vec<STMT>),
    Call(Range<usize>, String, Vec<EXPR>),
    Return(Range<usize>, EXPR),
}

pub fn get_innermost_stmts(stmts: &[STMT], p: usize) -> NodeQuery {
    if let Some(n) = stmts
        .iter()
        .map(|s| s.get_innermost(p))
        .filter(|q| !q.not_found())
        .next()
    {
        n
    } else {
        NodeQuery::NotFound
    }
}

pub fn get_innermost_exprs(exprs: &[EXPR], p: usize) -> Option<&EXPR> {
    exprs.iter().flat_map(|e| e.get_innermost(p)).next()
}

impl STMT {
    pub fn get_innermost(&self, p: usize) -> NodeQuery {
        if self.get_range().contains(&p) {
            match self {
                Self::Declr(_, _, e) => {
                    if e.get_range().contains(&p) {
                        e.get_innermost(p).into()
                    } else {
                        NodeQuery::Statement(self)
                    }
                }
                Self::Affect(_, _, e) => {
                    if e.get_range().contains(&p) {
                        e.get_innermost(p).into()
                    } else {
                        NodeQuery::Statement(self)
                    }
                }
                Self::If(_, c, iftrue, iffalse) => {
                    if c.get_range().contains(&p) {
                        c.get_innermost(p).into()
                    } else {
                        let n = get_innermost_stmts(iftrue, p);
                        if !n.not_found() {
                            n
                        } else {
                            iffalse
                                .as_ref()
                                .map(|s| get_innermost_stmts(s, p))
                                .unwrap_or(NodeQuery::Statement(self))
                        }
                    }
                }
                Self::While(_, c, b) => {
                    if c.get_range().contains(&p) {
                        c.get_innermost(p).into()
                    } else {
                        get_innermost_stmts(b, p)
                    }
                }
                Self::Defun(_, _, _, b) => get_innermost_stmts(b, p),
                Self::Call(_, _, es) => get_innermost_exprs(es, p).into(),
                Self::Return(_, e) => e.get_innermost(p).into(),
            }
        } else {
            NodeQuery::NotFound
        }
    }

    pub fn get_declarations(&self) -> Vec<(Range<usize>, Cow<str>)> {
        match self {
            Self::Declr(r, s, _) => vec![(r.clone(), Cow::Borrowed(s))],
            Self::Defun(_, _, _, b) => b
                .iter()
                .flat_map(|s| s.get_declarations().into_iter())
                .collect(),
            Self::If(_, _, iftrue, iffalse) => iftrue
                .iter()
                .flat_map(|s| s.get_declarations().into_iter())
                .chain(
                    iffalse
                        .iter()
                        .flatten()
                        .flat_map(|s| s.get_declarations().into_iter()),
                )
                .collect(),
            Self::While(_, _, b) => b
                .iter()
                .flat_map(|s| s.get_declarations().into_iter())
                .collect(),
            _ => vec![],
        }
    }
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
        }
        .clone()
    }
}

#[test]
fn test_main() {
    println!(
        "{:?}",
        leek_prog::prog("if (1) { print(a); } else { print(b); }")
    );
}
