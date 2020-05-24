// Social Robotics Platform 03
// Desmond Germans, Ph.D
// Pixel

/// Pixel with 24-bit color and 8-bit alpha.
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct RGBA8 {
    pub d: u32,
}

/// Pixel with 24-bit color and 8-bit alpha (OS friendly).
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct ARGB8 {
    pub d: u32,
}

/// Pixel with 24-bit color.
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct RGB8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Pixel with 16-bit color.
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct R5G6B5 {
    pub d: u16,
}

/// Pixel with 15-bit color and 1-bit alpha.
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct RGB5A1 {
    pub d: u16,
}

/// Pixel with 30-bit color and 2-bit alpha.
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct RGB10A2 {
    pub d: u32,
}

/// Pixel trait.
pub trait Pixel: Clone + Copy {
    /// Pixel from RGB bytes.
    /// # Arguments
    /// * `r`: Red byte value.
    /// * `g`: Green byte value.
    /// * `b`: Blue byte value.
    /// # Returns
    /// Encoded pixel.
    fn new_rgb(r: u8,g: u8,b: u8) -> Self;

    /// Pixel from RGBA bytes.
    /// # Arguments
    /// * `r`: Red byte value.
    /// * `g`: Green byte value.
    /// * `b`: Blue byte value.
    /// * `a`: Alpha byte value.
    /// # Returns
    /// Encoded pixel.
    fn new_rgba(r: u8,g: u8,b: u8,a: u8) -> Self;

    /// Set red byte.
    /// # Arguments
    /// * `self`: Pixel to set red value of.
    /// * `r`: Red byte value.
    fn set_r(&mut self,r: u8);

    /// Set green byte.
    /// # Arguments
    /// * `self`: Pixel to set green value of.
    /// * `g`: Green byte value.
    fn set_g(&mut self,g: u8);

    /// Set blue byte.
    /// # Arguments
    /// * `self`: Pixel to set blue value of.
    /// * `b`: Blue byte value.
    fn set_b(&mut self,b: u8);

    /// Set alpha byte.
    /// # Arguments
    /// * `self`: Pixel to set alpha value of.
    /// * `a`: Alpha byte value.
    fn set_a(&mut self,a: u8);

    /// Set RGB bytes.
    /// # Arguments
    /// * `self`: Pixel to set.
    /// * `r`: Red byte value.
    /// * `g`: Green byte value.
    /// * `b`: Blue byte value.
    fn set_rgb(&mut self,r: u8,g: u8,b: u8);

    /// Set RGBA bytes.
    /// # Arguments
    /// * `self`: Pixel to set.
    /// * `r`: Red byte value.
    /// * `g`: Green byte value.
    /// * `b`: Blue byte value.
    /// * `a`: Alpha byte value.
    fn set_rgba(&mut self,r: u8,g: u8,b: u8,a: u8);

    /// Get red byte.
    /// # Arguments
    /// * `self`: Pixel to get red byte from.
    /// # Returns
    /// Red byte value of the pixel.
    fn r(&self) -> u8;

    /// Get green byte.
    /// # Arguments
    /// * `self`: Pixel to get green byte from.
    /// # Returns
    /// Green byte value of the pixel.
    fn g(&self) -> u8;

    /// Get blue byte.
    /// # Arguments
    /// * `self`: Pixel to get blue byte from.
    /// # Returns
    /// Blue byte value of the pixel.
    fn b(&self) -> u8;

    /// Get red byte.
    /// # Arguments
    /// * `self`: Pixel to get alpha byte from.
    /// # Returns
    /// Alpha byte value of the pixel.
    fn a(&self) -> u8;

    /// Get RGB bytes.
    /// # Arguments
    /// * `self`: Pixel to get RGB bytes from.
    /// * `r`: Receives red byte.
    /// * `g`: Receives green byte.
    /// * `b`: Receives blue byte.
    fn rgb(&self,r: &mut u8,g: &mut u8,b: &mut u8);

