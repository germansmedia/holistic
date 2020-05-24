// Social Robotics Platform 04
// Desmond Germans, Ph.D
// Image<T>

use crate::math::*;
use crate::*;

/// 2D Image of pixels.
pub struct Image<T> {
    size: usizev2,
    data: Box<[T]>,
}

impl<T: Pixel> Image<T> {
    /// Create 2D image.
    /// # Arguments
    /// * `size`: Size of new 2D image.
    /// # Returns
    /// New black image of given size.
    pub fn new(size: usizev2) -> Image<T> {
        let data: Box<[T]> = vec![T::new_rgb(0,0,0); size.x * size.y].into_boxed_slice();
        Image {
            size: size,
            data: data,
        }
    }

    /// Size accessor.
    /// # Arguments
    /// * `self`: Image to get size from.
    /// # Returns
    /// Size of the image.
    pub fn size(&self) -> &usizev2 {
        &self.size
    }

    /// Direct data accessor.
    /// # Arguments
    /// * `self`: Image to access data on.
    /// # Returns
    /// Accessor to the data.
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Mutable direct data accessor.
    /// # Arguments
    /// * `self`: Image to access data on.
    /// # Returns
    /// Accessor to the data.
    pub fn data_mut(&mut self) -> &[T] {
        &mut self.data
    }

    /// Pixel accessor.
    /// # Arguments
    /// * `self`: Image to access pixel on.
    /// * `p`: Position of the pixel.
    /// # Returns
    /// Accessor to the indicated pixel.
    pub fn pixel(&self,p: usizev2) -> &T {
        &self.data[p.y * self.size.x + p.x]
    }

    /// Mutable pixel accessor.
    /// # Arguments
    /// * `self`: Image to access pixel on.
    /// * `p`: Position of the pixel.
    /// # Returns
    /// Mutable accessor to the indicated pixel.
    pub fn pixel_mut(&mut self,p: usizev2) -> &mut T {
        &mut self.data[p.y * self.size.x + p.x]
    }
}
