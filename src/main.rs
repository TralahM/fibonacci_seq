use std::env;
mod another;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    let terms=args[1].parse::<i64>().unwrap();
    another::fibonacci(terms);
}
