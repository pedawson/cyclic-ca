use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ColorScheme {
    Rainbow,
    Ocean,
    Fire,
    Grayscale,
    Custom,
}

impl ColorScheme {
    pub const ALL: [ColorScheme; 5] = [
        ColorScheme::Rainbow,
        ColorScheme::Ocean,
        ColorScheme::Fire,
        ColorScheme::Grayscale,
        ColorScheme::Custom,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            ColorScheme::Rainbow => "Rainbow",
            ColorScheme::Ocean => "Ocean",
            ColorScheme::Fire => "Fire",
            ColorScheme::Grayscale => "Grayscale",
            ColorScheme::Custom => "Custom",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Neighborhood {
    VonNeumann,
    Moore,
    Extended,
}

impl Neighborhood {
    pub const ALL: [Neighborhood; 3] = [
        Neighborhood::VonNeumann,
        Neighborhood::Moore,
        Neighborhood::Extended,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Neighborhood::VonNeumann => "Von Neumann (4)",
            Neighborhood::Moore => "Moore (8)",
            Neighborhood::Extended => "Extended (12)",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Symmetry {
    None,
    Horizontal,
    Vertical,
    FourFold,
}

impl Symmetry {
    pub const ALL: [Symmetry; 4] = [
        Symmetry::None,
        Symmetry::Horizontal,
        Symmetry::Vertical,
        Symmetry::FourFold,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Symmetry::None       => "None",
            Symmetry::Horizontal => "Horizontal (L=R)",
            Symmetry::Vertical   => "Vertical (T=B)",
            Symmetry::FourFold   => "Four-fold (L=R=T=B)",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
    pub grid: Vec<u8>,
    next_grid: Vec<u8>,
    pub color_scheme: ColorScheme,
    colors: Vec<[u8; 3]>,
    pub neighborhood: Neighborhood,
    pub threshold: usize,
    pixel_buf: Vec<egui::Color32>,
}

impl CyclicCellularAutomata {
    pub fn new(width: usize, height: usize, num_types: usize) -> Self {
        let len = width * height;
        let color_scheme = ColorScheme::Rainbow;
        let colors = Self::generate_colors(num_types, color_scheme);

        let mut ca = Self {
            width,
            height,
            num_types,
            grid: vec![0u8; len],
            next_grid: vec![0u8; len],
            color_scheme,
            colors,
            neighborhood: Neighborhood::VonNeumann,
            threshold: 1,
            pixel_buf: vec![egui::Color32::BLACK; len],
        };
        ca.apply_pattern(Pattern::Random);
        ca
    }

    pub fn resize(&mut self, width: usize, height: usize, num_types: usize) {
        self.width = width;
        self.height = height;
        self.num_types = num_types;
        let len = width * height;
        self.grid = vec![0u8; len];
        self.next_grid = vec![0u8; len];
        self.pixel_buf = vec![egui::Color32::BLACK; len];
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
                    ColorScheme::Custom => {
                        // Pastel palette: offset hues with moderate saturation
                        let h = (t * 360.0 + 30.0) % 360.0;
                        let s: f32 = 0.6;
                        let l: f32 = 0.65;
                        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
                        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
                        let m = l - c / 2.0;
                        let (r, g, b) = if h < 60.0 {
                            (c, x, 0.0)
                        } else if h < 120.0 {
                            (x, c, 0.0)
                        } else if h < 180.0 {
                            (0.0, c, x)
                        } else if h < 240.0 {
                            (0.0, x, c)
                        } else if h < 300.0 {
                            (x, 0.0, c)
                        } else {
                            (c, 0.0, x)
                        };
                        [((r + m) * 255.0) as u8, ((g + m) * 255.0) as u8, ((b + m) * 255.0) as u8]
                    }
                }
            })
            .collect()
    }

    pub fn apply_pattern(&mut self, pattern: Pattern) {
        let mut rng = rand::thread_rng();

        for y in 0..self.height {
            for x in 0..self.width {
                let val = match pattern {
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
                self.grid[y * self.width + x] = val as u8;
            }
        }
    }

    pub fn randomize(&mut self) {
        self.apply_pattern(Pattern::Random);
    }

    pub fn clear(&mut self) {
        self.grid.fill(0);
    }

    fn get_neighbors(&self, x: usize, y: usize) -> ([(usize, usize); 12], usize) {
        let w = self.width as i32;
        let h = self.height as i32;
        let xi = x as i32;
        let yi = y as i32;
        let mut buf = [(0usize, 0usize); 12];

        match self.neighborhood {
            Neighborhood::VonNeumann => {
                buf[0] = ((xi - 1).rem_euclid(w) as usize, y);
                buf[1] = ((xi + 1).rem_euclid(w) as usize, y);
                buf[2] = (x, (yi - 1).rem_euclid(h) as usize);
                buf[3] = (x, (yi + 1).rem_euclid(h) as usize);
                (buf, 4)
            }
            Neighborhood::Moore => {
                let mut len = 0;
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        buf[len] = (
                            (xi + dx).rem_euclid(w) as usize,
                            (yi + dy).rem_euclid(h) as usize,
                        );
                        len += 1;
                    }
                }
                (buf, len)
            }
            Neighborhood::Extended => {
                let offsets: [(i32, i32); 12] = [
                    (-1, 0), (1, 0), (0, -1), (0, 1),
                    (-2, 0), (2, 0), (0, -2), (0, 2),
                    (-1, -1), (1, -1), (-1, 1), (1, 1),
                ];
                for (i, &(dx, dy)) in offsets.iter().enumerate() {
                    buf[i] = (
                        (xi + dx).rem_euclid(w) as usize,
                        (yi + dy).rem_euclid(h) as usize,
                    );
                }
                (buf, 12)
            }
        }
    }

    pub fn update(&mut self) {
        let mut prey_types = [0u8; 25];
        for i in 0..self.num_types {
            prey_types[i] = ((i + self.num_types - 1) % self.num_types) as u8;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let current_type = self.grid[idx];
                let prey_type = prey_types[current_type as usize];
                let mut prey_count = 0usize;

                let (neighbors, nlen) = self.get_neighbors(x, y);
                for &(nx, ny) in &neighbors[..nlen] {
                    if self.grid[ny * self.width + nx] == prey_type {
                        prey_count += 1;
                    }
                }

                self.next_grid[idx] = if prey_count >= self.threshold {
                    prey_type
                } else {
                    current_type
                };
            }
        }

        std::mem::swap(&mut self.grid, &mut self.next_grid);
    }

