pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();
    let mut factor = 2;
    let mut num = n;
    while num > 1 {
        if num % factor == 0 {
            prime_factors.push(factor);
            num /= factor;
        } else {
            factor += 1;
        }
    }
    prime_factors
}
