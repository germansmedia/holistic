// Social Robotics Platform 04
// Desmond Germans, Ph.D
// Multivectors

use std::{cmp,fmt,ops};
use crate::*;

#[derive(Clone)]
pub struct MultiVec2<T> {
    pub r: T,
    pub x: T,
    pub y: T,
    pub xy: T,
}

macro_rules! impl_multivec2 (
    ($t:ty) => (
        impl MultiVec2<$t> {
            pub fn new(r: $t,x: $t,y: $t,xy: $t) -> MultiVec2<$t> {
                MultiVec2 {
                    r: r,
                    x: x,
                    y: y,
                    xy: xy,
                }
            }
        }

        impl cmp::PartialEq for MultiVec2<$t> {
            fn eq(&self,other: &MultiVec2<$t>) -> bool {
                (self.r == other.r)
                && (self.x == other.x)
                && (self.y == other.y)
                && (self.xy == other.xy)
            }
        }            

        impl fmt::Display for MultiVec2<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                let sx = if self.x < <$t>::zero() {
                    format!("{}x",self.x)
                } else {
                    format!("+{}x",self.x)
                };
                let sy = if self.y < <$t>::zero() {
                    format!("{}y",self.y)
                } else {
                    format!("+{}y",self.y)
                };
                let sxy = if self.xy < <$t>::zero() {
                    format!("{}xy",self.xy)
                } else {
                    format!("+{}xy",self.xy)
                };
                write!(f,"{}{}{}{}",
                    self.r,
                    sx,sy,
                    sxy)
            }
        }            

        impl fmt::Debug for MultiVec2<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                let sx = if self.x < <$t>::zero() {
                    format!("{}x",self.x)
                } else {
                    format!("+{}x",self.x)
                };
                let sy = if self.y < <$t>::zero() {
                    format!("{}y",self.y)
                } else {
                    format!("+{}y",self.y)
                };
                let sxy = if self.xy < <$t>::zero() {
                    format!("{}xy",self.xy)
                } else {
                    format!("+{}xy",self.xy)
                };
                write!(f,"{}{}{}{}",
                    self.r,
                    sx,sy,
                    sxy)
            }
        }

        impl Zero for MultiVec2<$t> {
            fn zero() -> MultiVec2<$t> {
                MultiVec2 {
                    r: <$t>::zero(),
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    xy: <$t>::zero(),
                }
            }
        }

        impl ops::Neg for MultiVec2<$t> {
            type Output = MultiVec2<$t>;
            fn neg(self) -> MultiVec2<$t> {
                MultiVec2 {
                    r: -self.r,
                    x: -self.x,
                    y: -self.y,
                    xy: -self.xy,
                }
            }
        }

        impl ops::Add<MultiVec2<$t>> for MultiVec2<$t> {
            type Output = MultiVec2<$t>;
            fn add(self,other: MultiVec2<$t>) -> Self::Output {
                MultiVec2 {
                    r: self.r + other.r,
                    x: self.x + other.x,
                    y: self.y + other.y,
                    xy: self.xy + other.xy,
                }
            }
        }

        impl ops::Sub<MultiVec2<$t>> for MultiVec2<$t> {
            type Output = MultiVec2<$t>;
            fn sub(self,other: MultiVec2<$t>) -> Self::Output {
                MultiVec2 {
                    r: self.r - other.r,
                    x: self.x - other.x,
                    y: self.y - other.y,
                    xy: self.xy - other.xy,
                }
            }
        }

        impl ops::AddAssign<MultiVec2<$t>> for MultiVec2<$t> {
            fn add_assign(&mut self,other: MultiVec2<$t>) {
                self.r += other.r;
                self.x += other.x;
                self.y += other.y;
                self.xy += other.xy;
            }
        }

        impl ops::SubAssign<MultiVec2<$t>> for MultiVec2<$t> {
            fn sub_assign(&mut self,other: MultiVec2<$t>) {
                self.r -= other.r;
                self.x -= other.x;
                self.y -= other.y;
                self.xy -= other.xy;
            }
        }

        impl ops::Mul<$t> for MultiVec2<$t> {
            type Output = MultiVec2<$t>;
            fn mul(self,other: $t) -> Self::Output {
                MultiVec2 {
                    r: self.r * other,
                    x: self.x * other,
                    y: self.y * other,
                    xy: self.xy * other,
                }
            }
        }

        impl ops::Mul<MultiVec2<$t>> for $t {
            type Output = MultiVec2<$t>;
            fn mul(self,other: MultiVec2<$t>) -> Self::Output {
                MultiVec2 {
                    r: self * other.r,
                    x: self * other.x,
                    y: self * other.y,
                    xy: self * other.xy,
                }
            }
        }

        impl ops::Mul<MultiVec2<$t>> for MultiVec2<$t> {
            type Output = MultiVec2<$t>;
            fn mul(self,other: MultiVec2<$t>) -> Self::Output {
                MultiVec2 {
                    r: self.r * other.r - self.x * other.x - self.y * other.y - self.xy * other.xy,
                    x: self.r * other.x + self.x * other.r + self.y * other.xy - self.xy * other.y,
                    y: self.r * other.y + self.y * other.r - self.x * other.xy + self.xy * other.x,
                    xy: self.r * other.xy + self.xy * other.r + self.x * other.y - self.y * other.x,
                }
            }
        }

        impl ops::Div<$t> for MultiVec2<$t> {
            type Output = MultiVec2<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    MultiVec2 {
                        r: self.r / other,
                        x: self.x / other,
                        y: self.y / other,
                        xy: self.xy / other,
                    }
                }
                else {
                    self
                }
            }
        }

        impl ops::MulAssign<$t> for MultiVec2<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.r *= other;
                self.x *= other;
                self.y *= other;
                self.xy *= other;
            }
        }

        impl ops::MulAssign<MultiVec2<$t>> for MultiVec2<$t> {
            fn mul_assign(&mut self,other: MultiVec2<$t>) {
                let nr = self.r * other.r - self.x * other.x - self.y * other.y - self.xy * other.xy;
                let nx = self.r * other.x + self.x * other.r + self.y * other.xy - self.xy * other.y;
                let ny = self.r * other.y + self.y * other.r - self.x * other.xy + self.xy * other.x;
                let nxy = self.r * other.xy + self.xy * other.r + self.x * other.y - self.y * other.x;
                self.r = nr;
                self.x = nx;
                self.y = ny;
                self.xy = nxy;   
            }
        }

        impl ops::DivAssign<$t> for MultiVec2<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.r /= other;
                    self.x /= other;
                    self.y /= other;
                    self.xy /= other;
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32m2 = MultiVec2<f32>;

