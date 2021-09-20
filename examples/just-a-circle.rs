extern crate just_asc;
use crate::just_asc::Draw;
use std::f64::consts::PI;

fn main() {
    let config = just_asc::GridConfig {
        cell_width: 30,
        cell_height: 15,
        tileset: just_asc::tilesets::PURE_ASCII,
        max_framerate: Some(1),
        print_timing: false,
    };

    just_asc::once(config, |grid: &mut just_asc::Grid| {
        grid.with_transform(|mut transform| {
            transform.translate(50., 50.);
            transform.circle(0., 0., 50.);
            transform.rotate(PI / 4.);
            transform.line(-25., 0., 25., 0.);
            transform.rotate(PI / 2.);
            transform.line(-25., 0., 25., 0.);
        });
    });
}