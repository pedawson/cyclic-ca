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

    pub fn as_u8(self) -> u8 {
        match self {
            ColorScheme::Rainbow   => 0,
            ColorScheme::Ocean     => 1,
            ColorScheme::Fire      => 2,
            ColorScheme::Grayscale => 3,
        }
    }

    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => ColorScheme::Ocean,
            2 => ColorScheme::Fire,
            3 => ColorScheme::Grayscale,
            _ => ColorScheme::Rainbow,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

    pub fn as_u8(self) -> u8 {
        match self {
            Neighborhood::VonNeumann => 0,
            Neighborhood::Moore      => 1,
            Neighborhood::Extended   => 2,
        }
    }

    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => Neighborhood::Moore,
            2 => Neighborhood::Extended,
            _ => Neighborhood::VonNeumann,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

    pub fn as_u8(self) -> u8 {
        match self {
            Symmetry::None       => 0,
            Symmetry::Horizontal => 1,
            Symmetry::Vertical   => 2,
            Symmetry::FourFold   => 3,
        }
    }

    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => Symmetry::Horizontal,
            2 => Symmetry::Vertical,
            3 => Symmetry::FourFold,
            _ => Symmetry::None,
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
    pub neighborhood: Neighborhood,
    pub threshold: usize,
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
            neighborhood: Neighborhood::VonNeumann,
            threshold: 1,
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
        // neighborhood and threshold are preserved
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

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let w = self.width as i32;
        let h = self.height as i32;
        let xi = x as i32;
        let yi = y as i32;

        match self.neighborhood {
            Neighborhood::VonNeumann => vec![
                ((xi - 1).rem_euclid(w) as usize, y),
                ((xi + 1).rem_euclid(w) as usize, y),
                (x, (yi - 1).rem_euclid(h) as usize),
                (x, (yi + 1).rem_euclid(h) as usize),
            ],
            Neighborhood::Moore => {
                let mut neighbors = Vec::with_capacity(8);
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        neighbors.push((
                            (xi + dx).rem_euclid(w) as usize,
                            (yi + dy).rem_euclid(h) as usize,
                        ));
                    }
                }
                neighbors
            }
            Neighborhood::Extended => {
                // Cardinal directions at distance 1 and 2, plus diagonals at distance 1 (12 total)
                let offsets: [(i32, i32); 12] = [
                    (-1, 0), (1, 0), (0, -1), (0, 1),
                    (-2, 0), (2, 0), (0, -2), (0, 2),
                    (-1, -1), (1, -1), (-1, 1), (1, 1),
                ];
                offsets
                    .iter()
                    .map(|&(dx, dy)| {
                        (
                            (xi + dx).rem_euclid(w) as usize,
                            (yi + dy).rem_euclid(h) as usize,
                        )
                    })
                    .collect()
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
                let mut prey_count = 0;

                for (nx, ny) in self.get_neighbors(x, y) {
                    if self.grid[ny][nx] == prey_type {
                        prey_count += 1;
                    }
                }

                self.next_grid[y][x] = if prey_count >= self.threshold {
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
        match sym {
            Symmetry::None => {}
            Symmetry::Horizontal => {
                for y in 0..self.height {
                    for x in 0..self.width / 2 {
                        let val = self.grid[y][x];
                        self.grid[y][self.width - 1 - x] = val;
                    }
                }
            }
            Symmetry::Vertical => {
                for y in 0..self.height / 2 {
                    for x in 0..self.width {
                        let val = self.grid[y][x];
                        self.grid[self.height - 1 - y][x] = val;
                    }
                }
            }
            Symmetry::FourFold => {
                for y in 0..self.height / 2 {
                    for x in 0..self.width / 2 {
                        let val = self.grid[y][x];
                        self.grid[y][self.width - 1 - x] = val;
                        self.grid[self.height - 1 - y][x] = val;
                        self.grid[self.height - 1 - y][self.width - 1 - x] = val;
                    }
                }
            }
        }
    }

    /// Returns flat RGB bytes (r,g,b per pixel, row-major) for PNG export.
    pub fn to_rgb_bytes(&self) -> Vec<u8> {
        self.grid
            .iter()
            .flat_map(|row| {
                row.iter().flat_map(|&cell| {
                    let [r, g, b] = self.colors[cell];
                    [r, g, b]
                })
            })
            .collect()
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
