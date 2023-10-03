mod fib;
use fib::*;

fn main() {
    let mut fib_map = FibMap::new();
    println!("{}", fib_map.get_fib(6));
}