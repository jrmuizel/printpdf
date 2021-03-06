//! Color module (CMYK or RGB). Shared between 2D and 3D module.

use image;

use *;
use glob_defines::*;
use lopdf::content::Operation;

/// Tuple for differentiating outline and fill colors
#[derive(Debug, Clone, PartialEq)]
pub enum PdfColor {
    FillColor(Color),
    OutlineColor(Color),
}

impl Into<Operation> for PdfColor {

    fn into(self)
    -> Operation
    {
        use lopdf::Object::*;
        use lopdf::content::Operation;

        // todo: incorporate ICC profile instead of just setting the default device cmyk color space
        let (color_identifier, color_vec) = {
            use self::PdfColor::*;
            match self {
                FillColor(fill) => {
                    let ci = match fill {
                        Color::Rgb(_) => { OP_COLOR_SET_FILL_CS_DEVICERGB }
                        Color::Cmyk(_) => { OP_COLOR_SET_FILL_CS_DEVICECMYK }
                        Color::Greyscale(_) => { OP_COLOR_SET_FILL_CS_DEVICEGRAY }
                        Color::SpotColor(_) => { OP_COLOR_SET_FILL_CS_DEVICECMYK }
                    };
                    let cvec = fill.into_vec().into_iter().map(move |float| Real(float)).collect();
                    (ci, cvec)
                },
                OutlineColor(outline) => {
                    let ci = match outline {
                        Color::Rgb(_) => { OP_COLOR_SET_STROKE_CS_DEVICERGB }
                        Color::Cmyk(_) => { OP_COLOR_SET_STROKE_CS_DEVICECMYK }
                        Color::Greyscale(_) => { OP_COLOR_SET_STROKE_CS_DEVICEGRAY }
                        Color::SpotColor(_) => { OP_COLOR_SET_STROKE_CS_DEVICECMYK }
                    };

                    let cvec = outline.into_vec().into_iter().map(move |float| Real(float)).collect();
                    (ci, cvec)
                }
            }
        };

        Operation::new(color_identifier, color_vec)
    }
}

/// Color space (enum for marking the number of bits a color has)
#[derive(Debug)]
pub enum ColorSpace {
    Rgb,
    Cmyk,
    Greyscale,
}

impl From<image::ColorType> for ColorSpace {
    fn from(color_type: image::ColorType)
    -> Self
    {
        use image::ColorType::*;
        match color_type {
            Gray(_) => ColorSpace::Greyscale,
            RGB(_) => ColorSpace::Rgb,
            Palette(_) => ColorSpace::Rgb, /* todo: support indexed colors*/
            GrayA(_) => ColorSpace::Greyscale,
            RGBA(_) => ColorSpace::Rgb,
        }
    }
}

impl Into<&'static str> for ColorSpace {
    fn into(self)
    -> &'static str
    {
        match self {
            ColorSpace::Rgb => "DeviceRGB",
            ColorSpace::Cmyk => "DeviceCMYK",
            ColorSpace::Greyscale => "DeviceGray",
        }
    }
}

/// How many bits does a color have?
#[derive(Debug)]
pub enum ColorBits {
    Bit1,
    Bit8,
    Bit16,
}

impl From<image::ColorType> for ColorBits {
    fn from(color_type: image::ColorType)
    -> ColorBits
    {
        use image::ColorType::*;
        use ColorBits::*;

        // not sure why the compile does not see this
        #[allow(unused_assignments)]
        let mut num_bytes_color_type = ColorBits::Bit1;

        match color_type {
            Gray(num_bytes) => num_bytes_color_type = match num_bytes {
                1 =>  Bit1,
                8 =>  Bit8,
                16 => Bit16,
                _ => Bit1,
            },
            RGB(num_bytes) => num_bytes_color_type = match num_bytes {
                1 =>  Bit1,
                8 =>  Bit8,
                16 => Bit16,
                _ => Bit1,
            },
            Palette(num_bytes) => num_bytes_color_type = match num_bytes {
                1 =>  Bit1,
                8 =>  Bit8,
                16 => Bit16,
                _ => Bit1,
            },
            GrayA(num_bytes) => num_bytes_color_type = match num_bytes {
                1 =>  Bit1,
                8 =>  Bit8,
                16 => Bit16,
                _ => Bit1,
            },
            RGBA(num_bytes) => num_bytes_color_type = match num_bytes {
                1 =>  Bit1,
                8 =>  Bit8,
                16 => Bit16,
                _ => Bit1,
            },
        }

        return num_bytes_color_type;
    }
}

impl Into<i64> for ColorBits {
    fn into(self)
    -> i64
    {
        match self {
            ColorBits::Bit1 => 1,
            ColorBits::Bit8 => 8,
            ColorBits::Bit16 => 16,
        }
    }
}

/// Wrapper for Rgb, Cmyk and other color types
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Rgb(Rgb),
    Cmyk(Cmyk),
    Greyscale(Greyscale),
    SpotColor(SpotColor)
}

impl Color {
    
    /// Consumes the color and converts into into a vector of numbers
    pub fn into_vec(self)
    -> Vec<f64>
    {
        match self {
            Color::Rgb(rgb) => { vec![rgb.r, rgb.g, rgb.b ]},
            Color::Cmyk(cmyk) => { vec![cmyk.c, cmyk.m, cmyk.y, cmyk.k ]},
            Color::Greyscale(gs) => { vec![gs.percent]},
            Color::SpotColor(spot) => { vec![spot.c, spot.m, spot.y, spot.k ]},
        }
    }

    /// Returns if the color has an icc profile attached
    pub fn get_icc_profile(&self)
    -> Option<&Option<IccProfileRef>>
    {
        match *self {
            Color::Rgb(ref rgb) => Some(&rgb.icc_profile),
            Color::Cmyk(ref cmyk) => Some(&cmyk.icc_profile),
            Color::Greyscale(ref gs) => Some(&gs.icc_profile),
            Color::SpotColor(_) => None,
        }
    }
}

/// RGB color
#[derive(Debug, Clone, PartialEq)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub icc_profile: Option<IccProfileRef>,
}

impl Rgb {

    pub fn new(r: f64, g: f64, b: f64, icc_profile: Option<IccProfileRef>)
    -> Self
    {
        Self { r, g, b, icc_profile }
    }
}


/// CMYK color
#[derive(Debug, Clone, PartialEq)]
pub struct Cmyk {
    pub c: f64,
    pub m: f64,
    pub y: f64,
    pub k: f64,
    pub icc_profile: Option<IccProfileRef>,
}

impl Cmyk {
    /// Creates a new CMYK color
    pub fn new(c: f64, m: f64, y: f64, k: f64, icc_profile: Option<IccProfileRef>)
    -> Self
    {
        Self { c, m, y, k, icc_profile }
    }
}

/// Greyscale color
#[derive(Debug, Clone, PartialEq)]
pub struct Greyscale {
    pub percent: f64,
    pub icc_profile: Option<IccProfileRef>,
}

impl Greyscale {
    pub fn new(percent: f64, icc_profile: Option<IccProfileRef>)
    -> Self
    {
        Self { percent, icc_profile }
    }
}


/// Spot color
/// Spot colors are like Cmyk, but without color space
/// They are essentially "named" colors from specific vendors
/// currently they are the same as a CMYK color.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpotColor {
    pub c: f64,
    pub m: f64,
    pub y: f64,
    pub k: f64,
}

impl SpotColor {
    pub fn new(c: f64, m: f64, y: f64, k: f64)
    -> Self
    {
        Self { c, m, y, k }
    }
}
