
use crate::types::{Node, Num, Parser, Token};

impl<'a> Parser<'a> {
    /// 構文木を作る処理
    pub fn parse(&mut self) -> Option<Box<Node>> {
        self.expr()
    }
    fn primary(&mut self) -> Option<Box<Node<'a>>> {
        // 最初の数字をとっている想定
        let st = self.token_iter.next();
        let node: Option<Box<Node>>;
        // println!("{:?}",st);
        if *st.as_ref().unwrap() == Token::Operand("(") {
            node = self.expr();
            let token = self.token_iter.next();
            if token != Some(Token::Operand(")")) {
                panic!("')' expected, but found {:?}, self: {:?}", token, self)
            }
            node
        } else {
            match st {
                Some(Token::Num(n)) => new_node_num(n),
                _ => {
                    panic!("{:?}, {:?}", self, st);
                }
            }
        }
    }
    /// 式に相当する節
    fn expr(&mut self) -> Option<Box<Node<'a>>> {
        return self.equality();
    }

    fn equality(&mut self) -> Option<Box<Node<'a>>> {
        let node = self.relational();
        if self.token_iter.consume("==") {
            return new_node(Token::Operand("=="), node, self.relational());
        } else if self.token_iter.consume("!=") {
            return new_node(Token::Operand("!="), node, self.relational());
        } else {
            return node;
        }
    }

    fn add(&mut self) -> Option<Box<Node<'a>>> {
        let mut node = self.mul();
        'outer: loop {
            let operands = vec!["+", "-"].into_iter();
            for op in operands {
                if self.token_iter.consume(op) {
                    node = new_node(Token::Operand(op), node, self.mul());
                    continue 'outer;
                }
            }
            return node;
        }
    }

    fn relational(&mut self) -> Option<Box<Node<'a>>> {
        let mut node = self.add();
        'outer: loop {
            // そのまま構文木に入れる
            if self.token_iter.consume("<=") {
                node = new_node(Token::Operand("<="), node, self.add());
                continue 'outer;
            } else if self.token_iter.consume("<") {
                node = new_node(Token::Operand("<"), node, self.add());
                continue 'outer;
            }
            // 左右を反転させて対応した演算を指定し、構文木に入れる（例: "3>4"-> "4<3"）
            else if self.token_iter.consume("=>") {
                node = new_node(Token::Operand("<="), self.add(), node);
                continue 'outer;
            } else if self.token_iter.consume(">") {
                node = new_node(Token::Operand("<"), self.add(), node);
                continue 'outer;
            }
            return node;
        }
    }
    /// 乗法、除法に対応する節
    fn mul(&mut self) -> Option<Box<Node<'a>>> {
        // primary { * primary}
        let mut node = self.unary();
        loop {
            if self.token_iter.consume("*") {
                node = new_node(Token::Operand("*"), node, self.unary());
            } else if self.token_iter.consume("/") {
                node = new_node(Token::Operand("/"), node, self.unary());
            } else {
                return node;
            };
        }
    }

    fn unary(&mut self) -> Option<Box<Node<'a>>> {
        if self.token_iter.consume("+") {
            return self.primary();
        }
        if self.token_iter.consume("-") {
            return new_node(Token::Operand("-"), new_node_num(0), self.primary());
        }
        self.primary()
    }

    // 次のトークンを読み取るが、文字列の変更はしない
   /* pub fn next_reaonly(&self) -> Option<Token> {
        if self.token_iter.s.is_empty() {
            return None;
        }

        let first_idx_not_whitespace = self.token_iter.s.chars().position(|c| !c.is_alphabetic() && c != '_');
        if let Some(idx) = first_idx_not_whitespace {

        }

        // > と =>のような部分列の関係にある文字列に注意
        let operands = vec!["+", "-", "*", "/", "(", ")", "<=", "=>", ">", "<", "=="];
        // operands.sort_by_key(f)
        for op in operands {
            if self.token_iter.s.starts_with(op) {
                self.token_iter.consume(op);
                return Some(Token::Operand(op));
            }
        }
        // 'return' や 'var'などの文字列の場合の処理
        if let Some(char) = self.token_iter.s.chars().nth(0) {
            if char.is_alphabetic() {
                // 変数名としてはアルファベットか_のみを許容
                let first_non_alphabetic_idx = self.token_iter.s.chars().position(|c| !c.is_alphabetic() && c != '_');
                // アルファベットではない文字が文字列中にあるので、その前までをresに格納->consume
                if let Some(idx) = first_non_alphabetic_idx {
                    let res = &self.token_iter.s[..idx];
                    self.token_iter.consume(res);
                    if is_reserved_words(&res) {
                        // 
                        return Some(Token::Reserved(res));
                    }
                    return Some(Token::LVar(res));
                }
                // 文字列のすべてがアルファベットなので、すべてを消化する。
                else {
                    let res = self.token_iter.s;
                    self.token_iter.consume(res);
                    if is_reserved_words(&res) {
                        return Some(Token::Reserved(res));
                    }
                    return Some(Token::LVar(res));
                }
            }
        } else {
            // self.s == ""
            return None;
        }
        let (digit_s, _remain_s) = split_digit(self.token_iter.s);
        if !digit_s.is_empty() {
            return Some(Token::Num(Num::from_str_radix(digit_s, 10).unwrap()));
        }
        panic!("Invalid token stream");
    }*/
}

/// 構文木を作るための補助的な関数
/// Some<Box<...>>でくるんで返す
fn new_node<'a>(
    kind: Token<'a>,
    lhs: Option<Box<Node<'a>>>,
    rhs: Option<Box<Node<'a>>>,
) -> Option<Box<Node<'a>>> {
    Some(Box::new(Node { kind, lhs, rhs }))
}

/// 数字に対応した節を作る
fn new_node_num<'a>(val: Num) -> Option<Box<Node<'a>>> {
    let node = Node {
        kind: Token::Num(val),
        lhs: None,
        rhs: None,
    };
    Some(Box::new(node))
}

#[cfg(test)]
mod test {
    use crate::{
        parser::new_node_num,
        types::{Node, Parser, Token, TokenIter},
    };

    #[test]
    fn test_parser() {
        let mut iter = TokenIter { s: "1 < 2 + 3" };
        let mut parser = Parser {
            token_iter: &mut iter,
        };

        assert_eq!(
            *(parser.parse().unwrap()),
            Node {
                kind: Token::Operand("<"),
                lhs: new_node_num(1),
                rhs: Some(Box::new(Node {
                    kind: Token::Operand("+"),
                    lhs: new_node_num(2),
                    rhs: new_node_num(3)
                }))
            }
        )
    }
}
