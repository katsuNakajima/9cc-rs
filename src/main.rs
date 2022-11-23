use std::{env, iter::Peekable};

fn strtol<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> u32 {
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

fn gen(args: Vec<String>) {
    let mut p = args[1].chars().peekable();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", strtol(&mut p));

    loop {
        let c = p.next();
        match c {
            Some(a) => match a {
                '+' => println!("  add rax, {}", strtol(&mut p)),
                '-' => println!("  sub rax, {}", strtol(&mut p)),
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
