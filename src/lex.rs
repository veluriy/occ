use crate::types::{Num, Token, TokenIter};

/// Cのstrtolと似た処理
pub fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
}

pub fn is_ctrl_sytx(s: &str) -> bool {
    if s == "return" {return true;}
    return false;
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
        self.s
            .starts_with(s)
            .then(|| {
                self.s = &self.s[s.len()..];
            })
            .is_some()
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
        while self.consume(" ") {}


        // > と =>のような部分列の関係にある文字列に注意
        let operands = vec!["+", "-", "*", "/", "(", ")", "<=", "=>", ">", "<", "=="];
        // operands.sort_by_key(f)
        for op in operands {
            if self.s.starts_with(op) {
                self.consume(op);
                return Some(Token::Operand(op));
            }
        }
        // 'return' や 'var'などの文字列の場合の処理
        if let Some(char) = self.s.chars().nth(0) {
            if char.is_alphabetic() {
                // 変数名としてはアルファベットか_のみを許容
                let first_non_alphabetic_idx = self.s.chars().position(|c| !c.is_alphabetic() && c != '_');
                // アルファベットではない文字が文字列中にあるので、その前までをresに格納->consume
                if let Some(idx) = first_non_alphabetic_idx {
                    let res = &self.s[..idx];
                    self.consume(res);
                    if is_ctrl_sytx(&res) {
                        // 
                        return Some(Token::Ctrl(res));
                    }
                    return Some(Token::LVar(res));
                }
                // 文字列のすべてがアルファベットなので、すべてを消化する。
                else {
                    let res = self.s;
                    self.consume(res);
                    if is_ctrl_sytx(&res) {
                        return Some(Token::Ctrl(res));
                    }
                    return Some(Token::LVar(res));
                }
            }
        } else {
            // self.s == ""
            return None;
        }
        // self.s.chars().position(|c| !c.is_alphabetic() && c != '_');

        let (digit_s, remain_s) = split_digit(self.s);
        if !digit_s.is_empty() {
            self.s = remain_s;
            return Some(Token::Num(Num::from_str_radix(digit_s, 10).unwrap()));
        }
        panic!("");
    }
}

#[cfg(test)]
mod test {
    use crate::types::TokenIter;

    #[test]
    fn test_return() {
        let mut iter = TokenIter { s: "return returns" };
        println!("{:?}",iter.next());
        assert_eq!(" returns", iter.s);
        println!("{:?}",iter.next());
        assert_eq!("", iter.s);
    }
    #[test]
    fn test_expr() {
        let mut iter = TokenIter { s: "1 + 2 * 3" };
        println!("{:?}",iter.next());
        assert_eq!(" + 2 * 3", iter.s);
        println!("{:?}",iter.next());
        assert_eq!(" 2 * 3", iter.s);
    }
}
