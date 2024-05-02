use std::{ collections::HashMap, ops::Sub, time::{ Duration, Instant } };
use color_art::{ color, Color };

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Neuz_Color {
    pub color: Color,
    pub hsv_tolerance: Option<[f64; 3]>,
}

impl Neuz_Color {
    fn new_rgb(rgb: [u8; 3], rgb_tolerance: Option<u8>) -> Self {
        Self {
            hsv_tolerance: None,
            color: Color::from_rgb(rgb[0], rgb[1], rgb[2]).expect("Invalid RGB color"),
        }
    }

    // Constructeur pour les couleurs HSL qui convertit en RGB
    fn new_hsv(hsv: [f64; 3], hsv_tolerance: Option<[f64; 3]>) -> Self {
        Self {
            hsv_tolerance,
            color: Color::from_hsv(hsv[0] , hsv[1] , hsv[2] ).expect("Invalid HSV color"),
        }
    }

    fn to_hsv(pixel: [u8; 4]) -> Color {
        color!(rgb(pixel[0], pixel[1], pixel[2]))
    }

    pub fn match_pixel(&self, pixel: [u8; 4], label: String) -> bool {
        let pixel = Self::to_hsv(pixel);
        let tolerance = self.hsv_tolerance.unwrap_or([0.0, 0.0, 0.0]);

        let hue_diff = pixel.hsv_hue() - self.color.hsv_hue();
        let hue_valid = hue_diff.abs() <= tolerance[0] ;
        
        let sat_diff = pixel.hsv_saturation() - self.color.hsv_saturation();
        let sat_valid = sat_diff.abs() <= tolerance[1] ;
        
        let val_diff = pixel.hsv_value() - self.color.hsv_value();
        let val_valid = val_diff.abs() <= tolerance[2] ;
        
        hue_valid && sat_valid && val_valid
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorDetection {
    label: String,
    pub colors: Vec<Neuz_Color>,
    pub tolerance: u8,
}

impl ColorDetection {
    pub fn from(color: Vec<[u8; 3]>) -> Self {
        Self {
            colors: {
                let mut colors = vec![];
                for c in color {
                    colors.push(Neuz_Color::new_rgb(c, None));
                }
                colors
            },
            tolerance: 5,
            label: "default".to_string(),
        }
    }

    pub fn from_hsv(color: Vec<[f64; 3]>, tolerance: Option<[f64; 3]>) -> Self {
        Self {
            colors: {
                let mut colors = vec![];
                for c in color {
                    colors.push(Neuz_Color::new_hsv([c[0], c[1], c[2]], tolerance));
                }
                colors
            },
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
        for ref_color in &self.colors {
            if ref_color.match_pixel(*color, self.label.clone()) {
                return true;
            } else {
            }
        }
        false
    }
}

impl Default for ColorDetection {
    fn default() -> Self {
        Self {
            colors: vec![],
            tolerance: 0,
            label: "default".to_string(),
        }
    }
}
