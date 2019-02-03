use std::io;

fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-1) + fib(n-2)
    }
}
fn main() {
    loop {
        println!("Enter fib num:");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let num: i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        let fibn: i32 = fib(num);

        println!("fib({}) = {}", num, fibn);
    }
}