// Social Robotics Platform 04
// Desmond Germans, Ph.D
// Colors

use std::{cmp,fmt,ops};
use crate::*;

#[derive(Copy,Clone)]
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

macro_rules! impl_color (
    ($t:ty) => (
        impl Color<$t> {
            pub fn new(r: $t,g: $t,b: $t) -> Color<$t> {
                Color {
                    r: r,
                    g: g,
                    b: b,
                }
            }
        }

        impl cmp::PartialEq for Color<$t> {
            fn eq(&self,other: &Color<$t>) -> bool {
                (self.r == other.r)
                && (self.g == other.g)
                && (self.b == other.b)
            }
        }

        impl fmt::Display for Color<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{})",self.r,self.g,self.b)
            }
        }

        impl fmt::Debug for Color<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{})",self.r,self.g,self.b)
            }
        }

        impl Zero for Color<$t> {
            fn zero() -> Color<$t> {
                Color {
                    r: <$t>::zero(),
                    g: <$t>::zero(),
                    b: <$t>::zero(),
                }
            }
        }

        impl ops::Neg for Color<$t> {
            type Output = Color<$t>;
            fn neg(self) -> Self::Output {
                Color {
                    r: -self.r,
                    g: -self.g,
                    b: -self.b,
                }
            }
        }

        impl ops::Add<Color<$t>> for Color<$t> {
            type Output = Color<$t>;
            fn add(self,other: Color<$t>) -> Self::Output {
                Color {
                    r: self.r + other.r,
                    g: self.g + other.g,
                    b: self.b + other.b,
                }
            }
        }

        impl ops::Sub<Color<$t>> for Color<$t> {
            type Output = Color<$t>;
            fn sub(self,other: Color<$t>) -> Color<$t> {
                Color {
                    r: self.r - other.r,
                    g: self.g - other.g,
                    b: self.b - other.b,
                }
            }
        }

        impl ops::AddAssign<Color<$t>> for Color<$t> {
            fn add_assign(&mut self,other: Color<$t>) {
                self.r += other.r;
                self.g += other.g;
                self.b += other.b;
            }
        }

        impl ops::SubAssign<Color<$t>> for Color<$t> {
            fn sub_assign(&mut self,other: Color<$t>) {
                self.r -= other.r;
                self.g -= other.g;
                self.b -= other.b;
            }
        }

        impl ops::Mul<$t> for Color<$t> {
            type Output = Color<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Color {
                    r: self.r * other,
                    g: self.g * other,
                    b: self.b * other,
                }
            }
        }
                    

        impl ops::Mul<Color<$t>> for $t {
            type Output = Color<$t>;
            fn mul(self,other: Color<$t>) -> Self::Output {
                Color {
                    r: self * other.r,
                    g: self * other.g,
                    b: self * other.b,
                }
            }
        }

        impl ops::Mul<Color<$t>> for Color<$t> {
            type Output = Color<$t>;
            fn mul(self,other: Color<$t>) -> Self::Output {
                Color {
                    r: self.r * other.r,
                    g: self.g * other.g,
                    b: self.b * other.b,
                }
            }
        }

        impl ops::MulAssign<$t> for Color<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.r *= other;
                self.g *= other;
                self.b *= other;
            }
        }

        impl ops::MulAssign<Color<$t>> for Color<$t> {
            fn mul_assign(&mut self,other: Color<$t>) {
                self.r *= other.r;
                self.g *= other.g;
                self.b *= other.b;
            }
        }

        impl ops::Div<$t> for Color<$t> {
            type Output = Color<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    Color {
                        r: self.r / other,
                        g: self.g / other,
                        b: self.b / other,
                    }    
                }
                else {
                    self
                }
            }
        }

        impl ops::DivAssign<$t> for Color<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.r /= other;
                    self.g /= other;
                    self.b /= other;
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32k = Color<f32>;

impl_color!(f32);

#[allow(non_camel_case_types)]
pub type f64k = Color<f64>;

impl_color!(f64);
