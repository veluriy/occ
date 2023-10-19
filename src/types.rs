use std::collections::HashMap;

/// 構文木
#[derive(Debug, PartialEq, Clone)]
pub struct Node<'a> {
    pub kind: Token<'a>,
    pub lhs: Option<Box<Node<'a>>>,
    pub rhs: Option<Box<Node<'a>>>,
}

/// 使う数値型
pub type Num = u8;

/// トークン
#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Operand(&'a str), // Plusなどの演算子をまとめる予定
    Num(Num),
    /// 変数
    LVar(&'a str),
    /// 予約語に対応する
    Reserved(&'a str),
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

/// パーサー。イテレータを持つ
#[derive(Debug)]
pub struct Parser<'a> {
    pub token_iter: &'a mut TokenIter<'a>,
    /// 検出した変数名とそれに対応するオフセットを格納する。
    pub vars: &'a mut Variables<'a>,
}

#[derive(Debug)]
pub struct Variables<'a> {
    pub offsets: &'a mut HashMap<&'a str, usize>,
}

impl<'a> Variables<'a> {
    pub fn insert(&mut self, str: &'a str) -> Option<usize> {
        if !self.offsets.contains_key(str) {
            self.offsets.insert(str, self.offsets.len() * 8)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::Variables;
    use std::collections::HashMap;

    #[test]
    fn test_var() {
        let mut vars = Variables {
            offsets: &mut HashMap::new(),
        };
        vars.insert("var1");
        vars.insert("var2");
        vars.insert("var1");
        println!("{:?}", vars);
    }
}
