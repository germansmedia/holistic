// Social Robotics Platform 04
// Desmond Germans, Ph.D
// Quaternion

use std::{cmp,fmt};
use crate::*;

#[derive(Clone)]
pub struct Quaternion<T> {
    pub r: T,
    pub i: T,
    pub j: T,
    pub k: T,
}

#[allow(non_camel_case_types)]
pub type f32q = Quaternion<f32>;

#[allow(non_camel_case_types)]
pub type f64q = Quaternion<f64>;

impl<T> Quaternion<T> {
    pub fn new(r: T,i: T,j: T,k: T) -> Quaternion<T> {
        Quaternion {
            r: r,
            i: i,
            j: j,
            k: k,
        }
    }
}

impl<T: cmp::PartialEq> cmp::PartialEq for Quaternion<T> {
    fn eq(&self,other: &Quaternion<T>) -> bool {
        (self.r == other.r)
        && (self.i == other.i)
        && (self.j == other.j)
        && (self.k == other.k)
    }
}

impl<T: fmt::Display + cmp::PartialOrd + Zero> fmt::Display for Quaternion<T> {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        let si = if self.i < T::zero() {
            format!("{}i",self.i)
        } else {
            format!("+{}i",self.i)
        };
        let sj = if self.j < T::zero() {
            format!("{}j",self.j)
        } else {
            format!("+{}j",self.j)
        };
        let sk = if self.k < T::zero() {
            format!("{}k",self.k)
        } else {
            format!("+{}k",self.k)
        };
        write!(f,"{}{}{}{}",self.r,si,sj,sk)
    }
}

impl<T: fmt::Display + cmp::PartialOrd + Zero> fmt::Debug for Quaternion<T> {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        let si = if self.i < T::zero() {
            format!("{}i",self.i)
        } else {
            format!("+{}i",self.i)
        };
        let sj = if self.j < T::zero() {
            format!("{}j",self.j)
        } else {
            format!("+{}j",self.j)
        };
        let sk = if self.k < T::zero() {
            format!("{}k",self.k)
        } else {
            format!("+{}k",self.k)
        };
        write!(f,"{}{}{}{}",self.r,si,sj,sk)
    }
}

impl<T: Zero> Zero for Quaternion<T> {
    fn zero() -> Quaternion<T> {
        Quaternion {
            r: T::zero(),
            i: T::zero(),
            j: T::zero(),
            k: T::zero(),
        }
    }
}
