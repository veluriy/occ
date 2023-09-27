/// 構文木
#[derive(Debug)]
pub struct Node {
    pub kind: Token,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}

/// 使う数値型
pub type Num = u8;

/// トークン
#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,  // "+"
    Minus, // "-"
    Mul,   // "*"
    Div,   // "/"
    Bra,   // "("
    Ket,   // ")"
    Num(Num),
}

/// トークンのイテレータを表す構造体
#[derive(Debug)]
pub struct TokenIter<'a> {
    pub s: &'a str,
}

/// 文字列を受け取って、トークンのイテレータを返す関数。つまりトークナイザー。
/// Rustのイテレータは遅延評価なのでここでは何もしていない。
pub fn tokenize<'a>(s: &'a str) -> TokenIter<'a> {
    TokenIter { s }
}

/// パーサー
/// イテレータを持つ
#[derive(Debug)]
pub struct Parser<'a> {
    pub token_iter: &'a mut TokenIter<'a>,
}