    /// Mirror the grid according to the chosen symmetry mode.
    pub fn apply_symmetry(&mut self, sym: Symmetry) {
        let w = self.width;
        match sym {
            Symmetry::None => {}
            Symmetry::Horizontal => {
                for y in 0..self.height {
                    for x in 0..w / 2 {
                        let val = self.grid[y * w + x];
                        self.grid[y * w + (w - 1 - x)] = val;
                    }
                }
            }
            Symmetry::Vertical => {
                for y in 0..self.height / 2 {
                    for x in 0..w {
                        let val = self.grid[y * w + x];
                        self.grid[(self.height - 1 - y) * w + x] = val;
                    }
                }
            }
            Symmetry::FourFold => {
                for y in 0..self.height / 2 {
                    for x in 0..w / 2 {
                        let val = self.grid[y * w + x];
                        self.grid[y * w + (w - 1 - x)] = val;
                        self.grid[(self.height - 1 - y) * w + x] = val;
                        self.grid[(self.height - 1 - y) * w + (w - 1 - x)] = val;
                    }
                }
            }
        }
    }

    /// Returns flat RGB bytes (r,g,b per pixel, row-major) for PNG export.
    pub fn to_rgb_bytes(&self) -> Vec<u8> {
        self.grid
            .iter()
            .flat_map(|&cell| {
                let [r, g, b] = self.colors[cell as usize];
                [r, g, b]
            })
            .collect()
    }

    pub fn to_color_image(&mut self) -> egui::ColorImage {
        for (i, &cell) in self.grid.iter().enumerate() {
            let [r, g, b] = self.colors[cell as usize];
            self.pixel_buf[i] = egui::Color32::from_rgb(r, g, b);
        }

        egui::ColorImage {
            size: [self.width, self.height],
            pixels: self.pixel_buf.clone(),
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell_type: u8) {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x] = cell_type;
        }
    }

    pub fn population_counts(&self) -> Vec<usize> {
        let mut counts = vec![0usize; self.num_types];
        for &cell in &self.grid {
            if (cell as usize) < self.num_types {
                counts[cell as usize] += 1;
            }
        }
        counts
    }

    pub fn get_color(&self, idx: usize) -> [u8; 3] {
        self.colors.get(idx).copied().unwrap_or([128, 128, 128])
    }
}
