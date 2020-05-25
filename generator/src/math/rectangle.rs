// Social Robotics Platform 04
// Desmond Germans, Ph.D
// Rectangle

use std::{cmp,fmt};
use crate::*;

#[derive(Clone)]
pub struct Rectangle<T> {
    pub o: Vec2<T>,
    pub s: Vec2<T>,
}

#[allow(non_camel_case_types)]
pub type u16r = Rectangle<u16>;

#[allow(non_camel_case_types)]
pub type i16r = Rectangle<i16>;

#[allow(non_camel_case_types)]
pub type u32r = Rectangle<u32>;

#[allow(non_camel_case_types)]
pub type i32r = Rectangle<i32>;

#[allow(non_camel_case_types)]
pub type u64r = Rectangle<u64>;

#[allow(non_camel_case_types)]
pub type i64r = Rectangle<i64>;

#[allow(non_camel_case_types)]
pub type usizer = Rectangle<usize>;

#[allow(non_camel_case_types)]
pub type isizer = Rectangle<isize>;

#[allow(non_camel_case_types)]
pub type f32r = Rectangle<f32>;

#[allow(non_camel_case_types)]
pub type f64r = Rectangle<f64>;

impl<T> Rectangle<T> {
    pub fn new(o: Vec2<T>,s: Vec2<T>) -> Rectangle<T> {
        Rectangle {
            o: o,
            s: s,
        }
    }
}

impl<T: cmp::PartialEq> cmp::PartialEq for Rectangle<T> where Vec2<T>: cmp::PartialEq {
    fn eq(&self,other: &Rectangle<T>) -> bool {
        (self.o == other.o)
        && (self.s == other.s)
    }
}

impl<T: fmt::Display> fmt::Display for Rectangle<T> where Vec2<T>: fmt::Display {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"({},{}; {}x{})",self.o.x,self.o.y,self.s.x,self.s.y)
    }
}

impl<T: fmt::Display> fmt::Debug for Rectangle<T> where Vec2<T>: fmt::Display {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"({},{}; {}x{})",self.o.x,self.o.y,self.s.x,self.s.y)
    }
}

/*impl<T> Zero for Rectangle<T> {
    fn zero() -> Rectangle<T> {
        Rectangle {
            o: Vec2::<T>::zero(),
            s: Vec2::<T>::zero(),
        }
    }
}*/