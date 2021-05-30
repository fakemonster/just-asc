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

#[derive(Debug)]
struct Cell {
    coords: (Point, Point),
    quads_filled: [bool; 4],
}

impl Cell {
    fn new(p1: Point, p2: Point) -> Self {
        Cell {
            coords: (p1, p2),
            quads_filled: [false, false, false, false],
        }
    }
}

#[derive(Debug)]
struct GridConfig {
    width: f64,
    height: f64,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Cell>>,
    config: GridConfig,
}

impl Grid {
    fn new(config: GridConfig) -> Grid {
        let cell_width = 72;
        let cell_height = 45;
        let x_unit = config.width / cell_width as f64;
        let y_unit = config.height / cell_height as f64;

        Grid {
            config,
            grid: (0..cell_height)
                .map(|j| {
                    (0..cell_width)
                        .map(|i| {
                            let x = i as f64 * x_unit;
                            let y = j as f64 * y_unit;
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

    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        render(self, &(Point::new(x1, y1), Point::new(x2, y2)));
    }
}

fn lines_intersect(line1: &(Point, Point), line2: &(Point, Point)) -> bool {
    let (Point { x: a, y: b }, Point { x: c, y: d }) = line1;
    let (Point { x: p, y: q }, Point { x: r, y: s }) = line2;
    let det = (c - a) * (s - q) - (r - p) * (d - b);
    if det == 0. {
        return false;
    }
    let lambda = ((s - q) * (r - a) + (p - r) * (s - b)) / det;
    let gamma = ((b - d) * (r - a) + (c - a) * (s - b)) / det;

    0. < lambda && lambda < 1. && 0. < gamma && gamma < 1.
}

fn top_line(coords: &(Point, Point)) -> (Point, Point) {
    (coords.0.clone(), Point::new(coords.1.x, coords.0.y))
}

fn right_line(coords: &(Point, Point)) -> (Point, Point) {
    (Point::new(coords.1.x, coords.0.y), coords.1.clone())
}

fn bottom_line(coords: &(Point, Point)) -> (Point, Point) {
    (Point::new(coords.0.x, coords.1.y), coords.1.clone())
}

fn left_line(coords: &(Point, Point)) -> (Point, Point) {
    (coords.0.clone(), Point::new(coords.0.x, coords.1.y))
}

fn intersects(coords: &(Point, Point), line: &(Point, Point)) -> bool {
    lines_intersect(&top_line(coords), line)
        || lines_intersect(&right_line(coords), line)
        || lines_intersect(&bottom_line(coords), line)
        || lines_intersect(&left_line(coords), line)
}

fn get_quadrants(coords: &(Point, Point)) -> [(Point, Point); 4] {
    let (top_left, bottom_right) = coords;
    let mid_x = (top_left.x + bottom_right.x) / 2.;
    let mid_y = (top_left.y + bottom_right.y) / 2.;

    [
        (top_left.clone(), Point::new(mid_x, mid_y)),
        (
            Point::new(mid_x, top_left.y),
            Point::new(bottom_right.x, mid_y),
        ),
        (
            Point::new(top_left.x, mid_y),
            Point::new(mid_x, bottom_right.y),
        ),
        (Point::new(mid_x, mid_y), bottom_right.clone()),
    ]
}

fn render_line(cell: &mut Cell, line: &(Point, Point)) {
    let coords = &cell.coords;
    if !intersects(coords, line) {
        return;
    }
    let quadrants = get_quadrants(coords);
    cell.quads_filled[0] = cell.quads_filled[0] || intersects(&quadrants[0], line);
    cell.quads_filled[1] = cell.quads_filled[1] || intersects(&quadrants[1], line);
    cell.quads_filled[2] = cell.quads_filled[2] || intersects(&quadrants[2], line);
    cell.quads_filled[3] = cell.quads_filled[3] || intersects(&quadrants[3], line);
}

fn print_cell(cell: &Cell) -> char {
    let q = cell.quads_filled;

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

fn render(grid: &mut Grid, line: &(Point, Point)) {
    grid.each_cell_mut(|mut cell| {
        render_line(&mut cell, &line);
    });
}

fn print(grid: &Grid) {
    grid.grid
        .iter()
        .for_each(|row| println!("{}", row.iter().map(print_cell).collect::<String>()))
}

fn sleep_less(subtract_amount: u64, millis: u64) {
    if subtract_amount >= millis {
        return;
    }
    std::thread::sleep(std::time::Duration::from_millis(millis - subtract_amount));
}

fn draw<F>(draw_fn: F)
where
    F: Fn(&mut Grid, usize) -> (),
{
    let mut grid = Grid::new(GridConfig {
        width: 100.,
        height: 100.,
    });
    for frame in 0.. {
        let now = std::time::Instant::now();

        draw_fn(&mut grid, frame);
        print!("{}[2J", 27 as char);
        print(&grid);
        grid.clear();

        let spent = now.elapsed().as_millis();
        println!("time per frame: {}ms", spent);
        sleep_less(spent as u64, 60);
    }
}

fn main() {
    draw(|grid, frame| {
        let angle = (2. * 3.14159 / 60.) * frame as f64;
        let x = angle.cos();
        let y = angle.sin();
        grid.line(
            50. - (x * 5.),
            50. - (y * 5.),
            50. + (x * 30.),
            50. + (y * 30.),
        );
        grid.line(
            50. - (y * 3.),
            50. - (x * 3.),
            50. + (y * 10.),
            50. + (x * 10.),
        );
        grid.line(
            70. - (y * 5.),
            30. - (x * 5.),
            20. + (y * 10.),
            30. + (x * 10.),
        );
    });
}
