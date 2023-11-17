pub struct Palette {
    pub colors: Vec<(f32, [u8; 3])>,
}

impl Palette {
    pub fn new(colors: Vec<[u8; 3]>) -> Self {
        assert!(colors.len() >= 2, "must have at least 2 colors");
        let colors = colors
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, color)| (i as f32 / (colors.len() - 1) as f32, color))
            .collect();
        Self { colors }
    }

    pub fn levels(colors: Vec<(f32, [u8; 3])>) -> Self {
        let mut colors = colors.clone();
        colors.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        for color in colors.iter() {
            assert!(
                0.0 <= color.0 && color.0 <= 1.0,
                "value must be between 0 and 1"
            );
        }
        Self { colors }
    }

    pub fn inverse(&self) -> Self {
        let inverted = self
            .colors
            .iter()
            .map(|(p, color)| (1.0 - p, color.clone()))
            .collect();
        Self::levels(inverted)
    }

    pub fn nearest_color(&self, value: f32) -> [u8; 3] {
        assert!(
            0.0 <= value && value <= 1.0,
            "value must be between 0 and 1"
        );

        self.colors
            .iter()
            .min_by(|a, b| {
                let a = (a.0 - value).abs();
                let b = (b.0 - value).abs();
                a.partial_cmp(&b).unwrap()
            })
            .unwrap()
            .1
    }

    pub fn color(&self, value: f32) -> [u8; 3] {
        assert!(
            0.0 <= value && value <= 1.0,
            "value must be between 0 and 1"
        );

        if value <= self.colors[0].0 {
            return self.colors[0].1;
        }

        for (prev, next) in self.colors.iter().zip(self.colors.iter().skip(1)) {
            if prev.0 <= value && value <= next.0 {
                return self.interpolate((value - prev.0) / (next.0 - prev.0), next.1, prev.1);
            }
        }

        self.colors[self.colors.len() - 1].1
    }

    fn interpolate(&self, frac: f32, upper: [u8; 3], lower: [u8; 3]) -> [u8; 3] {
        assert!(0.0 <= frac && frac <= 1.0, "value must be between 0 and 1");

        // Perform the interpolation for each color channel
        let [rf, gf, bf] = lower;
        let [rc, gc, bc] = upper;

        let r = (1.0 - frac) * rf as f32 + frac * rc as f32;
        let g = (1.0 - frac) * gf as f32 + frac * gc as f32;
        let b = (1.0 - frac) * bf as f32 + frac * bc as f32;

        [r as u8, g as u8, b as u8]
    }

    pub fn monochrome() -> Palette {
        Palette::new(vec![
            [0, 0, 0],       // Black
            [64, 64, 64],    // Dark Gray
            [128, 128, 128], // Gray
            [192, 192, 192], // Light Gray
            [255, 255, 255], // White
        ])
    }

    pub fn ocean() -> Palette {
        Palette::new(vec![
            [7, 3, 89],      // Deep Ocean Blue
            [9, 106, 183],   // Mid Ocean Blue
            [78, 168, 222],  // Shallow Water Blue
            [144, 224, 239], // Surf Blue
            [0, 0, 0],       // Black
        ])
    }

    pub fn dragon() -> Palette {
        Palette::new(vec![
            [17, 9, 0],      // Deep Ember
            [128, 9, 0],     // Smoldering Red
            [255, 150, 0],   // Blaze Orange
            [255, 255, 102], // Hot Yellow
            [0, 0, 0],       // Black
        ])
    }

    pub fn fire() -> Palette {
        Palette::new(vec![
            [17, 9, 0],      // Deep Ember
            [128, 9, 0],     // Smoldering Red
            [255, 69, 0],    // Bright Fire
            [255, 150, 0],   // Blaze Orange
            [255, 255, 102], // Hot Yellow
        ])
    }

    pub fn wiki() -> Palette {
        Palette::new(vec![
            [66, 30, 15],    // dark brown
            [25, 7, 26],     // dark violett
            [4, 4, 73],      // darkest blue
            [104, 151, 199], // dark blue
            [78, 9, 0],      // Smoldering Red
            [134, 181, 229], // dark blue
            [171, 206, 218], // lightest blue
            [241, 233, 191], // lightest yellow
            [255, 170, 0],   // dirty yellow
            [204, 128, 0],   // brown 0
            [106, 52, 3],    // brown 1
            [0, 0, 0],       // black
        ])
    }

    pub fn rgb() -> Palette {
        Palette::new(vec![
            [255, 0, 0], // Red
            [0, 255, 0], // Green
            [0, 0, 255], // Blue
        ])
    }
}
