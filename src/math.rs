use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Not, Shr, Sub},
};

use serde::{Deserialize, Serialize};

pub fn frac(n: u32, d: u32) -> i64 {
    n as i64 / d as i64
}

pub fn cmp_f32(a: f32, b: f32) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Less)
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Point(pub i64, pub i64);

impl Point {
    pub fn dist_sq(&self, rhs: Self) -> i64 {
        (self.0 - rhs.0) * (self.0 - rhs.0) + (self.1 - rhs.1) * (self.1 - rhs.1)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.0)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.0)
    }
}

impl Mul for Point {
    type Output = i64;
    fn mul(self, rhs: Self) -> Self::Output {
        (self.0 * rhs.0) + (self.1 * rhs.1)
    }
}

impl Mul<i64> for Point {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Div for Point {
    type Output = i64;
    fn div(self, rhs: Self) -> Self::Output {
        (self.0 * rhs.1) - (self.1 * rhs.0)
    }
}

impl Not for Point {
    type Output = Self;
    fn not(self) -> Self::Output {
        self * -1
    }
}

impl Shr for Point {
    type Output = i64;
    fn shr(self, rhs: Self) -> Self::Output {
        let self_to_rhs = rhs - self;
        f64::sqrt((self_to_rhs * self_to_rhs) as f64).floor() as i64
    }
}
