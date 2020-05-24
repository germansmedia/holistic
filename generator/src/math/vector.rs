// Social Robotics Platform 04
// Desmond Germans, Ph.D
// Vectors

use std::{cmp,fmt,ops};
use crate::*;

#[derive(Copy,Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

macro_rules! impl_vec2 (
    ($t:ty) => (
        impl Vec2<$t> {
            pub fn new(x: $t,y: $t) -> Vec2<$t> {
                Vec2 {
                    x: x,
                    y: y,
                }
            }
        }

        impl cmp::PartialEq for Vec2<$t> {
            fn eq(&self,other: &Vec2<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
            }
        }

        impl fmt::Display for Vec2<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{})",self.x,self.y)
            }
        }

        impl fmt::Debug for Vec2<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{})",self.x,self.y)
            }
        }

        impl Zero for Vec2<$t> {
            fn zero() -> Vec2<$t> {
                Vec2 {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                }
            }
        }

        impl ops::Add<Vec2<$t>> for Vec2<$t> {
            type Output = Vec2<$t>;
            fn add(self,other: Vec2<$t>) -> Self::Output {
                Vec2 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        impl ops::Sub<Vec2<$t>> for Vec2<$t> {
            type Output = Vec2<$t>;
            fn sub(self,other: Vec2<$t>) -> Self::Output {
                Vec2 {
                    x: self.x - other.x,
                    y: self.y - other.y,
                }
            }
        }

        impl ops::AddAssign<Vec2<$t>> for Vec2<$t> {
            fn add_assign(&mut self,other: Vec2<$t>) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        impl ops::SubAssign<Vec2<$t>> for Vec2<$t> {
            fn sub_assign(&mut self,other: Vec2<$t>) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }

        impl ops::Mul<Vec2<$t>> for $t {
            type Output = Vec2<$t>;
            fn mul(self,other: Vec2<$t>) -> Self::Output {
                Vec2 {
                    x: self * other.x,
                    y: self * other.y,
                }
            }
        }

        impl ops::Mul<$t> for Vec2<$t> {
            type Output = Vec2<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Vec2 {
                    x: self.x * other,
                    y: self.y * other,
                }
            }
        }

        impl ops::MulAssign<$t> for Vec2<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
            }
        }

        impl ops::Div<$t> for Vec2<$t> {
            type Output = Vec2<$t>;
            fn div(self,other: $t) -> Self::Output {
                Vec2 {
                    x: self.x / other,
                    y: self.y / other,
                }
            }
        }

        impl ops::DivAssign<$t> for Vec2<$t> {
            fn div_assign(&mut self,other: $t) {
                self.x /= other;
                self.y /= other;
            }
        }
    );
);

macro_rules! impl_vec2_neg (
    ($t:ty) => (
        impl ops::Neg for Vec2<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Vec2 {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type u8v2 = Vec2<u8>;

impl_vec2!(u8);

#[allow(non_camel_case_types)]
pub type i8v2 = Vec2<i8>;

impl_vec2!(i8);
impl_vec2_neg!(i8);

#[allow(non_camel_case_types)]
pub type u16v2 = Vec2<u16>;

impl_vec2!(u16);

#[allow(non_camel_case_types)]
pub type i16v2 = Vec2<i16>;

impl_vec2!(i16);
impl_vec2_neg!(i16);

#[allow(non_camel_case_types)]
pub type u32v2 = Vec2<u32>;

impl_vec2!(u32);

#[allow(non_camel_case_types)]
pub type i32v2 = Vec2<i32>;

impl_vec2!(i32);
impl_vec2_neg!(i32);

#[allow(non_camel_case_types)]
pub type u64v2 = Vec2<u64>;

impl_vec2!(u64);

#[allow(non_camel_case_types)]
pub type i64v2 = Vec2<i64>;

impl_vec2!(i64);
impl_vec2_neg!(i64);

#[allow(non_camel_case_types)]
pub type f32v2 = Vec2<f32>;

impl_vec2!(f32);
impl_vec2_neg!(f32);

#[allow(non_camel_case_types)]
pub type f64v2 = Vec2<f64>;

impl_vec2!(f64);
impl_vec2_neg!(f64);

#[allow(non_camel_case_types)]
pub type usizev2 = Vec2<usize>;

impl_vec2!(usize);

#[allow(non_camel_case_types)]
pub type isizev2 = Vec2<isize>;

impl_vec2!(isize);
impl_vec2_neg!(isize);

#[derive(Copy,Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

macro_rules! impl_vec3 (
    ($t:ty) => (
        impl Vec3<$t> {
            pub fn new(x: $t,y: $t,z: $t) -> Vec3<$t> {
                Vec3 {
                    x: x,
                    y: y,
                    z: z,
                }
            }

            pub fn cross(a: Vec3<$t>,b: Vec3<$t>) -> Vec3<$t> {
                Vec3 {
                    x: a.y * b.z - a.z * b.y,
                    y: a.z * b.x - a.x * b.z,
                    z: a.x * b.y - a.y * b.x,
                }
            }

            pub fn dot(a: Vec3<$t>,b: Vec3<$t>) -> $t {
                a.x * b.x + a.y * b.y + a.z * b.z
            }

            pub fn abs(&self) -> $t {
                (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
            }

            pub fn norm(self) -> Vec3<$t> {
                let d = self.abs();
                if d != 0.0 {
                    self / d
                }
                else {
                    self
                }
            }
        }

        impl cmp::PartialEq for Vec3<$t> {
            fn eq(&self,other: &Vec3<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
                && (self.z == other.z)
            }
        }

        impl fmt::Display for Vec3<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{})",self.x,self.y,self.z)
            }
        }

        impl fmt::Debug for Vec3<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{})",self.x,self.y,self.z)
            }
        }

        impl Zero for Vec3<$t> {
            fn zero() -> Vec3<$t> {
                Vec3 {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                }
            }
        }

        impl ops::Neg for Vec3<$t> {
            type Output = Vec3<$t>;
            fn neg(self) -> Self::Output {
                Vec3 {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                }
            }
        }

        impl ops::Add<Vec3<$t>> for Vec3<$t> {
            type Output = Vec3<$t>;
            fn add(self,other: Vec3<$t>) -> Self::Output {
                Vec3 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        }

        impl ops::Sub<Vec3<$t>> for Vec3<$t> {
            type Output = Vec3<$t>;
            fn sub(self,other: Vec3<$t>) -> Vec3<$t> {
                Vec3 {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                }
            }
        }

        impl ops::AddAssign<Vec3<$t>> for Vec3<$t> {
            fn add_assign(&mut self,other: Vec3<$t>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        impl ops::SubAssign<Vec3<$t>> for Vec3<$t> {
            fn sub_assign(&mut self,other: Vec3<$t>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }

        impl ops::Mul<$t> for Vec3<$t> {
            type Output = Vec3<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Vec3 {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                }
            }
        }
                    

        impl ops::Mul<Vec3<$t>> for $t {
            type Output = Vec3<$t>;
            fn mul(self,other: Vec3<$t>) -> Self::Output {
                Vec3 {
                    x: self * other.x,
                    y: self * other.y,
                    z: self * other.z,
                }
            }
        }

        impl ops::MulAssign<$t> for Vec3<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
            }
        }

        impl ops::Div<$t> for Vec3<$t> {
            type Output = Vec3<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    Vec3 {
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                    }    
                }
                else {
                    self
                }
            }
        }

        impl ops::DivAssign<$t> for Vec3<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32v3 = Vec3<f32>;

