extern crate just_asc;
use crate::just_asc::Draw;
use std::f64::consts::PI;

fn borders(grid: &mut just_asc::Grid) {
    grid.line(0., 0., 100., 0.);
    grid.line(100., 0., 100., 100.);
    grid.line(100., 100., 0., 100.);
    grid.line(0., 0., 0., 100.);
}

fn spinning_triangle(grid: &mut just_asc::Grid, cycle: f64, height: f64) {
    let side_length = 2. * height / (3_f64.sqrt());
    let mut transform = grid.transform();
    transform.translate(50., 50.).rotate(cycle);

    let x1 = 0.;
    let y1 = -2. * height / 3.;
    let x2 = side_length / 2.;
    let y2 = height / 3.;
    let x3 = -side_length / 2.;
    let y3 = height / 3.;

    transform.line(x1, y1, x2, y2);
    transform.line(x2, y2, x3, y3);
    transform.line(x3, y3, x1, y1);
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
    let mut transform = grid.transform();
    transform
        .translate(50., 50.)
        .rotate(frame as f64 * 2. * PI / 240.);
    for _ in 0..12 {
        transform.rotate(2. * PI / 12.).circle(0., -42., 8.);
    }
}

fn spinner(grid: &mut just_asc::Grid, x_offset: f64, y_offset: f64, initial_angle: f64) {
    grid.with_transform(|mut transform| {
        transform.translate(x_offset, y_offset);
        transform.ellipse(0., 0., 18., 6., initial_angle);
        transform.ellipse(0., 0., 18., 6., initial_angle - PI / 4.);
        transform.ellipse(0., 0., 18., 6., initial_angle - PI / 2.);
        transform.ellipse(0., 0., 18., 6., initial_angle - (3. / 4.) * PI);
    });
}

fn ellipses(grid: &mut just_asc::Grid, frame: usize) {
    let slow = (2. * PI / 80.) * (frame + 75) as f64;

    spinner(grid, 3., 3., slow);
    spinner(grid, 3., 97., slow);
    spinner(grid, 97., 3., slow);
    spinner(grid, 97., 97., slow);
}

fn main() {
    let config = just_asc::GridConfig {
        cell_width: 72,
        cell_height: 36,
        tileset: just_asc::tilesets::BRAILLE,
        max_framerate: Some(50),
        print_timing: true,
    };

    just_asc::draw(config, |grid: &mut just_asc::Grid, frame: usize| {
        borders(grid);
        triangle_stuff(grid, frame);
        circle_stuff(grid, frame);
        ellipses(grid, frame);
    });
}
