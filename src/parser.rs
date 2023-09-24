use crate::lex::{Token, TokenIter, split_digit, Num};
/// 構文木
#[derive(Debug)]
pub struct Node {
    kind: Token,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

#[derive(Debug)]

pub struct Parser<'a> {
    pub tokenizer: &'a mut TokenIter<'a>,
}
// コンストラクタ -> gen

impl Parser<'_> {
    pub fn parse(&mut self) {
        let mut node = self.expr();
        if let Some(b) = node {
            generate_assembly_by_node(&b);
        }
    }
    fn primary(&mut self) -> Option<Box<Node>> {
        let st = self.tokenizer.next();
        let node: Option<Box<Node>>;
        if (*st.as_ref().unwrap() == Token::Bra) {
            node = self.expr();
            let token = self.tokenizer.next();
            if (token != Some(Token::Ket)) {
                panic!("）が期待されますが、見つかりませんでした。")
            }
            return node;
        } else {
            match st {
                Some(Token::Num(n)) => {
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
            // !
            let token = self.next_readonly();
            if token == None {
                return node;
            }
            if (*token.as_ref().unwrap() == Token::Plus) {
                self.tokenizer.next();
                // println!("plus:{:?}", self);
                node = new_node(Token::Plus, node, self.mul());
            } else if (token.unwrap() == Token::Minus) {
                self.tokenizer.next();
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
                    self.tokenizer.next();
                    node = new_node(Token::Mul, node, self.primary())
                }
                Some(Token::Div) => {
                    self.tokenizer.next();
                    node = new_node(Token::Div, node, self.primary())
                }
                _ => {
                    return node;
                }
            }
        }
    }
    /// 次のトークンを読み取るが、文字列の変更はしない
    fn next_readonly(&self) -> Option<Token> {
        if self.tokenizer.s.is_empty() {
            return None;
        }
        match self.tokenizer.s.as_bytes()[0] {
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

        let (digit_s, remain_s) = split_digit(self.tokenizer.s);
        if !digit_s.is_empty() {
            return Some(Token::Num(Num::from_str_radix(digit_s, 10).unwrap()));
        }
        panic!("Invalid token stream");
    }
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

/// 数字に対応した節
fn new_node_num(val: Num) -> Option<Box<Node>> {
    let node = Node {
        kind: Token::Num(val),
        lhs: None,
        rhs: None,
    };
    return Some(Box::new(node));
}

/// 構文木からアセンブリコードを再帰的に作る
pub fn generate_assembly_by_node(node: &Node) {
    if let Token::Num(n) = node.kind {
        println!("  push {:?}", n);
        return;
    }
    if let Some(b) = &node.lhs {
        generate_assembly_by_node(&b);
    }
    if let Some(b) = &node.rhs {
        generate_assembly_by_node(&b);
    }
    println!("  pop rdi");
    println!("  pop rax");
    match node.kind {
        Token::Plus => println!("  add rax, rdi"),
        Token::Minus => println!("  sub rax, rdi"),
        Token::Mul => println!("  imul rax, rdi"),
        Token::Div => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        _ => {}
    };
    println!("  push rax");
}