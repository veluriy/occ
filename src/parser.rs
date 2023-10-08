use crate::lex::split_digit;
use crate::types::{Node, Num, Parser, Token};

impl Parser<'_> {
    /// 構文木を作る処理
    pub fn parse(&mut self) -> Option<Box<Node>> {
        self.expr()
    }
    fn primary(&mut self) -> Option<Box<Node>> {
        let st = self.token_iter.next();
        let node: Option<Box<Node>>;
        if *st.as_ref().unwrap() == Token::Bra {
            node = self.expr();
            let token = self.token_iter.next();
            if token != Some(Token::Ket) {
                panic!("）が期待されますが、見つかりませんでした。")
            }
            node
        } else {
            match st {
                Some(Token::Num(n)) => {
                    new_node_num(n)
                }
                _ => {
                    panic!("{:?}", self);
                }
            }
        }
    }
    /// 式に相当する節
    fn expr(&mut self) -> Option<Box<Node>> {
        let mut node = self.mul();
        loop {
            // !
            let token = self.next_readonly();
            if token.is_none() {
                return node;
            }
            if *token.as_ref().unwrap() == Token::Plus {
                self.token_iter.next();
                // println!("plus:{:?}", self);
                node = new_node(Token::Plus, node, self.mul());
            } else if token.unwrap() == Token::Minus {
                self.token_iter.next();
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
                    self.token_iter.next();
                    node = new_node(Token::Mul, node, self.primary())
                }
                Some(Token::Div) => {
                    self.token_iter.next();
                    node = new_node(Token::Div, node, self.primary())
                }
                _ => {
                    return node;
                }
            }
        }
    }
    /// 次のトークンを読み取るが、文字列の変更はしない
    pub fn next_readonly(&self) -> Option<Token> {
        if self.token_iter.s.is_empty() {
            return None;
        }
        match self.token_iter.s.as_bytes()[0] {
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

        let (digit_s, _remain_s) = split_digit(self.token_iter.s);
        if !digit_s.is_empty() {
            return Some(Token::Num(Num::from_str_radix(digit_s, 10).unwrap()));
        }
        panic!("Invalid token stream");
    }
}

/// 構文木を作るための補助的な関数
/// Some<Box<...>>でくるんで返す
fn new_node(kind: Token, lhs: Option<Box<Node>>, rhs: Option<Box<Node>>) -> Option<Box<Node>> {
    
    Some(Box::new(Node { kind, lhs, rhs }))
}

/// 数字に対応した節を作る
fn new_node_num(val: Num) -> Option<Box<Node>> {
    let node = Node {
        kind: Token::Num(val),
        lhs: None,
        rhs: None,
    };
    Some(Box::new(node))
}
