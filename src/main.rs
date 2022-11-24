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
    #[allow(non_camel_case_types)]
    TK_RESERVED, // 記号
    #[allow(non_camel_case_types)]
    TK_NUM, // 整数トークン
    #[allow(non_camel_case_types)]
    TK_EOF, // 入力の終わりを表すトークン
}

/// トークン型
struct Token {
    kind: TokenKind, // トークンの型
    // next:Token,    // 次の入力トークン
    val: i32,  // kindがTK_NUMの場合、その数値
    str: char, // トークン文字列
}

/// 次のトークンが期待している記号のときには、トークンを1つ読み進めて真を返す。それ以外の場合には偽を返す。
fn consume<T: Iterator<Item = Token>>(op: char, token: &mut Peekable<T>) -> bool {
    let tk = token.peek().unwrap();
    if tk.kind != TokenKind::TK_RESERVED || tk.str != op {
        return false;
    }
    token.next();
    true
}

fn gen(args: Vec<String>) {
    let mut p = args[1].chars().peekable();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", str_to_u(&mut p));

    loop {
        let c = p.next();
        match c {
            Some(a) => match a {
                '+' => println!("  add rax, {}", str_to_u(&mut p)),
                '-' => println!("  sub rax, {}", str_to_u(&mut p)),
                _ => panic!("予期しない文字です: {}", a),
            },
            None => break,
        }
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
}
