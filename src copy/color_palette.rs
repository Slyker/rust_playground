use std::{ collections::HashMap, ops::Sub, time::{ Duration, Instant } };
use palette::{ rgb::{ self, Rgb }, FromColor, Hsv, IntoColor, Srgb };

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub hsv_tolerance: Option<[f32; 3]>,
    pub hsv: Hsv,
}

impl Color {
    fn new_rgb(rgb: [u8; 3], hsv_tolerance: Option<[f32; 3]>) -> Self {
        let hsv = Hsv::from_color(Srgb::new(rgb[0], rgb[1], rgb[2]).into_format());
        Self {
            hsv,
            hsv_tolerance,
        }
    }

    // Constructeur pour les couleurs HSL qui convertit en RGB
    fn new_hsv(hsv: [f32; 3], hsv_tolerance: Option<[f32; 3]>) -> Self {
        let hsv = Hsv::new(hsv[0], hsv[1], hsv[2]);
        Self {
            hsv,
            hsv_tolerance,
        }
    }

    fn to_hsv(pixel: [u8; 4]) -> Hsv {
        Hsv::from_color(Srgb::new(pixel[0], pixel[1], pixel[2]).into_format())
    }
    pub fn match_pixel(&self, pixel: [u8; 4], label: String) -> bool {
        let hsv = Self::to_hsv(pixel);
        let (hue_tolerance, saturation_tolerance, value_tolerance) = match self.hsv_tolerance {
            Some(tolerance) => (tolerance[0], tolerance[1], tolerance[2]),
            None => (0.0, 0.0, 0.0),
        };
        let h_diff =
            (self.hsv.hue.into_positive_degrees() - hsv.hue.into_positive_degrees()).abs() <=
            hue_tolerance;
        if !h_diff {
            return false;
        }
        let s_diff = (self.hsv.saturation - hsv.saturation).abs() <= saturation_tolerance;
        if !s_diff {
            return false;
        }
        let v_diff = (self.hsv.value - hsv.value).abs() <= value_tolerance;
        if !v_diff {
            return false;
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorDetection {
    label: String,
    pub colors: Color,
    pub tolerance: u8,
}

impl ColorDetection {
    pub fn from(color: [u8; 3]) -> Self {
        Self {
            colors: Color::new_rgb([color[0], color[1], color[2]], None),
            tolerance: 5,
            label: "default".to_string(),
        }
    }

    pub fn from_hsv(color: [f32; 3], tolerance: Option<[f32; 3]>) -> Self {
        Self {
            colors: Color::new_hsv([color[0], color[1], color[2]], tolerance),
            tolerance: 5,
            label: "default".to_string(),
        }
    }

    pub fn set_label(&mut self, label: String) -> &mut Self {
        self.label = label;
        return self;
    }

    #[inline(always)]
    pub fn color_match(&self, color: &[u8; 4]) -> bool {
        // Color matching based on tolerance
        if self.colors.match_pixel(*color, self.label.clone()) {
            return true;
        } else {
        }
        false
    }
}

impl Default for ColorDetection {
    fn default() -> Self {
        Self {
            colors: Color::new_rgb([0, 0, 0], None),
            tolerance: 0,
            label: "default".to_string(),
        }
    }
}
