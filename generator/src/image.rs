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

    /// Crop new image. And for some reason this needs to be inverted vertically. Probably because of the texture setting to framebuffer.
    pub fn crop_upside_down(&self,r: usizer) -> Image<T> {
        let mut image = Image::<T>::new(r.s);
        for y in 0..r.s.y {
            for x in 0..r.s.x {
                *image.pixel_mut(usizev2 { x: x,y: r.s.y - y - 1, }) = *self.pixel(usizev2 { x: r.o.x + x,y: r.o.y + y, });
            }
        }
        image
    }
}