    /// Get RGBA bytes.
    /// # Arguments
    /// * `self`: Pixel to get RGBA bytes from.
    /// * `r`: Receives red byte.
    /// * `g`: Receives green byte.
    /// * `b`: Receives blue byte.
    /// * `a`: Receives alpha byte.
    fn rgba(&self,r: &mut u8,g: &mut u8,b: &mut u8,a: &mut u8);

    // TODO: rgbf, rgbaf, new_rgbf, new_rgbaf, etc.
}

impl Pixel for RGBA8 {

    fn new_rgb(r: u8,g: u8,b: u8) -> RGBA8 {
        let pr = r as u32;
        let pg = g as u32;
        let pb = b as u32;
        RGBA8 { d: (pr << 24) | (pg << 16) | (pb << 8) | 0x000000FF, }
    }

    fn new_rgba(r: u8,g: u8,b: u8,a: u8) -> RGBA8 {
        let pr = r as u32;
        let pg = g as u32;
        let pb = b as u32;
        let pa = a as u32;
        RGBA8 { d: (pr << 24) | (pg << 16) | (pb << 8) | pa, }
    }

    fn set_r(&mut self,r: u8) {
        let pr = r as u32;
        self.d = (self.d & 0x00FFFFFF) | (pr << 24);
    }

    fn set_g(&mut self,g: u8) {
        let pg = g as u32;
        self.d = (self.d & 0xFF00FFFF) | (pg << 16);
    }

    fn set_b(&mut self,b: u8) {
        let pb = b as u32;
        self.d = (self.d & 0xFFFF00FF) | (pb << 8);
    }

    fn set_a(&mut self,a: u8) {
        let pa = a as u32;
        self.d = (self.d & 0xFFFFFF00) | pa;
    }

    fn set_rgb(&mut self,r: u8,g: u8,b: u8) {
        let pr = r as u32;
        let pg = g as u32;
        let pb = b as u32;
        self.d = (self.d & 0x000000FF) | (pr << 24) | (pg << 16) | (pb << 8);
    }

    fn set_rgba(&mut self,r: u8,g: u8,b: u8,a: u8) {
        let pr = r as u32;
        let pg = g as u32;
        let pb = b as u32;
        let pa = a as u32;
        self.d = (pr << 24) | (pg << 16) | (pb << 8) | pa;
    }

    fn r(&self) -> u8 {
        (self.d >> 24) as u8
    }

    fn g(&self) -> u8 {
        ((self.d & 0x00FF0000) >> 16) as u8
    }

    fn b(&self) -> u8 {
        ((self.d & 0x0000FF00) >> 8) as u8
    }

    fn a(&self) -> u8 {
        (self.d & 0x000000FF) as u8
    }

    fn rgb(&self,r: &mut u8,g: &mut u8,b: &mut u8) {
        *r = (self.d >> 24) as u8;
        *g = ((self.d & 0x00FF0000) >> 16) as u8;
        *b = ((self.d & 0x0000FF00) >> 8 ) as u8;
    }

    fn rgba(&self,r: &mut u8,g: &mut u8,b: &mut u8,a: &mut u8) {
        *r = (self.d >> 24) as u8;
        *g = ((self.d & 0x00FF0000) >> 16) as u8;
        *b = ((self.d & 0x0000FF00) >> 8 ) as u8;
        *a = (self.d & 0x000000FF) as u8;
    }
}

impl PartialEq<RGBA8> for RGBA8 {
    fn eq(&self,other: &RGBA8) -> bool {
        self.d == other.d
    }
}

impl Pixel for ARGB8 {

    fn new_rgb(r: u8,g: u8,b: u8) -> ARGB8 {
        let pr = r as u32;
        let pg = g as u32;
        let pb = b as u32;
        ARGB8 { d: 0xFF000000 | (pr << 16) | (pg << 8) | pb, }
    }

