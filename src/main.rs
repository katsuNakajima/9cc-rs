use std::{char, env, iter::Peekable};

fn str_to_u<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> u32 {
    let mut res = 0;
    while let Some(i) = iter.peek() {
        match i.to_digit(10) {
            Some(n) => res = res * 10 + n,
            None => break,
        }
        iter.next();
    }
    res
}

/// トークンの種類
#[derive(PartialEq)]
enum TokenKind {
    /// 記号
    #[allow(non_camel_case_types)]
    TK_RESERVED,
    /// 整数トークン
    #[allow(non_camel_case_types)]
    TK_NUM,
    // 入力の終わりを表すトークン
    #[allow(non_camel_case_types)]
    TK_EOF,
}

/// トークン型
struct Token {
    /// トークンの型
    kind: TokenKind,
    /// kindがTK_NUMの場合、その数値
    val: Option<i32>,
    // トークン文字列
    str: String,
}

/// 次のトークンが期待している記号のときには、トークンを1つ読み進めて真を返す。
/// それ以外の場合には偽を返す。
fn consume<T: Iterator<Item = Token>>(op: char, token: &mut Peekable<T>) -> bool {
    let tk = token.peek().unwrap();
    if tk.kind != TokenKind::TK_RESERVED || tk.str != op.to_string() {
        return false;
    }
    token.next();
    true
}

/// 次のトークンが期待している記号のときには、トークンを1つ読み進める。
/// それ以外の場合にはエラーを報告する。
fn expect<T: Iterator<Item = Token>>(op: char, token: &mut Peekable<T>) {
    let tk = token.peek().unwrap();
    if tk.kind != TokenKind::TK_RESERVED || tk.str != op.to_string() {
        panic!("{}ではありません", op);
    }
    token.next();
}

/// 次のトークンが数値の場合、トークンを1つ読み進めてその数値を返す。
/// それ以外の場合にはエラーを報告する。
fn expect_number<T: Iterator<Item = Token>>(token: &mut Peekable<T>) -> i32 {
    let tk = token.peek().unwrap();
    if tk.kind != TokenKind::TK_NUM {
        panic!("数ではありません");
    }
    let val = tk.val.unwrap();
    token.next();
    val
}

#[allow(dead_code)]
fn at_eof<T: Iterator<Item = Token>>(token: &mut Peekable<T>) -> bool {
    let tk = token.peek().unwrap();
    tk.kind == TokenKind::TK_EOF
}

/// 新しいトークンを作成してcurに繋げる
fn new_token(kind: TokenKind, str: String, cur: &mut Vec<Token>) {
    let val = if kind == TokenKind::TK_NUM {
        Some(str.parse().unwrap())
    } else {
        None
    };
    let tok = Token {
        kind: kind,
        str: str,
        val: val,
    };
    cur.push(tok);
}

/// 入力文字列pをトークナイズしてそれを返す
fn tokienize(str: String, cur: &mut Vec<Token>) {
    let mut str_no_space = str.clone();
    // 空白文字を削除
    str_no_space.retain(|c| !c.is_whitespace());
    let mut p = str_no_space.chars().peekable();
    loop {
        let next_p = p.peek();
        match next_p {
            Some(a) => {
                if a == &'+' || a == &'-' || a == &'*' || a == &'/' || a == &'(' || a == &')' {
                    new_token(TokenKind::TK_RESERVED, a.to_string(), cur);
                } else if a.is_digit(10) {
                    new_token(TokenKind::TK_NUM, str_to_u(&mut p).to_string(), cur);
                    continue;
                } else {
                    panic!("トークナイズできません");
                }
            }
            None => break,
        }
        p.next();
    }
    new_token(TokenKind::TK_EOF, str, cur);
}

/// 抽象構文木のノードの種類
#[derive(PartialEq)]
enum NodeKind {
    /// +
    #[allow(non_camel_case_types)]
    ND_ADD,
    /// -
    #[allow(non_camel_case_types)]
    ND_SUB,
    /// *
    #[allow(non_camel_case_types)]
    ND_MUL,
    /// /
    #[allow(non_camel_case_types)]
    ND_DIV,
    /// 整数
    #[allow(non_camel_case_types)]
    ND_NUM,
}

