mod asc;

fn main() {
    asc::draw(|grid, frame| {
        let angle = (2. * 3.14159 / 60.) * frame as f64;
        let x = angle.cos();
        let y = angle.sin();
        let slow_angle = (2. * 3.14159 / 120.) * frame as f64;
        let slow_x = slow_angle.cos();
        let slow_y = slow_angle.sin();
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
        grid.circle(50. + (slow_x * 10.), 50. + (slow_y * 10.), 10.);
    });
}
