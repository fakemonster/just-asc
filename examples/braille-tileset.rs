extern crate just_asc;
use std::f64::consts::PI;

fn borders(grid: &mut just_asc::Grid) {
    grid.line(0., 0., 100., 0.);
    grid.line(100., 0., 100., 100.);
    grid.line(100., 100., 0., 100.);
    grid.line(0., 0., 0., 100.);
}

fn spinning_triangle(grid: &mut just_asc::Grid, cycle: f64, x: f64, y: f64, height: f64) {
    let side_length = 2. * height / (3_f64.sqrt());

    let x1 = x + (cycle.cos() * side_length);
    let y1 = y + (cycle.sin() * side_length);
    let x2 = x + ((cycle - 2. * PI / 3.).cos() * side_length);
    let y2 = y + ((cycle - 2. * PI / 3.).sin() * side_length);
    let x3 = x + ((cycle - 4. * PI / 3.).cos() * side_length);
    let y3 = y + ((cycle - 4. * PI / 3.).sin() * side_length);

    grid.line(x1, y1, x2, y2);
    grid.line(x2, y2, x3, y3);
    grid.line(x3, y3, x1, y1);
}

fn triangle_stuff(grid: &mut just_asc::Grid, frame: usize) {
    let big_cycle = -(2. * PI / 320.) * (frame + 240) as f64;
    let medium_cycle = -(2. * PI / 240.) * (frame + 180) as f64;
    let small_cycle = -(2. * PI / 160.) * (frame + 120) as f64;

    spinning_triangle(grid, big_cycle, 50., 50., 25.);
    spinning_triangle(grid, medium_cycle, 50., 50., 12.5);
    spinning_triangle(grid, small_cycle, 50., 50., 6.25);
}

fn circle_stuff(grid: &mut just_asc::Grid, frame: usize) {
    for i in 0..12 {
        let cycle = (2. * PI / 240.) * (frame + (20 * i)) as f64;
        let x = cycle.cos();
        let y = cycle.sin();

        grid.circle(50. + (x * 42.), 50. + (y * 42.), 8.);
    }
}

fn ellipses(grid: &mut just_asc::Grid, frame: usize) {
    let slow = (2. * PI / 80.) * (frame + 75) as f64;

    grid.ellipse(3., 3., 18., 6., slow);
    grid.ellipse(3., 3., 18., 6., slow - PI / 4.);
    grid.ellipse(3., 3., 18., 6., slow - PI / 2.);
    grid.ellipse(3., 3., 18., 6., slow - (3. / 4.) * PI);

    grid.ellipse(3., 97., 18., 6., slow);
    grid.ellipse(3., 97., 18., 6., slow - PI / 4.);
    grid.ellipse(3., 97., 18., 6., slow - PI / 2.);
    grid.ellipse(3., 97., 18., 6., slow - (3. / 4.) * PI);

    grid.ellipse(97., 3., 18., 6., slow);
    grid.ellipse(97., 3., 18., 6., slow - PI / 4.);
    grid.ellipse(97., 3., 18., 6., slow - PI / 2.);
    grid.ellipse(97., 3., 18., 6., slow - (3. / 4.) * PI);

    grid.ellipse(97., 97., 18., 6., slow);
    grid.ellipse(97., 97., 18., 6., slow - PI / 4.);
    grid.ellipse(97., 97., 18., 6., slow - PI / 2.);
    grid.ellipse(97., 97., 18., 6., slow - (3. / 4.) * PI);
}

fn main() {
    let config = just_asc::GridConfig {
        cell_width: 72,
        cell_height: 36,
        tileset: just_asc::BRAILLE,
        max_framerate: Some(50),
    };

    just_asc::draw(config, |grid: &mut just_asc::Grid, frame: usize| {
        borders(grid);
        triangle_stuff(grid, frame);
        circle_stuff(grid, frame);
        ellipses(grid, frame);
    });
}
