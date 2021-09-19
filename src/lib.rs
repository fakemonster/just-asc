mod shapes;

use crate::shapes::{Circle, Ellipse, Line, Point, Rectangle};

#[derive(Debug)]
struct Cell {
    coords: Rectangle,
    quadrants: [Rectangle; 4],
    quads_filled: [bool; 4],
}

impl Cell {
    fn new(p1: Point, p2: Point) -> Self {
        let mid_x = (p1.x + p2.x) / 2.;
        let mid_y = (p1.y + p2.y) / 2.;

        let quadrants = [
            Rectangle::new(p1.clone(), Point::new(mid_x, mid_y)),
            Rectangle::new(Point::new(mid_x, p1.y), Point::new(p2.x, mid_y)),
            Rectangle::new(Point::new(p1.x, mid_y), Point::new(mid_x, p2.y)),
            Rectangle::new(Point::new(mid_x, mid_y), p2.clone()),
        ];
        Cell {
            coords: Rectangle::new(p1, p2),
            quadrants,
            quads_filled: [false, false, false, false],
        }
    }

    fn render_line(&mut self, line: &Line) {
        if !self.coords.overlaps_line(line) {
            return;
        }
        self.quads_filled[0] = self.quads_filled[0] || self.quadrants[0].overlaps_line(line);
        self.quads_filled[1] = self.quads_filled[1] || self.quadrants[1].overlaps_line(line);
        self.quads_filled[2] = self.quads_filled[2] || self.quadrants[2].overlaps_line(line);
        self.quads_filled[3] = self.quads_filled[3] || self.quadrants[3].overlaps_line(line);
    }

    fn render_circle(&mut self, circle: &Circle) {
        if !self.coords.overlaps_circle(circle) {
            return;
        }
        self.quads_filled[0] = self.quads_filled[0] || self.quadrants[0].overlaps_circle(circle);
        self.quads_filled[1] = self.quads_filled[1] || self.quadrants[1].overlaps_circle(circle);
        self.quads_filled[2] = self.quads_filled[2] || self.quadrants[2].overlaps_circle(circle);
        self.quads_filled[3] = self.quads_filled[3] || self.quadrants[3].overlaps_circle(circle);
    }

    fn render_ellipse(&mut self, ellipse: &Ellipse) {
        if !self.coords.overlaps_ellipse(ellipse) {
            return;
        }
        self.quads_filled[0] = self.quads_filled[0] || self.quadrants[0].overlaps_ellipse(ellipse);
        self.quads_filled[1] = self.quads_filled[1] || self.quadrants[1].overlaps_ellipse(ellipse);
        self.quads_filled[2] = self.quads_filled[2] || self.quadrants[2].overlaps_ellipse(ellipse);
        self.quads_filled[3] = self.quads_filled[3] || self.quadrants[3].overlaps_ellipse(ellipse);
    }

    fn print(&self, tileset: &[char; 16]) -> char {
        let q = self.quads_filled;

        let mut i = 0;
        if q[3] {
            i += 1;
        }
        if q[2] {
            i += 2;
        }
        if q[1] {
            i += 4;
        }
        if q[0] {
            i += 8;
        }

        tileset[i]
    }
}

#[derive(Debug)]
pub struct GridConfig {
    pub cell_width: usize,
    pub cell_height: usize,
    pub tileset: [char; 16],
    pub max_framerate: Option<usize>,
    pub print_timing: bool,
}

// if the quadrants were listed top-left, top-right, bottom-left, bottom-right,
// with a 1 for 'filled' and a `0` for 'unfilled', then each configuration of a
// cell would be some binary number < 16, e.g.
//
// - 1010 (10): top-left and bottom-left are filled in
// - 1101 (13): top-left, top-right and bottom-right are filled in
// - 0100 (4): only top-right is filled in
pub const PURE_ASCII: [char; 16] = [
    ' ',  // 0000
    '.',  // 0001
    ',',  // 0010
    '_',  // 0011
    '\'', // 0100
    ']',  // 0101
    '/',  // 0110
    'd',  // 0111
    '`',  // 1000
    '\\', // 1001
    '[',  // 1010
    'b',  // 1011
    '"',  // 1100
    '¶', //  1101
    'P',  // 1110
    '#',  // 1111
];

pub const BRAILLE: [char; 16] = [
    '\u{2800}', // 0000
    '\u{28a0}', // 0001
    '\u{2844}', // 0010
    '\u{28e4}', // 0011
    '\u{2818}', // 0100
    '\u{28b8}', // 0101
    '\u{285c}', // 0110
    '\u{28fc}', // 0111
    '\u{2803}', // 1000
    '\u{28a3}', // 1001
    '\u{2847}', // 1010
    '\u{28e7}', // 1011
    '\u{281b}', // 1100
    '\u{28bb}', // 1101
    '\u{285f}', // 1110
    '\u{28ff}', // 1111
];

pub trait Draw {
    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64);
    fn circle(&mut self, x: f64, y: f64, r: f64);
    fn ellipse(&mut self, x: f64, y: f64, a: f64, b: f64, keel: f64);
}

#[derive(Debug)]
pub struct Transform<'a> {
    grid: &'a mut Grid,
    angle: f64,
    angle_sin: f64,
    angle_cos: f64,
    x: f64,
    y: f64,
}