impl_multivec2!(f32);

#[allow(non_camel_case_types)]
pub type f64m2 = MultiVec2<f64>;

impl_multivec2!(f64);

#[derive(Clone)]
pub struct MultiVec3<T> {
    pub r: T,
    pub x: T,
    pub y: T,
    pub z: T,
    pub xy: T,
    pub xz: T,
    pub yz: T,
    pub xyz: T,
}

macro_rules! impl_multivec3 (
    ($t:ty) => (
        impl MultiVec3<$t> {
            pub fn new(r: $t,x: $t,y: $t,z: $t,xy: $t,xz: $t,yz: $t,xyz: $t) -> MultiVec3<$t> {
                MultiVec3 {
                    r: r,
                    x: x,
                    y: y,
                    z: z,
                    xy: xy,
                    xz: xz,
                    yz: yz,
                    xyz: xyz,
                }
            }
        }

        impl cmp::PartialEq for MultiVec3<$t> {
            fn eq(&self,other: &MultiVec3<$t>) -> bool {
                (self.r == other.r)
                && (self.x == other.x)
                && (self.y == other.y)
                && (self.z == other.z)
                && (self.xy == other.xy)
                && (self.xz == other.xz)
                && (self.yz == other.yz)
                && (self.xyz == other.xyz)
            }
        }            

        impl fmt::Display for MultiVec3<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                let sx = if self.x < <$t>::zero() {
                    format!("{}x",self.x)
                } else {
                    format!("+{}x",self.x)
                };
                let sy = if self.y < <$t>::zero() {
                    format!("{}y",self.y)
                } else {
                    format!("+{}y",self.y)
                };
                let sz = if self.z < <$t>::zero() {
                    format!("{}z",self.z)
                } else {
                    format!("+{}z",self.z)
                };
                let sxy = if self.xy < <$t>::zero() {
                    format!("{}xy",self.xy)
                } else {
                    format!("+{}xy",self.xy)
                };
                let sxz = if self.xz < <$t>::zero() {
                    format!("{}xz",self.xz)
                } else {
                    format!("+{}xz",self.xz)
                };
                let syz = if self.yz < <$t>::zero() {
                    format!("{}yz",self.yz)
                } else {
                    format!("+{}yz",self.yz)
                };
                let sxyz = if self.xyz < <$t>::zero() {
                    format!("{}xyz",self.xyz)
                } else {
                    format!("+{}xyz",self.xyz)
                };
                write!(f,"{}{}{}{}{}{}{}{}",
                    self.r,
                    sx,sy,sz,
                    sxy,sxz,syz,
                    sxyz)
            }
        }            

        impl fmt::Debug for MultiVec3<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                let sx = if self.x < <$t>::zero() {
                    format!("{}x",self.x)
                } else {
                    format!("+{}x",self.x)
                };
                let sy = if self.y < <$t>::zero() {
                    format!("{}y",self.y)
                } else {
                    format!("+{}y",self.y)
                };
                let sz = if self.z < <$t>::zero() {
                    format!("{}z",self.z)
                } else {
                    format!("+{}z",self.z)
                };
                let sxy = if self.xy < <$t>::zero() {
                    format!("{}xy",self.xy)
                } else {
                    format!("+{}xy",self.xy)
                };
                let sxz = if self.xz < <$t>::zero() {
                    format!("{}xz",self.xz)
                } else {
                    format!("+{}xz",self.xz)
                };
                let syz = if self.yz < <$t>::zero() {
                    format!("{}yz",self.yz)
                } else {
                    format!("+{}yz",self.yz)
                };
                let sxyz = if self.xyz < <$t>::zero() {
                    format!("{}xyz",self.xyz)
                } else {
                    format!("+{}xyz",self.xyz)
                };
                write!(f,"{}{}{}{}{}{}{}{}",
                    self.r,
                    sx,sy,sz,
                    sxy,sxz,syz,
                    sxyz)
            }
        }

        impl Zero for MultiVec3<$t> {
            fn zero() -> MultiVec3<$t> {
                MultiVec3 {
                    r: <$t>::zero(),
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                    xy: <$t>::zero(),
                    xz: <$t>::zero(),
                    yz: <$t>::zero(),
                    xyz: <$t>::zero(),
                }
            }
        }

        impl ops::Neg for MultiVec3<$t> {
            type Output = MultiVec3<$t>;
            fn neg(self) -> MultiVec3<$t> {
                MultiVec3 {
                    r: -self.r,
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    xy: -self.xy,
                    xz: -self.xz,
                    yz: -self.yz,
                    xyz: -self.xyz,
                }
            }
        }

        impl ops::Add<MultiVec3<$t>> for MultiVec3<$t> {
            type Output = MultiVec3<$t>;
            fn add(self,other: MultiVec3<$t>) -> Self::Output {
                MultiVec3 {
                    r: self.r + other.r,
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                    xy: self.xy + other.xy,
                    xz: self.xz + other.xz,
                    yz: self.yz + other.yz,
                    xyz: self.xyz + other.xyz,
                }
            }
        }

        impl ops::Sub<MultiVec3<$t>> for MultiVec3<$t> {
            type Output = MultiVec3<$t>;
            fn sub(self,other: MultiVec3<$t>) -> Self::Output {
                MultiVec3 {
                    r: self.r - other.r,
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                    xy: self.xy - other.xy,
                    xz: self.xz - other.xz,
                    yz: self.yz - other.yz,
                    xyz: self.xyz - other.xyz,
                }
            }
        }

        impl ops::AddAssign<MultiVec3<$t>> for MultiVec3<$t> {
            fn add_assign(&mut self,other: MultiVec3<$t>) {
                self.r += other.r;
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
                self.xy += other.xy;
                self.xz += other.xz;
                self.yz += other.yz;
                self.xyz += other.xyz;
            }
        }

        impl ops::SubAssign<MultiVec3<$t>> for MultiVec3<$t> {
            fn sub_assign(&mut self,other: MultiVec3<$t>) {
                self.r -= other.r;
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
                self.xy -= other.xy;
                self.xz -= other.xz;
                self.yz -= other.yz;
                self.xyz -= other.xyz;
            }
        }

        impl ops::Mul<$t> for MultiVec3<$t> {
            type Output = MultiVec3<$t>;
            fn mul(self,other: $t) -> Self::Output {
                MultiVec3 {
                    r: self.r * other,
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                    xy: self.xy * other,
                    xz: self.xz * other,
                    yz: self.yz * other,
                    xyz: self.xyz * other,
                }
            }
        }

        impl ops::Mul<MultiVec3<$t>> for $t {
            type Output = MultiVec3<$t>;
            fn mul(self,other: MultiVec3<$t>) -> Self::Output {
                MultiVec3 {
                    r: self * other.r,
                    x: self * other.x,
                    y: self * other.y,
                    z: self * other.z,
                    xy: self * other.xy,
                    xz: self * other.xz,
                    yz: self * other.yz,
                    xyz: self * other.xyz,
                }
            }
        }

        impl ops::Mul<MultiVec3<$t>> for MultiVec3<$t> {
            type Output = MultiVec3<$t>;
            fn mul(self,other: MultiVec3<$t>) -> Self::Output {
                MultiVec3 {
                    r: self.r * other.r + self.x * other.x + self.y * other.y + self.z * other.z - self.xy * other.xy - self.xz * other.xz - self.yz * other.yz - self.xyz * other.xyz,
                    x: self.r * other.x + self.x * other.r - self.y * other.xy - self.z * other.xz + self.xy * other.y + self.xz * other.z - self.yz * other.xyz - self.xyz * other.yz,
                    y: self.r * other.y + self.x * other.xy + self.y * other.r - self.z * other.yz - self.xy * other.x + self.xz * other.xyz + self.yz * other.z + self.xyz * other.xz,
                    z: self.r * other.z + self.x * other.xz + self.y * other.yz + self.z * other.r - self.xy * other.xyz - self.xz * other.x - self.yz * other.y - self.xyz * other.xy,
                    xy: self.r * other.xy + self.x * other.y - self.y * other.x + self.z * other.xyz + self.xy * other.r - self.xz * other.yz + self.yz * other.xz + self.xyz * other.z,
                    xz: self.r * other.xz + self.x * other.z - self.y * other.xyz - self.z * other.x + self.xy * other.yz + self.xz * other.r - self.yz * other.xy - self.xyz * other.y,
                    yz: self.r * other.yz + self.x * other.xyz + self.y * other.z - self.z * other.y - self.xy * other.xz + self.xz * other.xy + self.yz * other.r + self.xyz * other.x,
                    xyz: self.r * other.xyz + self.x * other.yz + self.y * other.xz + self.z * other.xy + self.xy * other.z - self.xz * other.y + self.yz * other.x + self.xyz * other.r,
                }
            }
        }

        impl ops::Div<$t> for MultiVec3<$t> {
            type Output = MultiVec3<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    MultiVec3 {
                        r: self.r / other,
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                        xy: self.xy / other,
                        xz: self.xz / other,
                        yz: self.yz / other,
                        xyz: self.xyz / other,
                    }
                }
                else {
                    self
                }
            }
        }

        impl ops::MulAssign<$t> for MultiVec3<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.r *= other;
                self.x *= other;
                self.y *= other;
                self.z *= other;
                self.xy *= other;
                self.xz *= other;
                self.yz *= other;
                self.xyz *= other;
            }
        }

        impl ops::MulAssign<MultiVec3<$t>> for MultiVec3<$t> {
            fn mul_assign(&mut self,other: MultiVec3<$t>) {
                let nr = self.r * other.r + self.x * other.x + self.y * other.y + self.z * other.z - self.xy * other.xy - self.xz * other.xz - self.yz * other.yz - self.xyz * other.xyz;
                let nx = self.r * other.x + self.x * other.r - self.y * other.xy - self.z * other.xz + self.xy * other.y + self.xz * other.z - self.yz * other.xyz - self.xyz * other.yz;
                let ny = self.r * other.y + self.x * other.xy + self.y * other.r - self.z * other.yz - self.xy * other.x + self.xz * other.xyz + self.yz * other.z + self.xyz * other.xz;
                let nz = self.r * other.z + self.x * other.xz + self.y * other.yz + self.z * other.r - self.xy * other.xyz - self.xz * other.x - self.yz * other.y - self.xyz * other.xy;
                let nxy = self.r * other.xy + self.x * other.y - self.y * other.x + self.z * other.xyz + self.xy * other.r - self.xz * other.yz + self.yz * other.xz + self.xyz * other.z;
                let nxz = self.r * other.xz + self.x * other.z - self.y * other.xyz - self.z * other.x + self.xy * other.yz + self.xz * other.r - self.yz * other.xy - self.xyz * other.y;
                let nyz = self.r * other.yz + self.x * other.xyz + self.y * other.z - self.z * other.y - self.xy * other.xz + self.xz * other.xy + self.yz * other.r + self.xyz * other.x;
                let nxyz = self.r * other.xyz + self.x * other.yz + self.y * other.xz + self.z * other.xy + self.xy * other.z - self.xz * other.y + self.yz * other.x + self.xyz * other.r;
                self.r = nr;
                self.x = nx;
                self.y = ny;
                self.z = nz;
                self.xy = nxy;   
                self.xz = nxz;   
                self.yz = nyz;
                self.xyz = nxyz;
            }
        }

        impl ops::DivAssign<$t> for MultiVec3<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.r /= other;
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                    self.xy /= other;
                    self.xz /= other;
                    self.yz /= other;
                    self.xyz /= other;
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32m3 = MultiVec3<f32>;

