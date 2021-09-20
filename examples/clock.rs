extern crate just_asc;
use crate::just_asc::Draw;
use std::f64::consts::PI;

fn main() {
    just_asc::draw(
        just_asc::DEFAULT_CONFIG,
        |grid: &mut just_asc::Grid, frame: usize| {
            grid.circle(50., 50., 50.);

            grid.with_transform(|mut transform| {
                transform.translate(50., 50.);
                transform.rotate(PI / 20. * frame as f64);
                transform.line(0., 10., 0., -40.);
            });

            grid.with_transform(|mut transform| {
                transform.translate(50., 50.);
                transform.rotate(PI / 240. * frame as f64);
                transform.line(0., 5., 0., -20.);
            });
        },
    );
}
