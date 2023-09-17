use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map_res},
};
use std::{
    ops::{Mul, MulAssign},
    time::Duration,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg(feature = "serde")]
#[serde(try_from = "&str", into = "String")]
pub struct Interval(Duration);

impl Default for Interval {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub const MIN: Interval = Interval(Duration::from_secs(60));
pub const HOUR: Interval = Interval(Duration::from_secs(60 * 60));
pub const DAY: Interval = Interval(Duration::from_secs(60 * 60 * 24));

impl Interval {
    #[inline]
    pub fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }
}

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

impl From<Duration> for Interval {
    fn from(value: Duration) -> Self {
        Interval(value)
    }
}

impl From<Interval> for Duration {
    fn from(value: Interval) -> Self {
        value.0
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

impl Mul<Interval> for u32 {
    type Output = Interval;

    fn mul(self, rhs: Interval) -> Interval {
        Interval(self * rhs.0)
    }
}

impl<'t> TryFrom<&'t str> for Interval {
    type Error = nom::Err<nom::error::Error<&'t str>>;

    fn try_from(input: &'t str) -> Result<Self, Self::Error> {
        let (input, interval) = map_res(digit1, |s: &str| s.parse::<u64>())(input)?;

        let (_, duration) = all_consuming(alt((tag("Min"), tag("H"), tag("D"))))(input)?;

        Ok(Interval(Duration::from_secs(
            interval
                * match duration {
                    "Min" => 60,
                    "H" => 3600,
                    "D" => 86400,
                    _ => todo!("not a valid value"),
                },
        )))
    }
}