impl_multivec3!(f32);

#[allow(non_camel_case_types)]
pub type f64m3 = MultiVec3<f64>;

impl_multivec3!(f64);


#[derive(Clone)]
pub struct MultiVec4<T> {
    pub r: T,
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
    pub xy: T,
    pub xz: T,
    pub xw: T,
    pub yz: T,
    pub yw: T,
    pub zw: T,
    pub xyz: T,
    pub xyw: T,
    pub xzw: T,
    pub yzw: T,
    pub xyzw: T,
}

macro_rules! impl_multivec4 (
    ($t:ty) => (
        impl MultiVec4<$t> {
            pub fn new(r: $t,x: $t,y: $t,z: $t,w: $t,xy: $t,xz: $t,xw: $t,yz: $t,yw: $t,zw: $t,xyz: $t,xyw: $t,xzw: $t,yzw: $t,xyzw: $t) -> MultiVec4<$t> {
                MultiVec4 {
                    r: r,
                    x: x,
                    y: y,
                    z: z,
                    w: w,
                    xy: xy,
                    xz: xz,
                    xw: xw,
                    yz: yz,
                    yw: yw,
                    zw: zw,
                    xyz: xyz,
                    xyw: xyw,
                    xzw: xzw,
                    yzw: yzw,
                    xyzw: xyzw,
                }
            }
        }

        impl cmp::PartialEq for MultiVec4<$t> {
            fn eq(&self,other: &MultiVec4<$t>) -> bool {
                (self.r == other.r)
                && (self.x == other.x)
                && (self.y == other.y)
                && (self.z == other.z)
                && (self.w == other.w)
                && (self.xy == other.xy)
                && (self.xz == other.xz)
                && (self.xw == other.xw)
                && (self.yz == other.yz)
                && (self.yw == other.yw)
                && (self.zw == other.zw)
                && (self.xyz == other.xyz)
                && (self.xyw == other.xyw)
                && (self.xzw == other.xzw)
                && (self.yzw == other.yzw)
                && (self.xyzw == other.xyzw)
            }
        }            

        impl fmt::Display for MultiVec4<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                let sx = if self.x < <$t>::zero() {
                    format!("{}x",self.x)
                } else {
                    format!("+{}x",self.x)
                };
                let sy = if self.y < <$t>::zero() {
                    format!("{}y",self.y)
                } else {
                    format!("+{}y",self.y)
                };
                let sz = if self.z < <$t>::zero() {
                    format!("{}z",self.z)
                } else {
                    format!("+{}z",self.z)
                };
                let sw = if self.w < <$t>::zero() {
                    format!("{}w",self.w)
                } else {
                    format!("+{}w",self.w)
                };
                let sxy = if self.xy < <$t>::zero() {
                    format!("{}xy",self.xy)
                } else {
                    format!("+{}xy",self.xy)
                };
                let sxz = if self.xz < <$t>::zero() {
                    format!("{}xz",self.xz)
                } else {
                    format!("+{}xz",self.xz)
                };
                let sxw = if self.xw < <$t>::zero() {
                    format!("{}xw",self.xw)
                } else {
                    format!("+{}xw",self.xw)
                };
                let syz = if self.yz < <$t>::zero() {
                    format!("{}yz",self.yz)
                } else {
                    format!("+{}yz",self.yz)
                };
                let syw = if self.yw < <$t>::zero() {
                    format!("{}yw",self.yw)
                } else {
                    format!("+{}yw",self.yw)
                };
                let szw = if self.zw < <$t>::zero() {
                    format!("{}zw",self.zw)
                } else {
                    format!("+{}zw",self.zw)
                };
                let sxyz = if self.xyz < <$t>::zero() {
                    format!("{}xyz",self.xyz)
                } else {
                    format!("+{}xyz",self.xyz)
                };
                let sxyw = if self.xyw < <$t>::zero() {
                    format!("{}xyw",self.xyw)
                } else {
                    format!("+{}xyw",self.xyw)
                };
                let sxzw = if self.xzw < <$t>::zero() {
                    format!("{}xzw",self.xzw)
                } else {
                    format!("+{}xzw",self.xzw)
                };
                let syzw = if self.yzw < <$t>::zero() {
                    format!("{}yzw",self.yzw)
                } else {
                    format!("+{}yzw",self.yzw)
                };
                let sxyzw = if self.xyzw < <$t>::zero() {
                    format!("{}xyzw",self.xyzw)
                } else {
                    format!("+{}xyzw",self.xyzw)
                };
                write!(f,"{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                    self.r,
                    sx,sy,sz,sw,
                    sxy,sxz,sxw,syz,syw,szw,
                    sxyz,sxyw,sxzw,syzw,
                    sxyzw)
            }
        }            

        impl fmt::Debug for MultiVec4<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                let sx = if self.x < <$t>::zero() {
                    format!("{}x",self.x)
                } else {
                    format!("+{}x",self.x)
                };
                let sy = if self.y < <$t>::zero() {
                    format!("{}y",self.y)
                } else {
                    format!("+{}y",self.y)
                };
                let sz = if self.z < <$t>::zero() {
                    format!("{}z",self.z)
                } else {
                    format!("+{}z",self.z)
                };
                let sw = if self.w < <$t>::zero() {
                    format!("{}w",self.w)
                } else {
                    format!("+{}w",self.w)
                };
                let sxy = if self.xy < <$t>::zero() {
                    format!("{}xy",self.xy)
                } else {
                    format!("+{}xy",self.xy)
                };
                let sxz = if self.xz < <$t>::zero() {
                    format!("{}xz",self.xz)
                } else {
                    format!("+{}xz",self.xz)
                };
                let sxw = if self.xw < <$t>::zero() {
                    format!("{}xw",self.xw)
                } else {
                    format!("+{}xw",self.xw)
                };
                let syz = if self.yz < <$t>::zero() {
                    format!("{}yz",self.yz)
                } else {
                    format!("+{}yz",self.yz)
                };
                let syw = if self.yw < <$t>::zero() {
                    format!("{}yw",self.yw)
                } else {
                    format!("+{}yw",self.yw)
                };
                let szw = if self.zw < <$t>::zero() {
                    format!("{}zw",self.zw)
                } else {
                    format!("+{}zw",self.zw)
                };
                let sxyz = if self.xyz < <$t>::zero() {
                    format!("{}xyz",self.xyz)
                } else {
                    format!("+{}xyz",self.xyz)
                };
                let sxyw = if self.xyw < <$t>::zero() {
                    format!("{}xyw",self.xyw)
                } else {
                    format!("+{}xyw",self.xyw)
                };
                let sxzw = if self.xzw < <$t>::zero() {
                    format!("{}xzw",self.xzw)
                } else {
                    format!("+{}xzw",self.xzw)
                };
                let syzw = if self.yzw < <$t>::zero() {
                    format!("{}yzw",self.yzw)
                } else {
                    format!("+{}yzw",self.yzw)
                };
                let sxyzw = if self.xyzw < <$t>::zero() {
                    format!("{}xyzw",self.xyzw)
                } else {
                    format!("+{}xyzw",self.xyzw)
                };
                write!(f,"{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                    self.r,
                    sx,sy,sz,sw,
                    sxy,sxz,sxw,syz,syw,szw,
                    sxyz,sxyw,sxzw,syzw,
                    sxyzw)
            }
        }

        impl Zero for MultiVec4<$t> {
            fn zero() -> MultiVec4<$t> {
                MultiVec4 {
                    r: <$t>::zero(),
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                    w: <$t>::zero(),
                    xy: <$t>::zero(),
                    xz: <$t>::zero(),
                    xw: <$t>::zero(),
                    yz: <$t>::zero(),
                    yw: <$t>::zero(),
                    zw: <$t>::zero(),
                    xyz: <$t>::zero(),
                    xyw: <$t>::zero(),
                    xzw: <$t>::zero(),
                    yzw: <$t>::zero(),
                    xyzw: <$t>::zero(),
                }
            }
        }

        impl ops::Neg for MultiVec4<$t> {
            type Output = MultiVec4<$t>;
            fn neg(self) -> MultiVec4<$t> {
                MultiVec4 {
                    r: -self.r,
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    w: -self.w,
                    xy: -self.xy,
                    xz: -self.xz,
                    xw: -self.xw,
                    yz: -self.yz,
                    yw: -self.yw,
                    zw: -self.zw,
                    xyz: -self.xyz,
                    xyw: -self.xyw,
                    xzw: -self.xzw,
                    yzw: -self.yzw,
                    xyzw: -self.xyzw,
                }
            }
        }

        impl ops::Add<MultiVec4<$t>> for MultiVec4<$t> {
            type Output = MultiVec4<$t>;
            fn add(self,other: MultiVec4<$t>) -> Self::Output {
                MultiVec4 {
                    r: self.r + other.r,
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                    w: self.w + other.w,
                    xy: self.xy + other.xy,
                    xz: self.xz + other.xz,
                    xw: self.xw + other.xw,
                    yz: self.yz + other.yz,
                    yw: self.yw + other.yw,
                    zw: self.zw + other.zw,
                    xyz: self.xyz + other.xyz,
                    xyw: self.xyw + other.xyw,
                    xzw: self.xzw + other.xzw,
                    yzw: self.yzw + other.yzw,
                    xyzw: self.xyzw + other.xyzw,
                }
            }
        }

        impl ops::Sub<MultiVec4<$t>> for MultiVec4<$t> {
            type Output = MultiVec4<$t>;
            fn sub(self,other: MultiVec4<$t>) -> Self::Output {
                MultiVec4 {
                    r: self.r - other.r,
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                    w: self.w - other.w,
                    xy: self.xy - other.xy,
                    xz: self.xz - other.xz,
                    xw: self.xw - other.xw,
                    yz: self.yz - other.yz,
                    yw: self.yw - other.yw,
                    zw: self.zw - other.zw,
                    xyz: self.xyz - other.xyz,
                    xyw: self.xyw - other.xyw,
                    xzw: self.xzw - other.xzw,
                    yzw: self.yzw - other.yzw,
                    xyzw: self.xyzw - other.xyzw,
                }
            }
        }

        impl ops::AddAssign<MultiVec4<$t>> for MultiVec4<$t> {
            fn add_assign(&mut self,other: MultiVec4<$t>) {
                self.r += other.r;
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
                self.w += other.w;
                self.xy += other.xy;
                self.xz += other.xz;
                self.xw += other.xw;
                self.yz += other.yz;
                self.yw += other.yw;
                self.zw += other.zw;
                self.xyz += other.xyz;
                self.xyw += other.xyw;
                self.xzw += other.xzw;
                self.yzw += other.yzw;
                self.xyzw += other.xyzw;
            }
        }

        impl ops::SubAssign<MultiVec4<$t>> for MultiVec4<$t> {
            fn sub_assign(&mut self,other: MultiVec4<$t>) {
                self.r -= other.r;
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
                self.w -= other.w;
                self.xy -= other.xy;
                self.xz -= other.xz;
                self.xw -= other.xw;
                self.yz -= other.yz;
                self.yw -= other.yw;
                self.zw -= other.zw;
                self.xyz -= other.xyz;
                self.xyw -= other.xyw;
                self.xzw -= other.xzw;
                self.yzw -= other.yzw;
                self.xyzw -= other.xyzw;
            }
        }

        impl ops::Mul<$t> for MultiVec4<$t> {
            type Output = MultiVec4<$t>;
            fn mul(self,other: $t) -> Self::Output {
                MultiVec4 {
                    r: self.r * other,
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                    w: self.w * other,
                    xy: self.xy * other,
                    xz: self.xz * other,
                    xw: self.xw * other,
                    yz: self.yz * other,
                    yw: self.yw * other,
                    zw: self.zw * other,
                    xyz: self.xyz * other,
                    xyw: self.xyw * other,
                    xzw: self.xzw * other,
                    yzw: self.yzw * other,
                    xyzw: self.xyzw * other,
                }
            }
        }

        impl ops::Mul<MultiVec4<$t>> for $t {
            type Output = MultiVec4<$t>;
            fn mul(self,other: MultiVec4<$t>) -> Self::Output {
                MultiVec4 {
                    r: self * other.r,
                    x: self * other.x,
                    y: self * other.y,
                    z: self * other.z,
                    w: self * other.w,
                    xy: self * other.xy,
                    xz: self * other.xz,
                    xw: self * other.xw,
                    yz: self * other.yz,
                    yw: self * other.yw,
                    zw: self * other.zw,
                    xyz: self * other.xyz,
                    xyw: self * other.xyw,
                    xzw: self * other.xzw,
                    yzw: self * other.yzw,
                    xyzw: self * other.xyzw,
                }
            }
        }

        impl ops::Mul<MultiVec4<$t>> for MultiVec4<$t> {
            type Output = MultiVec4<$t>;
            fn mul(self,other: MultiVec4<$t>) -> Self::Output {
                MultiVec4 {
                    r: self.r * other.r + self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w - self.xy * other.xy - self.xz * other.xz - self.xw * other.xw - self.yz * other.yz - self.yw * other.yw - self.zw * other.zw - self.xyz * other.xyz - self.xyw * other.xyw - self.xzw * other.xzw - self.yzw * other.yzw + self.xyzw * other.xyzw,
                    x: self.r * other.x + self.x * other.r - self.y * other.xy - self.z * other.xz - self.w * other.xw + self.xy * other.y + self.xz * other.z + self.xw * other.w - self.yz * other.xyz - self.yw * other.xyw - self.zw * other.xzw - self.xyz * other.yz - self.xyw * other.yw - self.xzw * other.zw + self.yzw * other.xyzw - self.xyzw * other.yzw,
                    y: self.r * other.y + self.x * other.xy + self.y * other.r - self.z * other.yz - self.w * other.yw - self.xy * other.x + self.xz * other.xyz + self.xw * other.xyw + self.yz * other.z + self.yw * other.w - self.zw * other.yzw + self.xyz * other.xz + self.xyw * other.xw - self.xzw * other.xyzw - self.yzw * other.zw + self.xyzw * other.xzw,
                    z: self.r * other.z + self.x * other.xz + self.y * other.yz + self.z * other.r - self.w * other.zw - self.xy * other.xyz - self.xz * other.x + self.xw * other.xzw - self.yz * other.y + self.yw * other.yzw + self.zw * other.w - self.xyz * other.xy + self.xyw * other.xyzw + self.xzw * other.xw + self.yzw * other.yw - self.xyzw * other.xyw,
                    w: self.r * other.w + self.x * other.xw + self.y * other.yw + self.z * other.zw + self.w * other.r - self.xy * other.xyw - self.xz * other.xzw - self.xw * other.x - self.yz * other.yzw - self.yw * other.y - self.zw * other.z - self.xyz * other.xyzw - self.xyw * other.xy - self.xzw * other.xz - self.yzw * other.yz + self.xyzw * other.xyz,
                    xy: self.r * other.xy + self.x * other.y - self.y * other.x + self.z * other.xyz + self.w * other.xyw + self.xy * other.r - self.xz * other.yz - self.xw * other.yw + self.yz * other.xz + self.yw * other.xw - self.zw * other.xyzw + self.xyz * other.z + self.xyw * other.w - self.xzw * other.yzw + self.yzw * other.xzw - self.xyzw * other.zw,
                    xz: self.r * other.xz + self.x * other.z - self.y * other.xyz - self.z * other.x + self.w * other.xzw + self.xy * other.yz + self.xz * other.r - self.xw * other.zw - self.yz * other.xy + self.yw * other.xyzw + self.zw * other.xw - self.xyz * other.y + self.xyw * other.yzw + self.xzw * other.w - self.yzw * other.xyw + self.xyzw * other.yw,
                    xw: self.r * other.xw + self.x * other.w - self.y * other.xyw - self.z * other.xzw - self.w * other.x + self.xy * other.yw + self.xz * other.zw + self.xw * other.r - self.yz * other.xyzw - self.yw * other.xy - self.zw * other.xz - self.xyz * other.yzw - self.xyw * other.y - self.xzw * other.z + self.yzw * other.xyz - self.xyzw * other.yz,
                    yz: self.r * other.yz + self.x * other.xyz + self.y * other.z - self.z * other.y + self.w * other.yzw - self.xy * other.xz + self.xz * other.xy - self.xw * other.xyzw + self.yz * other.r - self.yw * other.zw + self.zw * other.yw + self.xyz * other.x - self.xyw * other.xzw + self.xzw * other.xyw + self.yzw * other.w - self.xyzw * other.xw,
                    yw: self.r * other.yw + self.x * other.xyw + self.y * other.w - self.z * other.yzw - self.w * other.y - self.xy * other.xw + self.xz * other.xyzw + self.xw * other.xy + self.yz * other.zw + self.yw * other.r - self.zw * other.yz + self.xyz * other.xzw + self.xyw * other.x - self.xzw * other.xyz - self.yzw * other.z + self.xyzw * other.xz,
                    zw: self.r * other.zw + self.x * other.xzw + self.y * other.yzw + self.z * other.w - self.w * other.z - self.xy * other.xyzw - self.xz * other.xw + self.xw * other.xz - self.yz * other.yw + self.yw * other.yz + self.zw * other.r - self.xyz * other.xyw + self.xyw * other.xyz + self.xzw * other.x + self.yzw * other.y - self.xyzw * other.xy,
                    xyz: self.r * other.xyz + self.x * other.yz - self.y * other.xz + self.z * other.xy - self.w * other.xyzw + self.xy * other.z - self.xz * other.y + self.xw * other.yzw + self.yz * other.x - self.yw * other.xzw + self.zw * other.xyw + self.xyz * other.r - self.xyw * other.zw + self.xzw * other.yw - self.yzw * other.xw + self.xyzw * other.w,
                    xyw: self.r * other.xyw + self.x * other.yw - self.y * other.xw + self.z * other.xyzw + self.w * other.xy + self.xy * other.w - self.xz * other.yzw - self.xw * other.y + self.yz * other.xzw + self.yw * other.x - self.zw * other.xyz + self.xyz * other.zw + self.xyw * other.r - self.xzw * other.yz + self.yzw * other.xz - self.xyzw * other.z,
                    xzw: self.r * other.xzw + self.x * other.zw - self.y * other.xyzw - self.z * other.xw + self.w * other.xz + self.xy * other.yzw + self.xz * other.w - self.xw * other.z - self.yz * other.xyw + self.yw * other.xyz + self.zw * other.x - self.xyz * other.yw + self.xyw * other.yz + self.xzw * other.r - self.yzw * other.xy + self.xyzw * other.y,
                    yzw: self.r * other.yzw + self.x * other.xyzw + self.y * other.zw - self.z * other.yw + self.w * other.yz - self.xy * other.xzw + self.xz * other.xyw - self.xw * other.xyz + self.yz * other.w - self.yw * other.z + self.zw * other.y + self.xyz * other.xw - self.xyw * other.xz + self.xzw * other.xy + self.yzw * other.r - self.xyzw * other.x,
                    xyzw: self.r * other.xyzw + self.x * other.yzw - self.y * other.xzw + self.z * other.xyw - self.w * other.xyz + self.xy * other.zw - self.xz * other.yw + self.xw * other.yz + self.yz * other.xw - self.yw * other.xz + self.zw * other.xy + self.xyz * other.w - self.xyw * other.z + self.xzw * other.y - self.yzw * other.x + self.xyzw * other.r,
                }
            }
        }

        impl ops::Div<$t> for MultiVec4<$t> {
            type Output = MultiVec4<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    MultiVec4 {
                        r: self.r / other,
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                        w: self.w / other,
                        xy: self.xy / other,
                        xz: self.xz / other,
                        xw: self.xw / other,
                        yz: self.yz / other,
                        yw: self.yw / other,
                        zw: self.zw / other,
                        xyz: self.xyz / other,
                        xyw: self.xyw / other,
                        xzw: self.xzw / other,
                        yzw: self.yzw / other,
                        xyzw: self.xyzw / other,
                    }
                }
                else {
                    self
                }
            }
        }

        impl ops::MulAssign<$t> for MultiVec4<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.r *= other;
                self.x *= other;
                self.y *= other;
                self.z *= other;
                self.w *= other;
                self.xy *= other;
                self.xz *= other;
                self.xw *= other;
                self.yz *= other;
                self.yw *= other;
                self.zw *= other;
                self.xyz *= other;
                self.xyw *= other;
                self.xzw *= other;
                self.yzw *= other;
                self.xyzw *= other;
            }
        }

        impl ops::MulAssign<MultiVec4<$t>> for MultiVec4<$t> {
            fn mul_assign(&mut self,other: MultiVec4<$t>) {
                let nr = self.r * other.r + self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w - self.xy * other.xy - self.xz * other.xz - self.xw * other.xw - self.yz * other.yz - self.yw * other.yw - self.zw * other.zw - self.xyz * other.xyz - self.xyw * other.xyw - self.xzw * other.xzw - self.yzw * other.yzw + self.xyzw * other.xyzw;
                let nx = self.r * other.x + self.x * other.r - self.y * other.xy - self.z * other.xz - self.w * other.xw + self.xy * other.y + self.xz * other.z + self.xw * other.w - self.yz * other.xyz - self.yw * other.xyw - self.zw * other.xzw - self.xyz * other.yz - self.xyw * other.yw - self.xzw * other.zw + self.yzw * other.xyzw - self.xyzw * other.yzw;
                let ny = self.r * other.y + self.x * other.xy + self.y * other.r - self.z * other.yz - self.w * other.yw - self.xy * other.x + self.xz * other.xyz + self.xw * other.xyw + self.yz * other.z + self.yw * other.w - self.zw * other.yzw + self.xyz * other.xz + self.xyw * other.xw - self.xzw * other.xyzw - self.yzw * other.zw + self.xyzw * other.xzw;
                let nz = self.r * other.z + self.x * other.xz + self.y * other.yz + self.z * other.r - self.w * other.zw - self.xy * other.xyz - self.xz * other.x + self.xw * other.xzw - self.yz * other.y + self.yw * other.yzw + self.zw * other.w - self.xyz * other.xy + self.xyw * other.xyzw + self.xzw * other.xw + self.yzw * other.yw - self.xyzw * other.xyw;
                let nw = self.r * other.w + self.x * other.xw + self.y * other.yw + self.z * other.zw + self.w * other.r - self.xy * other.xyw - self.xz * other.xzw - self.xw * other.x - self.yz * other.yzw - self.yw * other.y - self.zw * other.z - self.xyz * other.xyzw - self.xyw * other.xy - self.xzw * other.xz - self.yzw * other.yz + self.xyzw * other.xyz;
                let nxy = self.r * other.xy + self.x * other.y - self.y * other.x + self.z * other.xyz + self.w * other.xyw + self.xy * other.r - self.xz * other.yz - self.xw * other.yw + self.yz * other.xz + self.yw * other.xw - self.zw * other.xyzw + self.xyz * other.z + self.xyw * other.w - self.xzw * other.yzw + self.yzw * other.xzw - self.xyzw * other.zw;
                let nxz = self.r * other.xz + self.x * other.z - self.y * other.xyz - self.z * other.x + self.w * other.xzw + self.xy * other.yz + self.xz * other.r - self.xw * other.zw - self.yz * other.xy + self.yw * other.xyzw + self.zw * other.xw - self.xyz * other.y + self.xyw * other.yzw + self.xzw * other.w - self.yzw * other.xyw + self.xyzw * other.yw;
                let nxw = self.r * other.xw + self.x * other.w - self.y * other.xyw - self.z * other.xzw - self.w * other.x + self.xy * other.yw + self.xz * other.zw + self.xw * other.r - self.yz * other.xyzw - self.yw * other.xy - self.zw * other.xz - self.xyz * other.yzw - self.xyw * other.y - self.xzw * other.z + self.yzw * other.xyz - self.xyzw * other.yz;
                let nyz = self.r * other.yz + self.x * other.xyz + self.y * other.z - self.z * other.y + self.w * other.yzw - self.xy * other.xz + self.xz * other.xy - self.xw * other.xyzw + self.yz * other.r - self.yw * other.zw + self.zw * other.yw + self.xyz * other.x - self.xyw * other.xzw + self.xzw * other.xyw + self.yzw * other.w - self.xyzw * other.xw;
                let nyw = self.r * other.yw + self.x * other.xyw + self.y * other.w - self.z * other.yzw - self.w * other.y - self.xy * other.xw + self.xz * other.xyzw + self.xw * other.xy + self.yz * other.zw + self.yw * other.r - self.zw * other.yz + self.xyz * other.xzw + self.xyw * other.x - self.xzw * other.xyz - self.yzw * other.z + self.xyzw * other.xz;
                let nzw = self.r * other.zw + self.x * other.xzw + self.y * other.yzw + self.z * other.w - self.w * other.z - self.xy * other.xyzw - self.xz * other.xw + self.xw * other.xz - self.yz * other.yw + self.yw * other.yz + self.zw * other.r - self.xyz * other.xyw + self.xyw * other.xyz + self.xzw * other.x + self.yzw * other.y - self.xyzw * other.xy;
                let nxyz = self.r * other.xyz + self.x * other.yz - self.y * other.xz + self.z * other.xy - self.w * other.xyzw + self.xy * other.z - self.xz * other.y + self.xw * other.yzw + self.yz * other.x - self.yw * other.xzw + self.zw * other.xyw + self.xyz * other.r - self.xyw * other.zw + self.xzw * other.yw - self.yzw * other.xw + self.xyzw * other.w;
                let nxyw = self.r * other.xyw + self.x * other.yw - self.y * other.xw + self.z * other.xyzw + self.w * other.xy + self.xy * other.w - self.xz * other.yzw - self.xw * other.y + self.yz * other.xzw + self.yw * other.x - self.zw * other.xyz + self.xyz * other.zw + self.xyw * other.r - self.xzw * other.yz + self.yzw * other.xz - self.xyzw * other.z;
                let nxzw = self.r * other.xzw + self.x * other.zw - self.y * other.xyzw - self.z * other.xw + self.w * other.xz + self.xy * other.yzw + self.xz * other.w - self.xw * other.z - self.yz * other.xyw + self.yw * other.xyz + self.zw * other.x - self.xyz * other.yw + self.xyw * other.yz + self.xzw * other.r - self.yzw * other.xy + self.xyzw * other.y;
                let nyzw = self.r * other.yzw + self.x * other.xyzw + self.y * other.zw - self.z * other.yw + self.w * other.yz - self.xy * other.xzw + self.xz * other.xyw - self.xw * other.xyz + self.yz * other.w - self.yw * other.z + self.zw * other.y + self.xyz * other.xw - self.xyw * other.xz + self.xzw * other.xy + self.yzw * other.r - self.xyzw * other.x;
                let nxyzw = self.r * other.xyzw + self.x * other.yzw - self.y * other.xzw + self.z * other.xyw - self.w * other.xyz + self.xy * other.zw - self.xz * other.yw + self.xw * other.yz + self.yz * other.xw - self.yw * other.xz + self.zw * other.xy + self.xyz * other.w - self.xyw * other.z + self.xzw * other.y - self.yzw * other.x + self.xyzw * other.r;
                self.r = nr;
                self.x = nx;
                self.y = ny;
                self.z = nz;
                self.w = nw;
                self.xy = nxy;
                self.xz = nxz;
                self.xw = nxw;
                self.yz = nyz;
                self.yw = nyw;
                self.zw = nzw;
                self.xyz = nxyz;
                self.xyw = nxyw;
                self.xzw = nxzw;
                self.yzw = nyzw;
                self.xyzw = nxyzw;
            }
        }

        impl ops::DivAssign<$t> for MultiVec4<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.r /= other;
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                    self.w /= other;
                    self.xy /= other;
                    self.xz /= other;
                    self.xw /= other;
                    self.yz /= other;
                    self.yw /= other;
                    self.zw /= other;
                    self.xyz /= other;
                    self.xyw /= other;
                    self.xzw /= other;
                    self.yzw /= other;
                    self.xyzw /= other;
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32m4 = MultiVec4<f32>;

impl_multivec4!(f32);

#[allow(non_camel_case_types)]
pub type f64m4 = MultiVec4<f64>;

impl_multivec4!(f64);