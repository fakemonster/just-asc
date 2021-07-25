extern crate just_asc;
use std::f64::consts::PI;

fn borders(grid: &mut just_asc::Grid) {
    grid.line(0., 0., 100., 0.);
    grid.line(100., 0., 100., 100.);
    grid.line(100., 100., 0., 100.);
    grid.line(0., 0., 0., 100.);
}

fn spinning_triangle(grid: &mut just_asc::Grid, cycle: f64, height: f64) {
    let side_length = 2. * height / (3_f64.sqrt());
    grid.translate(50., 50.);
    grid.rotate(cycle);

    let x1 = 0.;
    let y1 = -2. * height / 3.;
    let x2 = side_length / 2.;
    let y2 = height / 3.;
    let x3 = -side_length / 2.;
    let y3 = height / 3.;

    grid.line(x1, y1, x2, y2);
    grid.line(x2, y2, x3, y3);
    grid.line(x3, y3, x1, y1);
    grid.clear_transform();
}

fn triangle_stuff(grid: &mut just_asc::Grid, frame: usize) {
    let big_cycle = -(2. * PI / 320.) * (frame + 240) as f64;
    let medium_cycle = -(2. * PI / 240.) * (frame + 180) as f64;
    let small_cycle = -(2. * PI / 160.) * (frame + 120) as f64;

    spinning_triangle(grid, big_cycle, 43.3);
    spinning_triangle(grid, medium_cycle, 21.65);
    spinning_triangle(grid, small_cycle, 10.825);
}

fn circle_stuff(grid: &mut just_asc::Grid, frame: usize) {
    grid.translate(50., 50.);
    grid.rotate(frame as f64 * 2. * PI / 240.);
    for _ in 0..12 {
        grid.rotate(2. * PI / 12.);
        grid.circle(0., -42., 8.);
    }
    grid.clear_transform();
}

fn spinner(grid: &mut just_asc::Grid, initial_angle: f64) {
    grid.ellipse(0., 0., 18., 6., initial_angle);
    grid.ellipse(0., 0., 18., 6., initial_angle - PI / 4.);
    grid.ellipse(0., 0., 18., 6., initial_angle - PI / 2.);
    grid.ellipse(0., 0., 18., 6., initial_angle - (3. / 4.) * PI);
}

fn ellipses(grid: &mut just_asc::Grid, frame: usize) {
    let slow = (2. * PI / 80.) * (frame + 75) as f64;

    grid.translate(3., 3.);
    spinner(grid, slow);
    grid.clear_transform();

    grid.translate(3., 97.);
    spinner(grid, slow);
    grid.clear_transform();

    grid.translate(97., 3.);
    spinner(grid, slow);
    grid.clear_transform();

    grid.translate(97., 97.);
    spinner(grid, slow);
    grid.clear_transform();
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
