use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColorScheme {
    Rainbow,
    Ocean,
    Fire,
    Grayscale,
}

impl ColorScheme {
    pub const ALL: [ColorScheme; 4] = [
        ColorScheme::Rainbow,
        ColorScheme::Ocean,
        ColorScheme::Fire,
        ColorScheme::Grayscale,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            ColorScheme::Rainbow => "Rainbow",
            ColorScheme::Ocean => "Ocean",
            ColorScheme::Fire => "Fire",
            ColorScheme::Grayscale => "Grayscale",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    Random,
    Stripes,
    Checkerboard,
    Gradient,
    Center,
}

impl Pattern {
    pub const ALL: [Pattern; 5] = [
        Pattern::Random,
        Pattern::Stripes,
        Pattern::Checkerboard,
        Pattern::Gradient,
        Pattern::Center,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Pattern::Random => "Random",
            Pattern::Stripes => "Stripes",
            Pattern::Checkerboard => "Checkerboard",
            Pattern::Gradient => "Gradient",
            Pattern::Center => "Center Blob",
        }
    }
}

pub struct CyclicCellularAutomata {
    pub width: usize,
    pub height: usize,
    pub num_types: usize,
    pub grid: Vec<Vec<usize>>,
    next_grid: Vec<Vec<usize>>,
    pub color_scheme: ColorScheme,
    colors: Vec<[u8; 3]>,
}

impl CyclicCellularAutomata {
    pub fn new(width: usize, height: usize, num_types: usize) -> Self {
        let grid = vec![vec![0; width]; height];
        let next_grid = vec![vec![0; width]; height];
        let color_scheme = ColorScheme::Rainbow;
        let colors = Self::generate_colors(num_types, color_scheme);

        let mut ca = Self {
            width,
            height,
            num_types,
            grid,
            next_grid,
            color_scheme,
            colors,
        };
        ca.apply_pattern(Pattern::Random);
        ca
    }

    pub fn resize(&mut self, width: usize, height: usize, num_types: usize) {
        self.width = width;
        self.height = height;
        self.num_types = num_types;
        self.grid = vec![vec![0; width]; height];
        self.next_grid = vec![vec![0; width]; height];
        self.colors = Self::generate_colors(num_types, self.color_scheme);
        self.apply_pattern(Pattern::Random);
    }

    pub fn set_color_scheme(&mut self, scheme: ColorScheme) {
        self.color_scheme = scheme;
        self.colors = Self::generate_colors(self.num_types, scheme);
    }

    fn generate_colors(num_types: usize, scheme: ColorScheme) -> Vec<[u8; 3]> {
        (0..num_types)
            .map(|i| {
                let t = i as f32 / num_types as f32;
                match scheme {
                    ColorScheme::Rainbow => {
                        let h = t * 6.0;
                        let c = 1.0;
                        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
                        let (r, g, b) = if h < 1.0 {
                            (c, x, 0.0)
                        } else if h < 2.0 {
                            (x, c, 0.0)
                        } else if h < 3.0 {
                            (0.0, c, x)
                        } else if h < 4.0 {
                            (0.0, x, c)
                        } else if h < 5.0 {
                            (x, 0.0, c)
                        } else {
                            (c, 0.0, x)
                        };
                        [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]
                    }
                    ColorScheme::Ocean => {
                        let r = (t * 50.0) as u8;
                        let g = (100.0 + t * 155.0) as u8;
                        let b = (150.0 + t * 105.0) as u8;
                        [r, g, b]
                    }
                    ColorScheme::Fire => {
                        let r = (255.0 * (0.5 + t * 0.5).min(1.0)) as u8;
                        let g = (255.0 * (t * 0.8)) as u8;
                        let b = (255.0 * (t * 0.3)) as u8;
                        [r, g, b]
                    }
                    ColorScheme::Grayscale => {
                        let v = (t * 255.0) as u8;
                        [v, v, v]
                    }
                }
            })
            .collect()
    }

    pub fn apply_pattern(&mut self, pattern: Pattern) {
        let mut rng = rand::thread_rng();

        for y in 0..self.height {
            for x in 0..self.width {
                self.grid[y][x] = match pattern {
                    Pattern::Random => rng.gen_range(0..self.num_types),
                    Pattern::Stripes => (x / 10) % self.num_types,
                    Pattern::Checkerboard => ((x / 10) + (y / 10)) % self.num_types,
                    Pattern::Gradient => {
                        let t = (x as f32 / self.width as f32 + y as f32 / self.height as f32) / 2.0;
                        (t * self.num_types as f32) as usize % self.num_types
                    }
                    Pattern::Center => {
                        let cx = self.width / 2;
                        let cy = self.height / 2;
                        let dx = (x as isize - cx as isize).abs() as usize;
                        let dy = (y as isize - cy as isize).abs() as usize;
                        let dist = ((dx * dx + dy * dy) as f32).sqrt();
                        let max_dist = ((cx * cx + cy * cy) as f32).sqrt();
                        ((dist / max_dist) * self.num_types as f32) as usize % self.num_types
                    }
                };
            }
        }
    }

    pub fn randomize(&mut self) {
        self.apply_pattern(Pattern::Random);
    }

    pub fn clear(&mut self) {
        for row in &mut self.grid {
            for cell in row {
                *cell = 0;
            }
        }
    }

    pub fn update(&mut self) {
        let prey_types: Vec<usize> = (0..self.num_types)
            .map(|i| (i + self.num_types - 1) % self.num_types)
            .collect();

        for y in 0..self.height {
            for x in 0..self.width {
                let current_type = self.grid[y][x];
                let prey_type = prey_types[current_type];

                self.next_grid[y][x] = current_type;

                let neighbors = [
                    ((x + self.width - 1) % self.width, y),
                    ((x + 1) % self.width, y),
                    (x, (y + self.height - 1) % self.height),
                    (x, (y + 1) % self.height),
                ];

                for &(nx, ny) in &neighbors {
                    if self.grid[ny][nx] == prey_type {
                        self.next_grid[y][x] = prey_type;
                        break;
                    }
                }
            }
        }

        std::mem::swap(&mut self.grid, &mut self.next_grid);
    }

    pub fn to_color_image(&self) -> egui::ColorImage {
        let pixels: Vec<egui::Color32> = self
            .grid
            .iter()
            .flat_map(|row| {
                row.iter().map(|&cell| {
                    let [r, g, b] = self.colors[cell];
                    egui::Color32::from_rgb(r, g, b)
                })
            })
            .collect();

        egui::ColorImage {
            size: [self.width, self.height],
            pixels,
        }
    }
}
