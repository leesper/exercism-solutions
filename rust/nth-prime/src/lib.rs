pub fn nth(n: u32) -> u32 {
    // let mut num = 2;
    // let mut index = n;
    // while index > 0 {
    //     num += 1;
    //     if is_prime(num) {
    //         index -= 1;
    //     }
    // }
    // num
    (2..).filter(|x| is_prime(*x)).nth(n as usize).unwrap()
}

fn is_prime(num: u32) -> bool {
    // for i in 2..=(num as f32).sqrt() as u32 {
    //     if num % i == 0 {
    //         return false;
    //     }
    // }
    // true
    !(2..(num as f32).sqrt() as u32 + 1).any(|x| num % x == 0)
}
