// Social Robotics Platform 04
// Desmond Germans, Ph.D
// Complex numbers

use std::{cmp,fmt,ops};
use crate::*;

#[derive(Copy,Clone)]
pub struct Complex<T> {
    pub r: T,
    pub i: T,
}

macro_rules! impl_complex (
    ($t:ty) => (
        impl Complex<$t> {
            pub fn new(r: $t,i: $t) -> Complex<$t> {
                Complex {
                    r: r,
                    i: i,
                }
            }

            pub fn conj(self) -> Complex<$t> {
                Complex {
                    r: self.r,
                    i: -self.i,
                }
            }

            pub fn abs(self) -> $t {
                (self.r * self.r + self.i * self.i).sqrt()
            }

            pub fn arg(self) -> $t {
                self.r.atan2(self.i)
            }
        }

        impl cmp::PartialEq for Complex<$t> {
            fn eq(&self,other: &Complex<$t>) -> bool {
                (self.r == other.r)
                && (self.i == other.i)
            }
        }

        impl fmt::Display for Complex<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                let si = if self.i < 0.0 {
                    format!("{}i",self.i)
                } else {
                    format!("+{}i",self.i)
                };
                write!(f,"{}{}",self.r,si)
            }
        }

        impl fmt::Debug for Complex<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                let si = if self.i < 0.0 {
                    format!("{}i",self.i)
                } else {
                    format!("+{}i",self.i)
                };
                write!(f,"{}{}",self.r,si)
            }
        }

        impl Zero for Complex<$t> {
            fn zero() -> Complex<$t> {
                Complex {
                    r: 0.0,
                    i: 0.0,
                }
            }
        }

        impl One for Complex<$t> {
            fn one() -> Complex<$t> {
                Complex {
                    r: 1.0,
                    i: 0.0,
                }
            }
        }

        impl ops::Neg for Complex<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Complex {
                    r: -self.r,
                    i: -self.i,
                }
            }
        }

        impl ops::Add<$t> for Complex<$t> {
            type Output = Complex<$t>;
            fn add(self,other: $t) -> Self::Output {
                Complex {
                    r: self.r + other,
                    i: self.i,
                }
            }
        }

        impl ops::Add<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn add(self,other: Complex<$t>) -> Self::Output {
                Complex {
                    r: self + other.r,
                    i: other.i,
                }
            }
        }

        impl ops::Add<Complex<$t>> for Complex<$t> {
            type Output = Complex<$t>;
            fn add(self,other: Complex<$t>) -> Self::Output {
                Complex {
                    r: self.r + other.r,
                    i: self.i + other.i,
                }
            }
        }

        impl ops::AddAssign<$t> for Complex<$t> {
            fn add_assign(&mut self,other: $t) {
                self.r += other;
            }
        }

        impl ops::AddAssign<Complex<$t>> for Complex<$t> {
            fn add_assign(&mut self,other: Complex<$t>) {
                self.r += other.r;
                self.i += other.i;
            }
        }

        impl ops::Sub<$t> for Complex<$t> {
            type Output = Complex<$t>;
            fn sub(self,other: $t) -> Self::Output {
                Complex {
                    r: self.r - other,
                    i: self.i,
                }
            }
        }

        impl ops::Sub<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn sub(self,other: Complex<$t>) -> Self::Output {
                Complex {
                    r: self - other.r,
                    i: -other.i,
                }
            }
        }

        impl ops::Sub<Complex<$t>> for Complex<$t> {
            type Output = Complex<$t>;
            fn sub(self,other: Complex<$t>) -> Self::Output {
                Complex {
                    r: self.r - other.r,
                    i: self.i - other.i,
                }
            }
        }

        impl ops::SubAssign<$t> for Complex<$t> {
            fn sub_assign(&mut self,other: $t) {
                self.r += other;
            }
        }

        impl ops::SubAssign<Complex<$t>> for Complex<$t> {
            fn sub_assign(&mut self,other: Complex<$t>) {
                self.r += other.r;
                self.i += other.i;
            }
        }

        impl ops::Mul<$t> for Complex<$t> {
            type Output = Complex<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Complex {
                    r: self.r * other,
                    i: self.i * other,
                }
            }
        }

        impl ops::Mul<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn mul(self,other: Complex<$t>) -> Self::Output {
                Complex {
                    r: self * other.r,
                    i: self * other.i,
                }
            }
        }

        impl ops::Mul<Complex<$t>> for Complex<$t> {
            type Output = Complex<$t>;
            fn mul(self,other: Complex<$t>) -> Self::Output {
                Complex {
                    r: self.r * other.r - self.i * other.i,
                    i: self.r * other.i - self.i * other.r,
                }
            }
        }

        impl ops::MulAssign<$t> for Complex<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.r *= other;
                self.i *= other;
            }
        }

        impl ops::MulAssign<Complex<$t>> for Complex<$t> {
            fn mul_assign(&mut self,other: Complex<$t>) {
                let nr = self.r * other.r - self.i * other.i;
                let ni = self.r * other.i - self.i * other.r;
                self.r = nr;
                self.i = ni;
            }
        }

        impl ops::Div<$t> for Complex<$t> {
            type Output = Complex<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    Complex {
                        r: self.r / other,
                        i: self.i / other,
                    }
                }
                else {
                    self
                }
            }
        }

        impl ops::Div<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn div(self,other: Complex<$t>) -> Self::Output {
                let d = other.r * other.r + other.i * other.i;
                if d != 0.0 {
                    Complex {
                        r: self * other.r / d,
                        i: self * other.i / d,
                    }
                }
                else {
                    Complex {
                        r: self,
                        i: 0.0,
                    }
                }
            }
        }

        impl ops::Div<Complex<$t>> for Complex<$t> {
            type Output = Complex<$t>;
            fn div(self,other: Complex<$t>) -> Self::Output {
                let d = other.r * other.r + other.i * other.i;
                if d != 0.0 {
                    Complex {
                        r: (self.r * other.r + self.i * other.i) / d,
                        i: (self.r * other.i + self.i * other.r) / d,
                    }
                }
                else {
                    self
                }
            }
        }

        impl ops::DivAssign<$t> for Complex<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.r /= other;
                    self.i /= other;
                }
            }
        }

        impl ops::DivAssign<Complex<$t>> for Complex<$t> {
            fn div_assign(&mut self,other: Complex<$t>) {
                let d = other.r * other.r + other.i * other.i;
                if d != 0.0 {
                    let nr = (self.r * other.r + self.i * other.i) / d;
                    let ni = (self.r * other.i + self.i * other.r) / d;
                    self.r = nr;
                    self.i = ni;
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32c = Complex<f32>;

impl_complex!(f32);

#[allow(non_camel_case_types)]
pub type f64c = Complex<f64>;

impl_complex!(f64);