impl<'a> Transform<'a> {
    fn from(grid: &'a mut Grid) -> Transform<'a> {
        Transform {
            grid,
            angle: 0.,
            angle_sin: 0.,
            angle_cos: 1.,
            x: 0.,
            y: 0.,
        }
    }

    fn point(&self, x: f64, y: f64) -> Point {
        Point {
            x: x * self.angle_cos - y * self.angle_sin + self.x,
            y: x * self.angle_sin + y * self.angle_cos + self.y,
        }
    }

    pub fn rotate(&mut self, radians: f64) -> &mut Self {
        let new_angle = self.angle + radians;
        self.angle = new_angle;
        self.angle_sin = new_angle.sin();
        self.angle_cos = new_angle.cos();
        self
    }

    pub fn translate(&mut self, x: f64, y: f64) -> &mut Self {
        self.x += x;
        self.y += y;
        self
    }
}

impl<'a> Draw for Transform<'a> {
    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let p1 = self.point(x1, y1);
        let p2 = self.point(x2, y2);
        self.grid.line(p1.x, p1.y, p2.x, p2.y);
    }
    fn circle(&mut self, x: f64, y: f64, r: f64) {
        let p = self.point(x, y);
        self.grid.circle(p.x, p.y, r);
    }
    fn ellipse(&mut self, x: f64, y: f64, a: f64, b: f64, keel: f64) {
        let p = self.point(x, y);
        self.grid.ellipse(p.x, p.y, a, b, self.angle + keel);
    }
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<Cell>>,
    tileset: [char; 16],
    max_framerate: usize,
    print_timing: bool,
}

// put a _tiny_ bit of padding on the edges so lines at the edges register
const BUMPER: f64 = 0.00001;

impl Draw for Grid {
    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let line = Line::new(Point::new(x1, y1), Point::new(x2, y2));
        self.each_cell_mut(|cell| {
            cell.render_line(&line);
        });
    }

    fn circle(&mut self, x: f64, y: f64, r: f64) {
        let circle = Circle::new(Point::new(x, y), r);
        self.each_cell_mut(|cell| {
            cell.render_circle(&circle);
        });
    }

    fn ellipse(&mut self, x: f64, y: f64, a: f64, b: f64, keel: f64) {
        let ellipse = Ellipse::new(Point::new(x, y), a, b, keel);
        self.each_cell_mut(|cell| {
            cell.render_ellipse(&ellipse);
        });
    }
}

impl Grid {
    fn new(config: GridConfig) -> Grid {
        let cell_width = config.cell_width;
        let cell_height = config.cell_height;
        let x_unit = (100. + BUMPER) / cell_width as f64;
        let y_unit = (100. + BUMPER) / cell_height as f64;

        Grid {
            tileset: config.tileset,
            max_framerate: config.max_framerate.unwrap_or(20),
            print_timing: config.print_timing,
            grid: (0..cell_height)
                .map(|j| {
                    (0..cell_width)
                        .map(|i| {
                            let x = i as f64 * x_unit - BUMPER / 2.;
                            let y = j as f64 * y_unit - BUMPER / 2.;
                            Cell::new(Point::new(x, y), Point::new(x + x_unit, y + y_unit))
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn transform(&mut self) -> Transform {
        Transform::from(self)
    }

    pub fn with_transform<F>(&mut self, f: F)
    where
        F: Fn(Transform) -> (),
    {
        f(Transform::from(self))
    }

    fn each_cell_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut Cell) -> (),
    {
        self.grid
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|cell| f(cell)));
    }

    fn clear(&mut self) {
        self.each_cell_mut(|cell| {
            cell.quads_filled[0] = false;
            cell.quads_filled[1] = false;
            cell.quads_filled[2] = false;
            cell.quads_filled[3] = false;
        });
    }

    fn print(&self) {
        self.grid.iter().for_each(|row| {
            println!(
                "{}",
                row.iter()
                    .map(|cell| cell.print(&self.tileset))
                    .collect::<String>()
            )
        })
    }
}

fn sleep_less(subtract_amount: usize, millis: usize) {
    if subtract_amount >= millis {
        return;
    }
    std::thread::sleep(std::time::Duration::from_millis(
        (millis - subtract_amount) as u64,
    ));
}

const TIMING_SIZE: usize = 50;

fn print_average(frame: usize, arr: &[u128; TIMING_SIZE]) {
    if frame > TIMING_SIZE {
        println!(
            "average time to paint (over {} frames): {}ms                       ",
            TIMING_SIZE,
            arr.iter().sum::<u128>() / TIMING_SIZE as u128
        );
    } else {
        println!(
            "average time to paint (over {} frames): calculating...             ",
            TIMING_SIZE,
        );
    }
}

pub fn draw<F>(config: GridConfig, draw_fn: F)
where
    F: Fn(&mut Grid, usize) -> (),
{
    let mut timing: [u128; TIMING_SIZE] = [4; TIMING_SIZE];
    let mut grid = Grid::new(config);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    for frame in 0.. {
        let now = std::time::Instant::now();

        draw_fn(&mut grid, frame);
        print!("{}[1;1H", 27 as char);
        grid.print();
        grid.clear();

        let spent = now.elapsed().as_millis();
        if grid.print_timing {
            timing[frame % TIMING_SIZE] = spent;
            print_average(frame, &timing);
        }
        print!("                         ");
        sleep_less(spent as usize, 1000 / grid.max_framerate);
    }
}