    fn new_rgba(r: u8,g: u8,b: u8,a: u8) -> ARGB8 {
        let pr = r as u32;
        let pg = g as u32;
        let pb = b as u32;
        let pa = a as u32;
        ARGB8 { d: (pa << 24) | (pr << 16) | (pg << 8) | pb, }
    }

    fn set_r(&mut self,r: u8) {
        let pr = r as u32;
        self.d = (self.d & 0xFF00FFFF) | (pr << 16);
    }

    fn set_g(&mut self,g: u8) {
        let pg = g as u32;
        self.d = (self.d & 0xFFFF00FF) | (pg << 8);
    }

    fn set_b(&mut self,b: u8) {
        let pb = b as u32;
        self.d = (self.d & 0xFFFFFF00) | pb;
    }

    fn set_a(&mut self,a: u8) {
        let pa = a as u32;
        self.d = (self.d & 0x00FFFFFF) | (pa << 24);
    }

    fn set_rgb(&mut self,r: u8,g: u8,b: u8) {
        let pr = r as u32;
        let pg = g as u32;
        let pb = b as u32;
        self.d = (self.d & 0xFF000000) | (pr << 16) | (pg << 8) | pb;
    }

    fn set_rgba(&mut self,r: u8,g: u8,b: u8,a: u8) {
        let pr = r as u32;
        let pg = g as u32;
        let pb = b as u32;
        let pa = a as u32;
        self.d = (pa << 24) | (pr << 16) | (pg << 8) | pb;
    }

    fn r(&self) -> u8 {
        ((self.d & 0x00FF0000) >> 16) as u8
    }

    fn g(&self) -> u8 {
        ((self.d & 0x0000FF00) >> 8) as u8
    }

    fn b(&self) -> u8 {
        (self.d & 0x000000FF) as u8
    }

    fn a(&self) -> u8 {
        (self.d >> 24) as u8
    }

    fn rgb(&self,r: &mut u8,g: &mut u8,b: &mut u8) {
        *r = ((self.d & 0x00FF0000) >> 16) as u8;
        *g = ((self.d & 0x0000FF00) >> 8) as u8;
        *b = (self.d & 0x000000FF) as u8;
    }

    fn rgba(&self,r: &mut u8,g: &mut u8,b: &mut u8,a: &mut u8) {
        *r = ((self.d & 0x00FF0000) >> 16) as u8;
        *g = ((self.d & 0x0000FF00) >> 8) as u8;
        *b = (self.d & 0x000000FF) as u8;
        *a = (self.d >> 24) as u8;
    }
}

impl PartialEq<ARGB8> for ARGB8 {
    fn eq(&self,other: &ARGB8) -> bool {
        self.d == other.d
    }
}

impl Pixel for RGB8 {

    fn new_rgb(r: u8,g: u8,b: u8) -> RGB8 {
        RGB8 { r: r,g: g,b: b, }
    }

    fn new_rgba(r: u8,g: u8,b: u8,_a: u8) -> RGB8 {
        RGB8 { r: r,g: g,b: b, }
    }

    fn set_r(&mut self,r: u8) {
        self.r = r;
    }

    fn set_g(&mut self,g: u8) {
        self.g = g;
    }

    fn set_b(&mut self,b: u8) {
        self.b = b;
    }

    fn set_a(&mut self,_a: u8) {
    }

    fn set_rgb(&mut self,r: u8,g: u8,b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }

    fn set_rgba(&mut self,r: u8,g: u8,b: u8,_a: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }

    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }

    fn a(&self) -> u8 {
        0xFF
    }

    fn rgb(&self,r: &mut u8,g: &mut u8,b: &mut u8) {
        *r = self.r;
        *g = self.g;
        *b = self.b;
    }

    fn rgba(&self,r: &mut u8,g: &mut u8,b: &mut u8,a: &mut u8) {
        *r = self.r;
        *g = self.g;
        *b = self.b;
        *a = 0xFF;
    }
}

