// Social Robotics Platform 04
// Desmond Germans, Ph.D
// Euler angles

use std::{cmp,fmt,ops};
use crate::*;

pub const TAU: f32 = 6.28318531;

#[derive(Copy,Clone)]
pub struct Euler<T> {
    pub y: T,
    pub p: T,
    pub b: T,
}

macro_rules! impl_euler (
    ($t:ty) => (
        impl Euler<$t> {
            pub fn new(y: $t,p: $t,b: $t) -> Euler<$t> {
                Euler {
                    y: y,
                    p: p,
                    b: b,
                }
            }
        }

        impl cmp::PartialEq for Euler<$t> {
            fn eq(&self,other: &Euler<$t>) -> bool {
                (self.y == other.y)
                && (self.p == other.p)
                && (self.b == other.b)
            }
        }

        impl fmt::Display for Euler<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{})",self.y,self.p,self.b)
            }
        }

        impl fmt::Debug for Euler<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{})",self.y,self.p,self.b)
            }
        }

        impl Zero for Euler<$t> {
            fn zero() -> Euler<$t> {
                Euler {
                    y: <$t>::zero(),
                    p: <$t>::zero(),
                    b: <$t>::zero(),
                }
            }
        }

        impl ops::Add<Euler<$t>> for Euler<$t> {
            type Output = Euler<$t>;
            fn add(self,other: Euler<$t>) -> Self::Output {
                Euler {
                    y: self.y + other.y,
                    p: self.p + other.p,
                    b: self.b + other.b,
                }
            }
        }

        impl ops::Sub<Euler<$t>> for Euler<$t> {
            type Output = Euler<$t>;
            fn sub(self,other: Euler<$t>) -> Self::Output {
                Euler {
                    y: self.y - other.y,
                    p: self.p - other.p,
                    b: self.b - other.b,
                }
            }
        }

        impl ops::AddAssign<Euler<$t>> for Euler<$t> {
            fn add_assign(&mut self,other: Euler<$t>) {
                self.y += other.y;
                self.p += other.p;
                self.b += other.b;
            }
        }

        impl ops::SubAssign<Euler<$t>> for Euler<$t> {
            fn sub_assign(&mut self,other: Euler<$t>) {
                self.y -= other.y;
                self.p -= other.p;
                self.b -= other.b;
            }
        }

        impl ops::Mul<Euler<$t>> for $t {
            type Output = Euler<$t>;
            fn mul(self,other: Euler<$t>) -> Self::Output {
                Euler {
                    y: self * other.y,
                    p: self * other.p,
                    b: self * other.b,
                }
            }
        }

        impl ops::Mul<$t> for Euler<$t> {
            type Output = Euler<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Euler {
                    y: self.y * other,
                    p: self.p * other,
                    b: self.b * other,
                }
            }
        }

        impl ops::MulAssign<$t> for Euler<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.y *= other;
                self.p *= other;
                self.b *= other;
            }
        }

        impl ops::Div<$t> for Euler<$t> {
            type Output = Euler<$t>;
            fn div(self,other: $t) -> Self::Output {
                Euler {
                    y: self.y / other,
                    p: self.p / other,
                    b: self.b / other,
                }
            }
        }

        impl ops::DivAssign<$t> for Euler<$t> {
            fn div_assign(&mut self,other: $t) {
                self.y /= other;
                self.p /= other;
                self.b /= other;
            }
        }

        impl ops::Neg for Euler<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Euler {
                    y: -self.y,
                    p: -self.p,
                    b: -self.b,
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32e = Euler<f32>;

impl_euler!(f32);

#[allow(non_camel_case_types)]
pub type f64e = Euler<f64>;

impl_euler!(f64);
