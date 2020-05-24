// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_SECONDS_YEAR: u32 = 31_557_600;

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (EARTH_SECONDS_YEAR as f64 * Self::ORBITAL_PERIOD)
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

impl Planet for Mercury {
    const ORBITAL_PERIOD: f64 = 0.240_846_7;
}
impl Planet for Venus {
    const ORBITAL_PERIOD: f64 = 0.615_197_26;
}
impl Planet for Earth {
    const ORBITAL_PERIOD: f64 = 1.0;
}
impl Planet for Mars {
    const ORBITAL_PERIOD: f64 = 1.880_815_8;
}
impl Planet for Jupiter {
    const ORBITAL_PERIOD: f64 = 11.862_615;
}
impl Planet for Saturn {
    const ORBITAL_PERIOD: f64 = 29.447_498;
}
impl Planet for Uranus {
    const ORBITAL_PERIOD: f64 = 84.016_846;
}
impl Planet for Neptune {
    const ORBITAL_PERIOD: f64 = 164.79132;
}
