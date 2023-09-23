// https://qiita.com/AtsukiTak/items/0819ee57af2639891ecf
fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
}

// 使う数値型
type Num = u8;

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

/// 構文木
#[derive(Debug)]
pub struct Node {
    kind: Token,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

/// Some<Box<...>>でくるんで返す
fn new_node(kind: Token, lhs: Option<Box<Node>>, rhs: Option<Box<Node>>) -> Option<Box<Node>> {
    let node = Some(Box::new(Node {
        kind: kind,
        lhs: lhs,
        rhs: rhs,
    }));
    return node;
}

/// 数字の節
fn new_node_num(val: Num) -> Option<Box<Node>> {
    let node = Node {
        kind: Token::Num(val),
        lhs: None,
        rhs: None,
    };
    return Some(Box::new(node));
}

/// トークンのイテレータを表す構造体
#[derive(Debug)]
pub struct TokenIter<'a> {
    s: &'a str,
}

impl TokenIter<'_> {
    /// primaryに対応する構文木を返す
    fn primary(&mut self) -> Option<Box<Node>> {
        let st = self.next();
        let node: Option<Box<Node>>;
        if (*st.as_ref().unwrap() == Token::Bra) {
            node = self.expr();
            let token = self.next();
            println!("{:?}", self);
            if (token != Some(Token::Ket)) {
                panic!("）が期待されますが、見つかりませんでした。")
            }
            return node;
        } else {
            match st {
                Some(Token::Num(n)) => {
                    println!("som: {:?}", self);

                    return new_node_num(n);
                }
                _ => {
                    panic!("{:?}", self);
                }
            }
        }
    }
    /// 式に相当する節
    pub fn expr(&mut self) -> Option<Box<Node>> {
        let mut node = self.mul();
        loop {
            let token = self.next_readonly();
            if token == None {
                return node;
            }
            if (*token.as_ref().unwrap() == Token::Plus) {
                self.next();
                // println!("plus:{:?}", self);
                node = new_node(Token::Plus, node, self.mul());
            } else if (token.unwrap() == Token::Minus) {
                self.next();
                // println!("minus:{:?}", self);
                node = new_node(Token::Minus, node, self.mul());
            } else {
                return node;
            }
        }
    }
    /// 乗法、除法に対応する節
    fn mul(&mut self) -> Option<Box<Node>> {
        // primary { * primary}
        let mut node = self.primary();
        loop {
            let next = self.next_readonly();
            match next {
                Some(Token::Mul) => {
                    self.next();
                    node = new_node(Token::Mul, node, self.primary())
                }
                Some(Token::Div) => {
                    self.next();
                    node = new_node(Token::Div, node, self.primary())
                }
                _ => {
                    return node;
                }
            }
        }
    }
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

impl TokenIter<'_> {
    /// 次のトークンを読み取るが、文字列の変更はしない
    fn next_readonly(&self) -> Option<Token> {
        if self.s.is_empty() {
            return None;
        }
        println!("ddsa: {:?}", self.s);

        match self.s.as_bytes()[0] {
            b'+' => {
                return Some(Token::Plus);
            }
            b'-' => {
                return Some(Token::Minus);
            }
            b'*' => {
                return Some(Token::Mul);
            }
            b'/' => {
                return Some(Token::Div);
            }
            b'(' => {
                return Some(Token::Bra);
            }
            b')' => {
                return Some(Token::Ket);
            }
            _ => {}
        }

        let (digit_s, remain_s) = split_digit(self.s);
        if !digit_s.is_empty() {
            return Some(Token::Num(Num::from_str_radix(digit_s, 10).unwrap()));
        }

        panic!("Invalid token stream");
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
