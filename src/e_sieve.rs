pub struct ESieve {
    arr: [bool; 1e6 as usize],
}

impl ESieve {

    pub fn isPrime(&self, p: usize) -> bool {
        self.arr[p]
    }

    pub fn primeList(&self) -> Vec<i32> {
        let mut p: Vec<i32> = Vec::new();

        for i in 2..self.arr.len() {
            if self.isPrime(i) {
                p.push(i as i32);
            }
        }

        p
    }

    pub fn new() -> ESieve {
        let mut temp: [bool; 1e6 as usize] = [true; 1e6 as usize];
        temp[0] = false;
        temp[1] = false;
    
        for i in 2..temp.len() {
            if temp[i] == true {
                for j in 2..(temp.len() / i) {
                    temp[i * j] = false;
                }
            }
        }

        ESieve {
            arr: temp,
        }
    }

}