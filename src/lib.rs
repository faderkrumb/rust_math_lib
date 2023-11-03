// use::std::collections::HashMap;
pub mod test;
pub use test::*;

pub mod fib;
pub use fib::*;

pub mod e_sieve;
pub use e_sieve::*;

/*
pub struct FibMap {
    map: HashMap<i64, i64>,
}

impl FibMap {

    pub fn new() -> FibMap{
        let mut map = HashMap::new();
        map.insert(0, 0);
        map.insert(1, 1);

        FibMap {
            map: map.clone(),
        }
    }

    pub fn get_fib(&mut self, n: i64) -> i64 {
        if self.map.contains_key(&n) {
            return *self.map.get(&n).unwrap();
        }
        let mut fib_n = self.get_fib(n - 1);
        fib_n += self.get_fib(n - 2);
        self.map.insert(n, fib_n);
        return *self.map.get(&n).unwrap();
        }
}*/