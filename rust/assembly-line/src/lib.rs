
const NR_CARS_PRODUCED_PER_HOUR: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let prod = speed as f64 * NR_CARS_PRODUCED_PER_HOUR;
    match speed {
        0 => 0.0,
        1..=4 => return prod,
        5..=8 => return prod * 0.9,
        9..=10 => return prod * 0.77,
        _ => panic!("Invalid speed"),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return (production_rate_per_hour(speed) / 60.0) as u32;
}
