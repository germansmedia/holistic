// Social Robotics Platform 04
// Desmond Germans, Ph.D
// image formats

pub mod bmp;
pub mod png;
pub mod jpeg;
pub mod gif;
pub mod tga;
pub mod tiff;
pub mod pbm;
pub mod xbm;
pub mod webp;

use crate::*;

/// Test if a slice can be decoded by any of the supported image formats.
/// # Details
/// `test` checks if the given slice is any of the supported image formats, and return the dimensions of the image.
/// 
/// The currently supported formats are:
/// 
/// format | decoding | encoding
/// -------+----------+-------------
/// BMP    | yes      | very limited
/// GIF    | TODO     | TODO
/// JPEG   | yes      | TODO
/// PBM    | TODO     | TODO
/// PNG    | yes      | TODO
/// TGA    | TODO     | TODO
/// TIFF   | TODO     | TODO
/// WEBP   | TODO     | TODO
/// XBM    | TODO     | TODO
/// # Arguments
/// * `src`: Slice to check.
/// # Returns
/// * `Some(size)`: The slice can be decoded, dimensions are given in `size`.
/// * `None`: The slice can not be decoded.
/// # Example
/// ```
/// if let Some(size) = test(&buffer) {
///     println!("This is a supported image of {}x{} pixels.",size.x,size.y);
/// }
/// ```
#[allow(dead_code)]
pub fn test(src: &[u8]) -> Option<usizev2> {
    if let Some(size) = bmp::test(src) {
        Some(size)
    }
    else if let Some(size) = png::test(src) {
        Some(size)
    }
    else if let Some(size) = jpeg::test(src) {
        Some(size)
    }
    else if let Some(size) = gif::test(src) {
        Some(size)
    }
    else if let Some(size) = tga::test(src) {
        Some(size)
    }
    else if let Some(size) = tiff::test(src) {
        Some(size)
    }
    else if let Some(size) = pbm::test(src) {
        Some(size)
    }
    else if let Some(size) = xbm::test(src) {
        Some(size)
    }
    else if let Some(size) = webp::test(src) {
        Some(size)
    }
    else {
        None
    }
}

/// Decode an image.
/// # Details
/// `decode<T>` decodes a slice into pixels of type `T`.
/// # Arguments
/// * `src`: Encoded slice.
/// # Returns
/// * `Ok(image)`: The image was decoded succesfully into `image`.
/// * `Err(text)`: An error occurred.
/// # Example
/// ```
/// if let Ok(image) = decode::<RGBA8>(&buffer) {
///     // do something with the image
/// }
/// ```
/// > TODO, development comments:
/// > Don't use strings for error returns.
#[allow(dead_code)]
pub fn decode<T: Pixel>(src: &[u8]) -> Result<Image<T>,String> {
    if let Ok(image) = bmp::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = png::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = jpeg::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = gif::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = tga::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = tiff::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = pbm::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = xbm::decode(src) {
        Ok(image)
    }
    else if let Ok(image) = webp::decode(src) {
        Ok(image)
    }
    else {
        Err("image format not supported".to_string())
    }
}