/// 抽象構文木のノードの型
struct Node {
    /// ノードの型
    kind: NodeKind,
    /// 左辺
    lhs: Option<Box<Node>>,
    /// 右辺
    rhs: Option<Box<Node>>,
    /// kindがND_NUMの場合のみ使う
    val: Option<i32>,
}

fn new_node(kind: NodeKind, lhs: Node, rhs: Node) -> Node {
    let node = Node {
        kind: kind,
        lhs: Some(Box::new(lhs)),
        rhs: Some(Box::new(rhs)),
        val: None,
    };
    node
}

fn new_node_num(val: i32) -> Node {
    let node = Node {
        kind: NodeKind::ND_NUM,
        lhs: None,
        rhs: None,
        val: Some(val),
    };
    node
}

fn expr<T: Iterator<Item = Token>>(token: &mut Peekable<T>) -> Node {
    let mut node: Node = mul(token);

    loop {
        if consume('+', token) {
            node = new_node(NodeKind::ND_ADD, node, mul(token));
        } else if consume('-', token) {
            node = new_node(NodeKind::ND_SUB, node, mul(token));
        } else {
            return node;
        }
    }
}

fn mul<T: Iterator<Item = Token>>(token: &mut Peekable<T>) -> Node {
    let mut node = unary(token);

    loop {
        if consume('*', token) {
            node = new_node(NodeKind::ND_MUL, node, unary(token));
        } else if consume('/', token) {
            node = new_node(NodeKind::ND_DIV, node, unary(token));
        } else {
            return node;
        }
    }
}

fn unary<T: Iterator<Item = Token>>(token: &mut Peekable<T>) -> Node {
    if consume('+', token) {
        return unary(token);
    }
    if consume('-', token) {
        return new_node(NodeKind::ND_SUB, new_node_num(0), unary(token));
    }
    primary(token)
}

fn primary<T: Iterator<Item = Token>>(token: &mut Peekable<T>) -> Node {
    // 次のトークンが"("なら、"(" expr ")"のはず
    if consume('(', token) {
        let node = expr(token);
        expect(')', token);
        return node;
    }
    // そうでなければ数値のはず
    new_node_num(expect_number(token))
}

fn gen(node: Node) {
    if node.kind == NodeKind::ND_NUM {
        println!("  push {}", node.val.unwrap());
        return;
    }

    gen(*node.lhs.unwrap());
    gen(*node.rhs.unwrap());

    println!("  pop rdi");
    println!("  pop rax");

    match node.kind {
        NodeKind::ND_ADD => println!("  add rax, rdi"),
        NodeKind::ND_SUB => println!("  sub rax, rdi"),
        NodeKind::ND_MUL => println!("  imul rax, rdi"),
        NodeKind::ND_DIV => println!("  cqo\n  idiv rdi"),
        NodeKind::ND_NUM => (),
    }

    println!("  push rax");
}

fn main_sub(args: Vec<String>) {
    let mut cur: Vec<Token> = Vec::new();
    let arg = args[1].clone();
    tokienize(arg, &mut cur);
    let mut token = cur.into_iter().peekable();
    let node = expr(&mut token);

    // アセンブリの前半部分を出力
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    // 抽象構文木を下りながらコード生成
    gen(node);

    // スタックトップに式全体の値が残っているはずなので
    // それをRAXにロードして関数からの返り値とする
    println!("  pop rax");
    println!("  ret");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("引数の個数が正しくありません");
    }
    main_sub(args);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn print_step1() {
        let args = vec!["dummy".to_string(), "123".to_string()];
        main_sub(args);
    }

    #[test]
    fn print_step2() {
        let args = vec!["dummy".to_string(), "5+20-4".to_string()];
        main_sub(args);
    }

    #[test]
    fn print_step3() {
        let args = vec!["dummy".to_string(), " 12 + 34 - 5 ".to_string()];
        main_sub(args);
    }

    #[test]
    fn print_step5_1() {
        let args = vec!["dummy".to_string(), "5*(9-6)".to_string()];
        main_sub(args);
    }

    #[test]
    fn print_step5_2() {
        let args = vec!["dummy".to_string(), "(3+5)/2".to_string()];
        main_sub(args);
    }

    #[test]
    fn print_step6_1() {
        let args = vec!["dummy".to_string(), "-10+20".to_string()];
        main_sub(args);
    }

    #[test]
    fn print_step6_2() {
        let args = vec!["dummy".to_string(), "- -10".to_string()];
        main_sub(args);
    }
}