impl_vec3!(f32);

#[allow(non_camel_case_types)]
pub type f64v3 = Vec3<f64>;

impl_vec3!(f64);

#[derive(Copy,Clone)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

macro_rules! impl_vec4 (
    ($t:ty) => (
        impl Vec4<$t> {
            pub fn new(x: $t,y: $t,z: $t,w: $t) -> Vec4<$t> {
                Vec4 {
                    x: x,
                    y: y,
                    z: z,
                    w: w,
                }
            }
        }

        impl cmp::PartialEq for Vec4<$t> {
            fn eq(&self,other: &Vec4<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
                && (self.z == other.z)
                && (self.w == other.w)
            }
        }

        impl fmt::Display for Vec4<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{},{})",self.x,self.y,self.z,self.w)
            }
        }

        impl fmt::Debug for Vec4<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{},{})",self.x,self.y,self.z,self.w)
            }
        }

        impl Zero for Vec4<$t> {
            fn zero() -> Vec4<$t> {
                Vec4 {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                    w: <$t>::zero(),
                }
            }
        }

        impl ops::Neg for Vec4<$t> {
            type Output = Vec4<$t>;
            fn neg(self) -> Self::Output {
                Vec4 {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    w: -self.w,
                }
            }
        }

        impl ops::Add<Vec4<$t>> for Vec4<$t> {
            type Output = Vec4<$t>;
            fn add(self,other: Vec4<$t>) -> Self::Output {
                Vec4 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                    w: self.w + other.w,
                }
            }
        }

        impl ops::Sub<Vec4<$t>> for Vec4<$t> {
            type Output = Vec4<$t>;
            fn sub(self,other: Vec4<$t>) -> Vec4<$t> {
                Vec4 {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                    w: self.w - other.w,
                }
            }
        }

        impl ops::AddAssign<Vec4<$t>> for Vec4<$t> {
            fn add_assign(&mut self,other: Vec4<$t>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
                self.w += other.w;
            }
        }

        impl ops::SubAssign<Vec4<$t>> for Vec4<$t> {
            fn sub_assign(&mut self,other: Vec4<$t>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
                self.w -= other.w;
            }
        }

        impl ops::Mul<$t> for Vec4<$t> {
            type Output = Vec4<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Vec4 {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                    w: self.w * other,
                }
            }
        }

        impl ops::Mul<Vec4<$t>> for $t {
            type Output = Vec4<$t>;
            fn mul(self,other: Vec4<$t>) -> Self::Output {
                Vec4 {
                    x: self * other.x,
                    y: self * other.y,
                    z: self * other.z,
                    w: self * other.w,
                }
            }
        }

        impl ops::MulAssign<$t> for Vec4<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
                self.w *= other;
            }
        }

        impl ops::Div<$t> for Vec4<$t> {
            type Output = Vec4<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    Vec4 {
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                        w: self.w / other,
                    }    
                }
                else {
                    self
                }
            }
        }

        impl ops::DivAssign<$t> for Vec4<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                    self.w /= other;
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32v4 = Vec4<f32>;

impl_vec4!(f32);

#[allow(non_camel_case_types)]
pub type f64v4 = Vec4<f64>;

impl_vec4!(f64);