impl PartialEq<RGB8> for RGB8 {
    fn eq(&self,other: &RGB8) -> bool {
        (self.r == other.r) && (self.g == other.g) && (self.b == other.b)
    }
}

impl Pixel for R5G6B5 {

    fn new_rgb(r: u8,g: u8,b: u8) -> R5G6B5 {
        let pr = (r as u16) >> 3;
        let pg = (g as u16) >> 2;
        let pb = (b as u16) >> 3;
        R5G6B5 { d: (pr << 11) | (pg << 5) | pb, }
    }

    fn new_rgba(r: u8,g: u8,b: u8,_a: u8) -> R5G6B5 {
        let pr = (r as u16) >> 3;
        let pg = (g as u16) >> 2;
        let pb = (b as u16) >> 3;
        R5G6B5 { d: (pr << 11) | (pg << 5) | pb, }
    }

    fn set_r(&mut self,r: u8) {
        let pr = (r as u16) >> 3;
        self.d = (self.d & 0x07FF) | (pr << 11);
    }

    fn set_g(&mut self,g: u8) {
        let pg = (g as u16) >> 2;
        self.d = (self.d & 0xF81F) | (pg << 5);
    }

    fn set_b(&mut self,b: u8) {
        let pb = (b as u16) >> 3;
        self.d = (self.d & 0xFFE0) | pb;
    }

    fn set_a(&mut self,_a: u8) {
    }

    fn set_rgb(&mut self,r: u8,g: u8,b: u8) {
        let pr = (r as u16) >> 3;
        let pg = (g as u16) >> 2;
        let pb = (b as u16) >> 3;
        self.d = (pr << 11) | (pg << 5) | pb;
    }

    fn set_rgba(&mut self,r: u8,g: u8,b: u8,_a: u8) {
        let pr = (r as u16) >> 3;
        let pg = (g as u16) >> 2;
        let pb = (b as u16) >> 3;
        self.d = (pr << 11) | (pg << 5) | pb;
    }

    fn r(&self) -> u8 {
        let pr = ((self.d >> 11) & 0x001F) as u8;
        (pr << 3) | (pr >> 2)
    }

    fn g(&self) -> u8 {
        let pg = ((self.d >> 5) & 0x003F) as u8;
        (pg << 2) | (pg >> 4)
    }

    fn b(&self) -> u8 {
        let pb = (self.d & 0x001F) as u8;
        (pb << 3) | (pb >> 2)
    }

    fn a(&self) -> u8 {
        0xFF
    }

    fn rgb(&self,r: &mut u8,g: &mut u8,b: &mut u8) {
        let pr = ((self.d >> 11) & 0x001F) as u8;
        *r = (pr << 3) | (pr >> 2);
        let pg = ((self.d >> 5) & 0x003F) as u8;
        *g = (pg << 2) | (pg >> 4);
        let pb = (self.d & 0x001F) as u8;
        *b = (pb << 3) | (pb >> 2);
    }

    fn rgba(&self,r: &mut u8,g: &mut u8,b: &mut u8,_a: &mut u8) {
        let pr = ((self.d >> 11) & 0x001F) as u8;
        *r = (pr << 3) | (pr >> 2);
        let pg = ((self.d >> 5) & 0x003F) as u8;
        *g = (pg << 2) | (pg >> 4);
        let pb = (self.d & 0x001F) as u8;
        *b = (pb << 3) | (pb >> 2);
    }
}

impl PartialEq<R5G6B5> for R5G6B5 {
    fn eq(&self,other: &R5G6B5) -> bool {
        self.d == other.d
    }
}

impl Pixel for RGB5A1 {

