pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num
    .to_string()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect::<Vec<_>>();

    let mut sum: u32 = 0;
    for digit in &digits {
        sum = match sum.checked_add(digit.pow(digits.len() as u32)) {
            Some(v) => v,
            None => return false,
        }
    }
    
    sum == num
}
