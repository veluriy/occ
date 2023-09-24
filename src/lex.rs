pub fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
}

/// 使う数値型
pub type Num = u8;

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

impl Token {
    /// あとで使う便利関数。
    pub fn expect_num(&self) -> Num {
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

        match self.s.as_bytes()[0] {
            b'+' => {
                self.s = self.s.split_at(1).1;
                return Some(Token::Plus);
            }
            b'-' => {
                self.s = self.s.split_at(1).1;
                return Some(Token::Minus);
            }
            b'*' => {
                self.s = self.s.split_at(1).1;
                return Some(Token::Mul);
            }
            b'/' => {
                self.s = self.s.split_at(1).1;
                return Some(Token::Div);
            }
            b'(' => {
                self.s = self.s.split_at(1).1;
                return Some(Token::Bra);
            }
            b')' => {
                self.s = self.s.split_at(1).1;
                return Some(Token::Ket);
            }
            _ => {}
        }

        let (digit_s, remain_s) = split_digit(self.s);
        if !digit_s.is_empty() {
            self.s = remain_s;
            return Some(Token::Num(Num::from_str_radix(digit_s, 10).unwrap()));
        }
        eprintln!("s:{:?}", remain_s);
        panic!("");
    }
}
