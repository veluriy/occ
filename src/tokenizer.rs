// https://qiita.com/AtsukiTak/items/0819ee57af2639891ecf
fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
}

#[derive(Debug)]
pub enum Token {
    Plus,
    Minus,
    Num(usize),
}

/// トークンのイテレータを表す構造体
pub struct TokenIter<'a> {
    s: &'a str,
}

/// 文字列を受け取って、トークンのイテレータを返す関数。つまりトークナイザー。
/// Rustのイテレータは遅延評価なのでここでは何もしていない。
pub fn tokenize<'a>(s: &'a str) -> TokenIter<'a> {
    TokenIter { s }
}

impl Token {
    /// あとで使う便利関数。
    pub fn expect_num(&self) -> usize {
        match self {
            Token::Num(n) => *n,
            t => panic!("Expect number but found {:?}", t),
        }
    }
}

/// トークナイザーの中身。
/// やっていることは、次のトークンの判定を行い、内部の文字列を更新するだけ。
impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.s = self.s.trim_start();
        if self.s.is_empty() {
            return None;
        }

        if self.s.as_bytes()[0] == b'+' {
            self.s = self.s.split_at(1).1;
            return Some(Token::Plus);
        }

        if self.s.as_bytes()[0] == b'-' {
            self.s = self.s.split_at(1).1;
            return Some(Token::Minus);
        }

        let (digit_s, remain_s) = split_digit(self.s);
        if !digit_s.is_empty() {
            self.s = remain_s;
            return Some(Token::Num(usize::from_str_radix(digit_s, 10).unwrap()));
        }

        panic!("Invalid token stream")
    }
}