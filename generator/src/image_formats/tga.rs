// Social Robotics Platform 03
// Desmond Germans, Ph.D
// image formats: TGA

use crate::*;

/// Test if a slice is a TGA image.
/// # Details
/// `tga::test` checks if the given slice is a TGA image, and return the dimensions of the image.
/// # Arguments
/// * `src`: Slice to check.
/// # Returns
/// * `Some(size)`: The slice is a TGA image, dimensions are given in `size`.
/// * `None`: The slice is not a TGA image.
/// # Example
/// ```
/// if let Some(size) = tga::test(&buffer) {
///     println!("This is a TGA image of {}x{} pixels.",size.x,size.y);
/// }
/// ```
/// > Unfinished.
pub fn test(_src: &[u8]) -> Option<usizev2> {
	None
}

/// Decode a TGA image.
/// # Details
/// `tga::decode<T>` decodes a TGA-encoded slice into pixels of type `T`.
/// # Arguments
/// * `src`: Encoded slice.
/// # Returns
/// * `Ok(image)`: The image was decoded succesfully into `image`.
/// * `Err(text)`: An error occurred.
/// # Example
/// ```
/// if let Ok(image) = tga::decode::<RGBA8>(&buffer) {
///     // do something with the image
/// }
/// ```
/// > TODO, development comments:
/// > Don't use strings for error returns.
/// > Unfinished.
pub fn decode<T: Pixel>(_src: &[u8]) -> Result<Image<T>,String> {
	Err("not implemented yet".to_string())
}

/// Encode a TGA image.
/// # Details
/// `tga::encode<T>` encodes pixels of type `T` into a TGA image.
/// # Arguments
/// * `image`: The pixels to be encoded.
/// # Returns
/// * `Ok(Vec<u8>)`: The encoded slice.
/// * `Err(text)`: An error occurred.
/// # Example
/// ```
/// let image = Image<RGBA>::...;
/// if let Ok(buffer) = tga::encode(&image) {
///     // do something with the image
/// }
/// ```
/// > TODO, development comments:
/// > Don't use strings for error returns.
/// > Unfinished.
#[allow(dead_code)]
pub fn encode<T: Pixel>(_image: &Image<T>) -> Result<Vec<u8>,String> {
	Err("not implemented yet".to_string())
}
