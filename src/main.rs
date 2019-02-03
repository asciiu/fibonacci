use std::io;

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

        println!("entered: {}", num);
    }
}