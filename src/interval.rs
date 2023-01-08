use std::{
    ops::{Mul, MulAssign},
    time::Duration,
};

#[derive(Debug, Clone, Copy)]
pub struct Interval(Duration);

pub const Min: Interval = Interval(Duration::from_secs(60));
pub const Hour: Interval = Interval(Duration::from_secs(60 * 60));
pub const Day: Interval = Interval(Duration::from_secs(60 * 60 * 24));

impl From<Interval> for String {
    fn from(value: Interval) -> Self {
        let min = value.0.as_secs() / 60;

        if min < 61 {
            return format!("{}Min", min);
        }

        let hours = min / 60;

        if hours < 25 {
            return format!("{}H", hours);
        }

        let days = hours / 24;
        return format!("{}D", days);
    }
}

impl Mul<Duration> for Interval {
    type Output = Self;

    fn mul(self, rhs: Duration) -> Self {
        Interval(Duration::from_secs_f64(
            rhs.as_secs_f64() * self.0.as_secs_f64(),
        ))
    }
}

impl MulAssign<Duration> for Interval {
    fn mul_assign(&mut self, rhs: Duration) {
        self.0 = Duration::from_secs_f64(rhs.as_secs_f64() * self.0.as_secs_f64());
    }
}

impl Mul<u32> for Interval {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        Interval(self.0 * rhs)
    }
}

impl MulAssign<u32> for Interval {
    fn mul_assign(&mut self, rhs: u32) {
        self.0 = self.0 * rhs;
    }
}
