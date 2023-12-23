// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_DAYS: f64 = 365.25;
const DAY_SECS: u64 = 24 * 60 * 60;

#[derive(Debug)]
pub struct Duration {
    secs: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { secs: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! impl_planet_for {
    ( $(($t:ty, $e:expr)),+ ) => {
        $(
            impl Planet for $t {
                fn years_during(d: &Duration) -> f64 {
                    d.secs as f64 / ($e * EARTH_DAYS * DAY_SECS as f64)
                }
            }
        )+
    }
}

impl_planet_for!(
    (Mercury, 0.2408467), 
    (Earth, 1.0),
    (Venus, 0.61519726),
    (Mars, 1.8808158),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132)
);