// Map Generator 5

// Generates a randomized rectangle of characters representing a map with different characters
// representing water, lowlands, and hills

// When I was teaching myself C# in high school, I wrote 4 versions of a "Map Generator" program.
// Several years later, I thought it would be fun to write to write this 5th version.

use std::io;
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::distributions::Uniform;

const WATER: &str = "  ";
const LOWLAND: &str = "##";
const HILL: &str = "ɅɅ";

struct Map {
    height: usize, // number of rows
    width:  usize, // number of columns
    grid: Vec<Vec<u16>>, // 2-dimensional grid of u16s on the heap
    rng: ThreadRng, // Random number generator
    area: usize, // Width * height
    min: u16, // Minimum value in grid
    avg: u16, // Average value in grid
    max: u16  // Maximum value in grid

    // Apparently array sizes must be known at compile time, so for grid I have to use a Vec, which
    // is similar to Java's ArrayList, even though grid does not need to be resizable.
}

impl Map {
    fn new(height: usize, width: usize) -> Map {
        // This may not be the most elegant way to do this. Oh well...
        let mut grid = Vec::with_capacity(height);
        for r in 0..height {
            grid.push(Vec::with_capacity(width));
            for _ in 0..width {
                grid[r].push(0);
            }
        }
        return Map { height, width, grid,
            rng: rand::thread_rng(), area: width * height, min: 0, avg: 0, max: 0 };
    }

    fn calc_min_avg_max(&mut self) {
        // Calculate self.min, self.avg, and self.max.
        let mut sum = 0usize;
        self.min = u16::MAX;
        self.max = 0u16;

        for r in 0..self.height {
            for c in 0..self.width {
                let level = self.grid[r][c];
                sum += level as usize;
                if level < self.min {
                    self.min = level;
                }
                else if self.max < level {
                    self.max = level;
                }
            }
        }
        self.avg = (sum / self.width / self.height) as u16;
    }

    fn mutate(&mut self) {
        const SLOPES: [f32; 10] = [-4.0, -2.0, -1.0, -0.5, -0.25, 0.25, 0.5, 1.0, 2.0, 4.0];
        // Possible slopes of a random line

        let cud = Uniform::new(0, self.width); // Column uniform dist.
        let rud = Uniform::new(0, self.height); // Row uniform dist.
        let sud = Uniform::new(0, SLOPES.len()); // Slope uniform dist.

        let row = self.rng.sample(cud) as f32; // Random row
        let col = self.rng.sample(rud) as f32; // Random column
        let slope = SLOPES[self.rng.sample(sud)]; // Random slope
        let b = self.rng.gen_bool(0.5); // Random true or false with probability 1/2

        // Let there be a line f(c) = slope * (c - col) + row.
        // Depending on b, either increment all grid elements below or on the line, or increment all
        // grid elements above or on the line.

        for r in 0..self.height {
            for c in 0..self.width {
                let r_f32 = r as f32;
                let f = slope * (c as f32 - col) + row;
                if f == r_f32 || ((f < r_f32) ^ b) {
                    self.grid[r][c] += 1;
                }
            }
        }
    }

    fn to_text(&self, beach_level: u16, hills_level: u16) -> String {
        let mut text = String::with_capacity(self.area * 3);
        // This should be more than enough capacity in most cases.

        let top_bottom_len = self.width * 2 + 4;
        let mut top_bottom = String::with_capacity(top_bottom_len);
        top_bottom.push(' ');
        for _ in 2..top_bottom_len {
            top_bottom.push('-');
        }
        top_bottom.push('\n');

        text.push_str(&top_bottom);
        for r in 0..self.height {
            text.push_str("| ");
            for c in 0..self.width {
                let level = self.grid[r][c];
                let ch =
                    if level < beach_level {WATER}
                    else if level < hills_level {LOWLAND}
                    else {HILL};
                text.push_str(ch);
            }
            text.push_str(" |\n");
        }
        text.push_str(&top_bottom);

        return text;
    }
}

const MAX_WIDTH: usize = 75;
const MAX_HEIGHT: usize = 150;

fn main() {
    println!("Map-Generator-5");
    let stdin = io::stdin();
    let (mut height, mut width) = (String::new(), String::new());

    println!("Height?");
    stdin.read_line(&mut height).expect("Failed to read height");
    let height = match height.trim().parse() {
        Ok(h) => {
            if h <= MAX_HEIGHT {
                h
            }
            else {
                println!("Height is too big. Limits are width ≤ {}, height ≤ {}.",
                         MAX_WIDTH, MAX_HEIGHT);
                return;
            }
        },
        Err(_) => {
            println!("Failed to parse height as an unsigned integer");
            return;
        }
    };

    println!("Width?");
    stdin.read_line(&mut width).expect("Failed to read width");
    let width = match width.trim().parse() {
        Ok(w) => {
            if w <= MAX_WIDTH {
                w
            }
            else {
                println!("Width is too big. Limits are width ≤ {}, height ≤ {}.",
                         MAX_WIDTH, MAX_HEIGHT);
                return;
            }
        },
        Err(_) => {
            println!("Failed to parse width as an unsigned integer");
            return;
        }
    };

    let mut map = Map::new(height, width);
    let mut rem = u16::MAX;

    loop {
        let mut nm = String::new(); // Number of mutations in this loop iteration
        println!("Mutate how many times? (At most {} times remain. 0 to exit.)", rem);
        stdin.read_line(&mut nm).expect("Failed to read number of mutations");
        let mut nm: u16 = match nm.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Failed to parse number of mutations as an unsigned integer");
                return;
            }
        };

        if nm == 0 {
            println!("Exited!");
            break;
        }
        else if nm > rem {
            println!("{} > {}. Mutating only {} times...", nm, rem, rem);
            nm = rem;
        }
        for _ in 0..nm {
            map.mutate();
        }
        rem -= nm;

        map.calc_min_avg_max();
        print!("Mutated {} times total | Area = {} | ",
            u16::MAX - rem, map.area);
        println!("Minimum elevation = {} | Average elevation = {} | Maximum elevation = {}",
            map.min, map.avg, map.max);

        let mut beach_level = String::new();
        println!("Beach elevation? (Lower elevations will be covered in water.)");
        stdin.read_line(&mut beach_level).expect("Failed to read beach elevation");
        let beach_level = match beach_level.trim().parse() {
            Ok(b) => b,
            Err(_) => {
                println!("Failed to parse beach elevation as an unsigned integer");
                return;
            }
        };

        let mut hills_level = String::new();
        println!("Elevation of lowest foothills?");
        stdin.read_line(&mut hills_level).expect("Failed to read hills elevation");
        let hills_level = match hills_level.trim().parse() {
            Ok(h) => h,
            Err(_) => {
                println!("Failed to parse hills elevation as an unsigned integer");
                return;
            }
        };

        let text = map.to_text(beach_level, hills_level);
        println!("{}", text); // Also print a blank line at end

        if rem == 0 {
            println!("Done!");
            break;
        }
    }
}
