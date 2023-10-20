use crate::types::{Node, Num, Parser, Token};

impl<'a> Parser<'a> {
    /// 構文木を作る処理
    pub fn parse(&mut self) -> Vec<Box<Node<'a>>> {
        self.program()
    }
    fn program(&mut self) -> Vec<Box<Node<'a>>> {
        let mut stmts: Vec<Box<Node>> = vec![];
        loop {
            let stmt = self.stmt();
            if let Some(node) = stmt {
                stmts.push(node);
            } else {
                break;
            }
        }
        stmts
    }
    fn stmt(&mut self) -> Option<Box<Node<'a>>> {
        let node;
        if self.token_iter.consume("return") {
            node = new_node(Token::Reserved("return"), self.expr(), None);
        } else {
            node = self.expr()
        }
        if !(self.token_iter.consume(";")) {
            return None;
        }
        node
    }
    fn assign(&mut self) -> Option<Box<Node<'a>>> {
        let mut node = self.equality();
        if self.token_iter.consume("=") {
            node = new_node(Token::Operand("="), node, self.assign())
        }
        node
    }

    fn primary(&mut self) -> Option<Box<Node<'a>>> {
        // 最初の数字をとっている想定
        let st = self.token_iter.next();
        if st.is_none() {
            return None;
        }
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
                Some(Token::LVar(var)) => {
                    self.vars.insert(var);
                    return new_node(Token::LVar(var), None, None);
                }
                _ => {
                    panic!("{:?}, {:?}", self, st);
                }
            }
        }
    }
    /// 式に相当する節
    fn expr(&mut self) -> Option<Box<Node<'a>>> {
        return self.assign();
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
    use std::collections::{HashMap, HashSet};

    use crate::types::{Parser, TokenIter, Variables};

    /*#[test]
    fn test_parser() {
        let mut iter = TokenIter { s: "1 < 2 + 3" };
        let mut vars = Variables {
            offsets: &mut HashMap::new()
        };
        let mut parser = Parser {
            token_iter: &mut iter,
            vars: &mut vars,
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
    }*/
    #[test]
    fn test_set() {
        let mut set: HashSet<i32> = HashSet::new();
        set.insert(1);
        set.insert(2);
        assert_eq!(set.len(), 2)
    }

    #[test]
    fn test_stmt() {
        let mut iter = TokenIter { s: "var = 1; var;" };
        let mut vars = Variables {
            offsets: &mut HashMap::new(),
        };
        let mut parser = Parser {
            token_iter: &mut iter,
            vars: &mut vars,
        };
        println!("{:?}", parser.parse());
        println!("{:?}", parser);
    }
}
