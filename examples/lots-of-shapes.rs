extern crate just_asc;
use std::f64::consts::PI;

fn offset_x_line(grid: &mut just_asc::Grid, offset: f64, x1: f64, y1: f64, x2: f64, y2: f64) {
    grid.line(x1 + offset, y1, x2 + offset, y2);
}

fn sliding_angles(grid: &mut just_asc::Grid, frame: usize) {
    let angle = (2. * PI / 50.) * frame as f64;
    let offset = angle.cos() * 4.;

    offset_x_line(grid, offset, 40., 3., 60., 3.);

    offset_x_line(grid, offset, 15., 5., 1.5, 8.);
    offset_x_line(grid, offset, 20., 5., 8., 9.);
    offset_x_line(grid, offset, 25., 5., 12.5, 10.);
    offset_x_line(grid, offset, 30., 5., 18., 11.);
    offset_x_line(grid, offset, 35., 5., 24.5, 12.);
    offset_x_line(grid, offset, 40., 5., 32., 13.);
    offset_x_line(grid, offset, 45., 5., 41., 14.);
    offset_x_line(grid, offset, 50., 5., 50., 15.);
    offset_x_line(grid, offset, 55., 5., 59., 14.);
    offset_x_line(grid, offset, 60., 5., 68., 13.);
    offset_x_line(grid, offset, 65., 5., 75.5, 12.);
    offset_x_line(grid, offset, 70., 5., 82., 11.);
    offset_x_line(grid, offset, 75., 5., 87.5, 10.);
    offset_x_line(grid, offset, 80., 5., 92., 9.);
    offset_x_line(grid, offset, 85., 5., 98.5, 8.);
}

fn spinning_lines(grid: &mut just_asc::Grid, frame: usize) {
    let angle = (2. * PI / 60.) * frame as f64;
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
}

fn circle_stuff(grid: &mut just_asc::Grid, frame: usize) {
    let slow = (2. * PI / 120.) * frame as f64;
    let x = slow.cos();
    let y = slow.sin();

    grid.circle(50. + (x * 10.), 50. + (y * 10.), 10.);
    grid.circle(50. + (x * 5.), 85., 10. + (x * 10.));
    grid.circle(50. + (x * 5.), 85., 10. + (y * 10.));
}

fn ellipses(grid: &mut just_asc::Grid, frame: usize) {
    let slow = (2. * PI / 80.) * frame as f64;
    let x = slow.cos();

    grid.ellipse(20., 70., 10. - (8. * x), 10. + 8. * x, 0.);
    grid.ellipse(20., 70., 10. - (8. * x), 10. + 8. * x, PI / 4.);
    grid.ellipse(20., 70., 4., 6., slow);

    grid.ellipse(80., 30., 12., 6., slow);
    grid.ellipse(80., 30., 12., 6., slow - PI / 4.);
    grid.ellipse(80., 30., 12., 6., slow - PI / 2.);
    grid.ellipse(80., 30., 12., 6., slow - (3. / 4.) * PI);
}

fn main() {
    let config = just_asc::GridConfig {
        cell_width: 72,
        cell_height: 36,
        tileset: just_asc::PURE_ASCII,
    };

    just_asc::draw(config, |grid: &mut just_asc::Grid, frame: usize| {
        sliding_angles(grid, frame);
        spinning_lines(grid, frame);
        circle_stuff(grid, frame);
        ellipses(grid, frame);
    });
}