    fn new_rgb(r: u8,g: u8,b: u8) -> RGB5A1 {
        let pr = (r as u16) >> 3;
        let pg = (g as u16) >> 3;
        let pb = (b as u16) >> 3;
        RGB5A1 { d: (pr << 11) | (pg << 6) | (pb << 1) | 0x0001, }
    }

    fn new_rgba(r: u8,g: u8,b: u8,a: u8) -> RGB5A1 {
        let pr = (r as u16) >> 3;
        let pg = (g as u16) >> 3;
        let pb = (b as u16) >> 3;
        let pa = (a as u16) >> 7;
        RGB5A1 { d: (pr << 11) | (pg << 6) | (pb << 1) | pa, }
    }

    fn set_r(&mut self,r: u8) {
        let pr = (r as u16) >> 3;
        self.d = (self.d & 0x07FF) | (pr << 11);
    }

    fn set_g(&mut self,g: u8) {
        let pg = (g as u16) >> 3;
        self.d = (self.d & 0xF83F) | (pg << 6);
    }

    fn set_b(&mut self,b: u8) {
        let pb = (b as u16) >> 3;
        self.d = (self.d & 0xFFC1) | (pb << 1);
    }

    fn set_a(&mut self,a: u8) {
        let pa = (a as u16) >> 7;
        self.d = (self.d & 0xFFFE) | pa;
    }

    fn set_rgb(&mut self,r: u8,g: u8,b: u8) {
        let pr = (r as u16) >> 3;
        let pg = (g as u16) >> 3;
        let pb = (b as u16) >> 3;
        self.d = (pr << 11) | (pg << 6) | (pb << 1) | 0x0001;
    }

    fn set_rgba(&mut self,r: u8,g: u8,b: u8,a: u8) {
        let pr = (r as u16) >> 3;
        let pg = (g as u16) >> 3;
        let pb = (b as u16) >> 3;
        let pa = (a as u16) >> 7;
        self.d = (pr << 11) | (pg << 6) | (pb << 1) | pa;
    }

    fn r(&self) -> u8 {
        let pr = ((self.d >> 11) & 0x001F) as u8;
        (pr << 3) | (pr >> 2)
    }

    fn g(&self) -> u8 {
        let pg = ((self.d >> 6) & 0x001F) as u8;
        (pg << 3) | (pg >> 2)
    }

    fn b(&self) -> u8 {
        let pb = ((self.d >> 1) & 0x001F) as u8;
        (pb << 3) | (pb >> 2)
    }

    fn a(&self) -> u8 {
        let pa = (self.d >> 15) as u8;
        if pa == 1 {
            0xFF
        }
        else {
            0x00
        }
    }

    fn rgb(&self,r: &mut u8,g: &mut u8,b: &mut u8) {
        let pr = ((self.d >> 11) & 0x001F) as u8;
        *r = (pr << 3) | (pr >> 2);
        let pg = ((self.d >> 6) & 0x001F) as u8;
        *g = (pg << 3) | (pg >> 2);
        let pb = ((self.d >> 1) & 0x001F) as u8;
        *b = (pb << 3) | (pb >> 2);
    }

    fn rgba(&self,r: &mut u8,g: &mut u8,b: &mut u8,a: &mut u8) {
        let pr = ((self.d >> 11) & 0x001F) as u8;
        *r = (pr << 3) | (pr >> 2);
        let pg = ((self.d >> 6) & 0x001F) as u8;
        *g = (pg << 3) | (pg >> 2);
        let pb = ((self.d >> 1) & 0x001F) as u8;
        *b = (pb << 3) | (pb >> 2);
        let pa = (self.d >> 15) as u8;
        if pa == 1 {
            *a = 0xFF;
        }
        else {
            *a = 0x00;
        }
    }
}

impl PartialEq<RGB5A1> for RGB5A1 {
    fn eq(&self,other: &RGB5A1) -> bool {
        self.d == other.d
    }
}

impl Pixel for RGB10A2 {

