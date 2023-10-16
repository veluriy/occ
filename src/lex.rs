use crate::types::{Num, Token, TokenIter};

/// Cのstrtolと似た処理
pub fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
}

impl Token<'_> {
    /// あとで使う便利関数。
    pub fn expect_num(&self) -> Num {
        match self {
            Token::Num(n) => *n,
            t => panic!("Expect number but found {:?}", t),
        }
    }
}

impl TokenIter<'_> {
    pub fn consume(&mut self, s: &str) -> bool {
        self.s.starts_with(s).then(|| {
            self.s = &self.s[s.len()..];
        }).is_some()
    }
}

/// トークナイザーの中身。
/// やっていることは、次のトークンの判定を行い、内部の文字列を更新するだけ。
impl<'a> Iterator for TokenIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.s = self.s.trim_start();
        if self.s.is_empty() {
            return None;
        }

        // > と =>のような部分列の関係にある文字列に注意
        let operands = vec!["+", "-", "*", "/", "(", ")", "<=", "=>", ">", "<", "=="];
        // operands.sort_by_key(f)
        for op in operands {
        if self.s.starts_with(op){
            self.consume(op);
            return Some(Token::Operand(op));
        }
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

#[cfg(test)]
mod test {
    use crate::types::TokenIter;

    #[test]
    fn test() {
        let mut iter = TokenIter{s: "3+4"};
        iter.next();
        assert_eq!("+4", iter.s);
        iter.next();
        assert_eq!("4", iter.s);
    }
}