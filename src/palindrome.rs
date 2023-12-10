pub fn isPalindrome (p: i32) -> bool {
    let mut temp = p;
    let mut check = 0;

    while temp > 0 {
        check *= 10;
        check += temp % 10;
        temp /= 10;
    }

    p == check
}