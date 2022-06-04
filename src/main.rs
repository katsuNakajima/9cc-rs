use std::env;

fn gen(args: Vec<String>) {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", args[1]);
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
    fn print_gen() {
        let args = vec!["dummy".to_string(),"123".to_string()];
        gen(args);
    }
}
