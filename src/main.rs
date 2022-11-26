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
    val: i32,
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
    let val = tk.val;
    token.next();
    val
}

fn at_eof<T: Iterator<Item = Token>>(token: &mut Peekable<T>) -> bool {
    let tk = token.peek().unwrap();
    tk.kind == TokenKind::TK_EOF
}

/// 新しいトークンを作成してcurに繋げる
fn new_token(kind: TokenKind, str: String, cur: &mut Vec<Token>) {
    let val = if kind == TokenKind::TK_NUM {
        str.parse().unwrap()
    } else {
        0
    };
    let tok = Token {
        kind: kind,
        str: str,
        val: val,
    };
    cur.push(tok);
}

// 入力文字列pをトークナイズしてそれを返す
fn tokienize(str: String, cur: &mut Vec<Token>) {
    let mut str_no_space = str.clone();
    // 空白文字を削除
    str_no_space.retain(|c| !c.is_whitespace());
    let mut p = str_no_space.chars().peekable();
    loop {
        let next_p = p.peek();
        match next_p {
            Some(a) => {
                if a == &'+' || a == &'-' {
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

fn gen(args: Vec<String>) {
    let mut cur: Vec<Token> = Vec::new();
    let arg = args[1].clone();
    tokienize(arg, &mut cur);
    let mut token = cur.into_iter().peekable();

    // アセンブリの前半部分を出力
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    // 式の最初は数でなければならないので、それをチェックして
    // 最初のmov命令を出力
    println!("  mov rax, {}", expect_number(&mut token));

    while !at_eof(&mut token) {
        if consume('+', &mut token) {
            println!("  add rax, {}", expect_number(&mut token));
            continue;
        }
        expect('-', &mut token);
        println!("  sub rax, {}", expect_number(&mut token));
    }
    println!("  ret");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("引数の個数が正しくありません");
    }
    gen(args);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn print_step1() {
        let args = vec!["dummy".to_string(), "123".to_string()];
        gen(args);
    }

    #[test]
    fn print_step2() {
        let args = vec!["dummy".to_string(), "5+20-4".to_string()];
        gen(args);
    }

    #[test]
    fn print_step3() {
        let args = vec!["dummy".to_string(), " 12 + 34 - 5 ".to_string()];
        gen(args);
    }
}
