/// 構文木
#[derive(Debug, PartialEq)]
pub struct Node<'a> {
  pub kind: Token<'a>,
  pub lhs: Option<Box<Node<'a>>>,
  pub rhs: Option<Box<Node<'a>>>,
}

/// 使う数値型
pub type Num = u8;

/// トークン
#[derive(Debug, PartialEq)]
pub enum Token<'a> {
  Operand(&'a str), // Plusなどの演算子をまとめる予定
  Num(Num),
  LVar(&'a str),
}

/// トークンのイテレータを表す構造体
#[derive(Debug)]
pub struct TokenIter<'a> {
  pub s: &'a str,
}

/// 文字列を受け取って、トークンのイテレータを返す関数。つまりトークナイザー。
/// Rustのイテレータは遅延評価なのでここでは何もしていない。
pub fn tokenize(s: &str) -> TokenIter<'_> {
  TokenIter { s }
}

/// パーサー
/// イテレータを持つ
#[derive(Debug)]
pub struct Parser<'a> {
  pub token_iter: &'a mut TokenIter<'a>,
}
