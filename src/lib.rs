#![warn(missing_docs)]
//! Have some fun, draw in your terminal! Or someone else's.
//!
//! This library provides a "canvas" for you to draw in, rendered in ASCII. Uses might include
//! loading animations in CLI tools, or dorky projector backdrops at your party.
//!
//! ```
//! use std::f64::consts::PI;
//!
//! let config = just_asc::GridConfig {
//!     cell_width: 40,
//!     cell_height: 20,
//!     tileset: just_asc::tilesets::PURE_ASCII,
//!     max_framerate: Some(1),
//!     print_timing: false,
//! };
//!
//! just_asc::draw(config, |grid: &mut just_asc::Grid, _frame: usize| {
//!     grid.with_transform(|mut transform| {
//!         transform.translate(50., 50.);
//!         transform.circle(0., 0., 50.);
//!         transform.rotate(PI / 4.);
//!         transform.line(-25., 0., 25., 0.);
//!         transform.rotate(PI / 2.);
//!         transform.line(-25., 0., 25., 0.);
//!     });
//! });
//! ```

mod shapes;
pub mod tilesets;

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

/// The configuration used when instantiating a grid. A few configurations about "cells" are
/// present: a cell represents one character on the rendered grid.
#[derive(Debug)]
pub struct GridConfig {
    /// The width of your canvas (# of characters).
    pub cell_width: usize,

    /// The height of your canvas (# of characters).
    pub cell_height: usize,

    /// A list of characters to use based on quadrants of a cell filled.
    ///
    /// If the quadrants were listed top-left, top-right, bottom-left, bottom-right, with a 1 for
    /// 'filled' and a `0` for 'unfilled', then each configuration of a cell would be some binary
    /// number < 16, e.g.
    ///
    /// - 1010 (10): top-left and bottom-left are filled in. In PURE_ASCII that looks like `[`.
    /// - 1101 (13): top-left, top-right and bottom-right are filled in. In PURE_ASCII that looks
    /// like `¶`.
    /// - 0100 (4): only top-right is filled in. In PURE_ASCII that looks like `'`.
    pub tileset: [char; 16],

    /// The maximum frames per second your grid will render. Defaults to 20.
    pub max_framerate: Option<usize>,

    /// Print a debug statement that displays a rolling average of how long it takes to render a
    /// frame.
    pub print_timing: bool,
}

/// The `Draw` trait defines the actual shapes you can add to your canvas. This trait is present on
/// both `Grid` and `Transform`.
pub trait Draw {
    /// `line` draws a line on your `Grid`! the first two arguments are your starting x and y, the
    /// latter two arguments are your ending x and y.
    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64);

    /// `ellipse` draws ellipses. The first two parameters are the position of its center, followed
    /// by its x axis length and y axis length. The last parameter is its "keel", i.e. its rotation
    /// around the center (in radians). Set to `0` if you don't want to rotate! Or PI, if you're
    /// feeling spicy.
    fn ellipse(&mut self, x: f64, y: f64, a: f64, b: f64, keel: f64);

    /// `circle` draws a circle, where the first two parameters are the position of its center, and
    /// the last is its radius. You could accomplish this with `ellipse`, but using `circle` will
    /// give a slight performance boost.
    fn circle(&mut self, x: f64, y: f64, r: f64);
}

#[derive(Debug)]
/// Transforms are grid "wrappers" that keep a mutable reference to the main Grid. The trick is
/// that they can be freely rotated and translated (moved left-right-up-down). This makes it _much_
/// easier to draw groups of things that spin, orbit, or travel.
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

    /// Rotates a transform (in radians).
    pub fn rotate(&mut self, radians: f64) -> &mut Self {
        let new_angle = self.angle + radians;
        self.angle = new_angle;
        self.angle_sin = new_angle.sin();
        self.angle_cos = new_angle.cos();
        self
    }

    /// Moves a transform along x and y axes.
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
/// A Grid is what you draw on. Rather than acting directly on cells, a Grid gives you a 100x100
/// canvas, with (0,0) in the top-left corner.
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

    /// Creates a Transform from a Grid. Transform takes a mutable reference, so you won't be able
    /// to use the Grid while the Transform is in scope.
    pub fn transform(&mut self) -> Transform {
        Transform::from(self)
    }

    /// Takes a closure which _receives_ a Transform. This is a convenience function to keep the
    /// scopes of a Transform nice and clear.
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

/// Our core function. This kicks off an unending drawing, taking two arguments:
///
/// 1. a GridConfig
/// 2. a drawing closure, which will receive a fresh grid (drawings are erased every frame), and
///    the current frame count.
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
