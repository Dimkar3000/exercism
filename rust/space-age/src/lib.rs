// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#![allow(unused_variables)]

pub struct Duration{
    time:f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration{time: (s as f64) / 31557600f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!();
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
    fn years_during(d: &Duration) -> f64{
        d.time / 0.2408467f64
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64{
        d.time / 0.61519726f64
        
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64{
        d.time / 1f64
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64{
        d.time / 1.8808158f64
        
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64{
        d.time / 11.862615f64
        
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64{
        d.time / 29.447498f64
        
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64{
        d.time / 84.016846f64
        
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64{
        d.time / 164.79132f64
        
    }
}