    fn new_rgb(r: u8,g: u8,b: u8) -> RGB10A2 {
        let pr = ((r as u32) << 2) | ((r as u32) >> 6);
        let pg = ((g as u32) << 2) | ((g as u32) >> 6);
        let pb = ((b as u32) << 2) | ((b as u32) >> 6);
        RGB10A2 { d: (pr << 22) | (pg << 12) | (pb << 2) | 0x00000003, }
    }

    fn new_rgba(r: u8,g: u8,b: u8,a: u8) -> RGB10A2 {
        let pr = ((r as u32) << 2) | ((r as u32) >> 6);
        let pg = ((g as u32) << 2) | ((g as u32) >> 6);
        let pb = ((b as u32) << 2) | ((b as u32) >> 6);
        let pa = (a as u32) >> 6;
        RGB10A2 { d: (pr << 22) | (pg << 12) | (pb << 2) | pa, }
    }

    fn set_r(&mut self,r: u8) {
        let pr = ((r as u32) << 2) | ((r as u32) >> 6);
        self.d = (self.d & 0x003FFFFF) | (pr << 22);
    }

    fn set_g(&mut self,g: u8) {
        let pg = ((g as u32) << 2) | ((g as u32) >> 6);
        self.d = (self.d & 0xFFC00FFF) | (pg << 12);
    }

    fn set_b(&mut self,b: u8) {
        let pb = ((b as u32) << 2) | ((b as u32) >> 6);
        self.d = (self.d & 0xFFFFF003) | (pb << 2);
    }

    fn set_a(&mut self,a: u8) {
        let pa = (a as u32) >> 6;
        self.d = (self.d & 0xFFFFFFFC) | pa;
    }

    fn set_rgb(&mut self,r: u8,g: u8,b: u8) {
        let pr = ((r as u32) << 2) | ((r as u32) >> 6);
        let pg = ((g as u32) << 2) | ((g as u32) >> 6);
        let pb = ((b as u32) << 2) | ((b as u32) >> 6);
        self.d = (pr << 22) | (pg << 12) | (pb << 2) | 0x00000003;
    }

    fn set_rgba(&mut self,r: u8,g: u8,b: u8,a: u8) {
        let pr = ((r as u32) << 2) | ((r as u32) >> 6);
        let pg = ((g as u32) << 2) | ((g as u32) >> 6);
        let pb = ((b as u32) << 2) | ((b as u32) >> 6);
        let pa = (a as u32) >> 6;
        self.d = (pr << 22) | (pg << 12) | (pb << 2) | pa;
    }

    fn r(&self) -> u8 {
        (self.d >> 24) as u8
    }

    fn g(&self) -> u8 {
        ((self.d >> 14) & 0x000000FF) as u8
    }

    fn b(&self) -> u8 {
        ((self.d >> 4) & 0x000000FF) as u8
    }

    fn a(&self) -> u8 {
        let pa = (self.d & 0x00000003) as u8;
        (pa << 6) | (pa << 4) | (pa << 2) | pa
    }

    fn rgb(&self,r: &mut u8,g: &mut u8,b: &mut u8) {
        *r = (self.d >> 24) as u8;
        *g = ((self.d >> 14) & 0x000000FF) as u8;
        *b = ((self.d >> 4) & 0x000000FF) as u8;
    }

    fn rgba(&self,r: &mut u8,g: &mut u8,b: &mut u8,a: &mut u8) {
        *r = (self.d >> 24) as u8;
        *g = ((self.d >> 14) & 0x000000FF) as u8;
        *b = ((self.d >> 4) & 0x000000FF) as u8;
        let pa = (self.d & 0x00000003) as u8;
        *a = (pa << 6) | (pa << 4) | (pa << 2) | pa;
    }
}

impl PartialEq<RGB10A2> for RGB10A2 {
    fn eq(&self,other: &RGB10A2) -> bool {
        self.d == other.d
    }
}
