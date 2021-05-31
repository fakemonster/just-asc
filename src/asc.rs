#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

// a pair of points defining a line segment, with some precomputed values
#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
    dx: f64,
    dy: f64,
    hypotenuse_squared: f64,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let hypotenuse_squared = dx.powf(2.) + dy.powf(2.);
        Line {
            start,
            end,
            dx,
            dy,
            hypotenuse_squared,
        }
    }

    fn intersects_line(&self, other: &Line) -> bool {
        let Point { x: a, y: b } = self.start;
        let Point { x: r, y: s } = other.end;
        let det = (self.dx) * (other.dy) - (other.dx) * (self.dy);
        if det == 0. {
            return false;
        }
        let lambda = (other.dy * (r - a) - other.dx * (s - b)) / det;
        let gamma = (self.dx * (s - b) - self.dy * (r - a)) / det;

        0. < lambda && lambda < 1. && 0. < gamma && gamma < 1.
    }

    fn intersects_circle(&self, circle: &Circle) -> bool {
        let ax = self.start.x - circle.center.x;
        let ay = self.start.y - circle.center.y;
        let a = self.hypotenuse_squared;
        let b = 2. * (ax * self.dx + ay * self.dy);
        let c = ax.powf(2.) + ay.powf(2.) - circle.r_squared;
        let disc = b.powf(2.) - 4. * a * c;
        if disc <= 0. {
            return false;
        }
        let sqrt_disc = disc.sqrt();
        let t1 = (-b + sqrt_disc) / (2. * a);
        let t2 = (-b - sqrt_disc) / (2. * a);

        (0. < t1 && t1 < 1.) || (0. < t2 && t2 < 1.)
    }
}

// a pair of points defining the bounds, with some precomputed values
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
    top: Line,
    right: Line,
    bottom: Line,
    left: Line,
}

impl Rectangle {
    fn new(top_left: Point, bottom_right: Point) -> Self {
        let top = Line::new(top_left.clone(), Point::new(bottom_right.x, top_left.y));
        let right = Line::new(Point::new(bottom_right.x, top_left.y), bottom_right.clone());
        let bottom = Line::new(Point::new(top_left.x, bottom_right.y), bottom_right.clone());
        let left = Line::new(top_left.clone(), Point::new(top_left.x, bottom_right.y));

        Rectangle {
            top_left,
            bottom_right,
            top,
            right,
            bottom,
            left,
        }
    }

    fn overlaps_line(&self, line: &Line) -> bool {
        self.top.intersects_line(line)
            || self.right.intersects_line(line)
            || self.bottom.intersects_line(line)
            || self.left.intersects_line(line)
    }

    fn overlaps_circle(&self, circle: &Circle) -> bool {
        self.top.intersects_circle(circle)
            || self.right.intersects_circle(circle)
            || self.bottom.intersects_circle(circle)
            || self.left.intersects_circle(circle)
    }
}

#[derive(Debug)]
struct Circle {
    center: Point,
    r: f64,
    r_squared: f64,
}

impl Circle {
    fn new(x: f64, y: f64, r: f64) -> Circle {
        let r_squared = r.powf(2.);
        Circle {
            center: Point::new(x, y),
            r,
            r_squared,
        }
    }
}

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

    fn print(&self) -> char {
        let q = self.quads_filled;

        // 1: ,.
        //    '`
        // 2: "_
        //    []
        //    \/
        // 3: P¶
        //    bd
        // 4:  #

        match q[0] {
            true => match q[1] {
                true => match q[2] {
                    true => match q[3] {
                        true => '#',  // 1111
                        false => 'P', // 1110
                    },
                    false => match q[3] {
                        true => '¶', // 1101
                        false => '"', // 1100
                    },
                },
                false => match q[2] {
                    true => match q[3] {
                        true => 'b',  // 1011
                        false => '[', // 1010
                    },
                    false => match q[3] {
                        true => '\\', // 1001
                        false => '`', // 1000
                    },
                },
            },
            false => match q[1] {
                true => match q[2] {
                    true => match q[3] {
                        true => 'd',  // 0111
                        false => '/', // 0110
                    },
                    false => match q[3] {
                        true => ']',   // 0101
                        false => '\'', // 0100
                    },
                },
                false => match q[2] {
                    true => match q[3] {
                        true => '_',  // 0011
                        false => ',', // 0010
                    },
                    false => match q[3] {
                        true => '.',  // 0001
                        false => ' ', // 0000
                    },
                },
            },
        }
    }
}

#[derive(Debug)]
struct GridConfig {
    width: f64,
    height: f64,
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<Cell>>,
    config: GridConfig,
}

// put a _tiny_ bit of padding on the edges so lines at the edges register
const BUMPER: f64 = 0.00001;

impl Grid {
    fn new(config: GridConfig) -> Grid {
        let cell_width = 72;
        let cell_height = 45;
        let x_unit = (config.width + BUMPER) / cell_width as f64;
        let y_unit = (config.height + BUMPER) / cell_height as f64;

        Grid {
            config,
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

    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let line = Line::new(Point::new(x1, y1), Point::new(x2, y2));
        self.each_cell_mut(|cell| {
            cell.render_line(&line);
        });
    }

    pub fn circle(&mut self, x: f64, y: f64, r: f64) {
        let circle = Circle::new(x, y, r);
        self.each_cell_mut(|cell| {
            cell.render_circle(&circle);
        });
    }

    fn print(&self) {
        self.grid
            .iter()
            .for_each(|row| println!("{}", row.iter().map(Cell::print).collect::<String>()))
    }
}

fn sleep_less(subtract_amount: u64, millis: u64) {
    if subtract_amount >= millis {
        return;
    }
    std::thread::sleep(std::time::Duration::from_millis(millis - subtract_amount));
}

pub fn draw<F>(draw_fn: F)
where
    F: Fn(&mut Grid, usize) -> (),
{
    let mut grid = Grid::new(GridConfig {
        width: 100.,
        height: 100.,
    });
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    for frame in 0.. {
        let now = std::time::Instant::now();

        draw_fn(&mut grid, frame);
        print!("{}[1;1H", 27 as char);
        grid.print();
        grid.clear();

        let spent = now.elapsed().as_millis();
        println!("time per frame: {}ms                       ", spent);
        print!("                         ");
        sleep_less(spent as u64, 50);
    }
}
