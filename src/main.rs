mod asc;

fn main() {
    asc::draw(|grid, frame| {
        let angle = (2. * 3.14159 / 60.) * frame as f64;
        let angle_x = angle.cos();
        let angle_y = angle.sin();
        let slow = (2. * 3.14159 / 120.) * frame as f64;
        let slow_x = slow.cos();
        let slow_y = slow.sin();

        grid.line(0., 0., 100., 0.);
        grid.line(100., 0., 100., 100.);
        grid.line(100., 100., 0., 100.);
        grid.line(0., 0., 0., 100.);

        grid.line(
            50. - (angle_x * 5.),
            50. - (angle_y * 5.),
            50. + (angle_x * 30.),
            50. + (angle_y * 30.),
        );
        grid.line(
            50. - (angle_y * 3.),
            50. - (angle_x * 3.),
            50. + (angle_y * 10.),
            50. + (angle_x * 10.),
        );
        grid.line(
            70. - (angle_y * 5.),
            30. - (angle_x * 5.),
            20. + (angle_y * 10.),
            30. + (angle_x * 10.),
        );
        grid.circle(50. + (slow_x * 10.), 50. + (slow_y * 10.), 10.);
        grid.circle(50. + (slow_x * 5.), 85., 10. + (slow_x * 10.));
    });
}
