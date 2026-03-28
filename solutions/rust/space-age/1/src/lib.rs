// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR_SECONDS: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration{
    seconds: u64,
}


impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Self { seconds }
    }
}

pub trait Planet {
    fn orbital_period() -> f64;
    fn years_during(d: &Duration) -> f64 {
    d.seconds as f64 / (EARTH_YEAR_SECONDS * Self::orbital_period())
    }
}
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
#[macro_export]
macro_rules! planet {
    ($planet:ident, $period:expr) => {
        pub struct $planet;

        impl Planet for $planet {
            fn orbital_period() -> f64 {
                $period
            }
        }
    };
}
