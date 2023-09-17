pub enum NodeKind {
    ND_ADD,
    ND_SUB,
    ND_MUL,
    ND_DIV,
    ND_NUM,
}

struct Node {
    kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
    val: u8, // kindがND_NUMの時使う
}

fn new_node(
    kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
) -> Node {
    let node = Node {
        kind: kind,
        lhs: lhs,
        rhs: rhs,
        val: 0, // 使わない
    };
    return node;
}

fn new_node_num(val: u8) -> Node {
    let node = Node {
        kind: ND_NUM,
        lhs: None,
        rhs: None,
        val: val,
    };
    return node;
}

fn expr() -> Node {
    
}