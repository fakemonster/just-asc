#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    fn rotate(&self, axis: &Point, keel: f64) -> Point {
        let s = keel.sin();
        let c = keel.cos();
        let tx = self.x - axis.x;
        let ty = self.y - axis.y;

        Point {
            x: tx * c - ty * s + axis.x,
            y: tx * s + ty * c + axis.y,
        }
    }
}

// a pair of points defining a line segment, with some precomputed values
#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
    dx: f64,
    dy: f64,
    length: f64,
    length_squared: f64,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let length_squared = dx.powf(2.) + dy.powf(2.);
        Line {
            start,
            end,
            dx,
            dy,
            length: length_squared.sqrt(),
            length_squared,
        }
    }

    fn rotate(&self, axis: &Point, keel: f64) -> Self {
        let start = self.start.rotate(axis, keel);
        let end = self.end.rotate(axis, keel);

        Line::new(start, end)
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

    fn intersects_ellipse(&self, ellipse: &Ellipse) -> bool {
        let ex = self.start.x - ellipse.center.x;
        let ey = self.start.y - ellipse.center.y;
        let e_dist = (ex.powf(2.) + ey.powf(2.)).sqrt();

        if e_dist > self.length + ellipse.max_axis {
            return false;
        }

        let slf = self.rotate(&ellipse.center, ellipse.keel);
        let ax = slf.start.x - ellipse.center.x;
        let ay = slf.start.y - ellipse.center.y;

        let a = (slf.dx.powf(2.) / ellipse.a_squared) + (slf.dy.powf(2.) / ellipse.b_squared);
        let b = 2. * ((ax * slf.dx / ellipse.a_squared) + (ay * slf.dy / ellipse.b_squared));
        let c = ax.powf(2.) / ellipse.a_squared + ay.powf(2.) / ellipse.b_squared - 1.;
        let discriminant = b.powf(2.) - 4. * a * c;
        if discriminant < 0. {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b + sqrt_discriminant) / (2. * a);
        let t2 = (-b - sqrt_discriminant) / (2. * a);

        (0. < t1 && t1 < 1.) || (0. < t2 && t2 < 1.)
    }

    fn intersects_circle(&self, circle: &Circle) -> bool {
        let ax = self.start.x - circle.center.x;
        let ay = self.start.y - circle.center.y;

        let a = self.length_squared;
        let b = 2. * (ax * self.dx + ay * self.dy);
        let c = ax.powf(2.) + ay.powf(2.) - circle.r_squared;
        let discriminant = b.powf(2.) - 4. * a * c;
        if discriminant < 0. {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b + sqrt_discriminant) / (2. * a);
        let t2 = (-b - sqrt_discriminant) / (2. * a);

        (0. < t1 && t1 < 1.) || (0. < t2 && t2 < 1.)
    }
}

// a pair of points defining the bounds, with some precomputed values
#[derive(Debug)]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
    top: Line,
    right: Line,
    bottom: Line,
    left: Line,
}

impl Rectangle {
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
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

    pub fn overlaps_line(&self, line: &Line) -> bool {
        self.top.intersects_line(line)
            || self.right.intersects_line(line)
            || self.bottom.intersects_line(line)
            || self.left.intersects_line(line)
    }

    pub fn overlaps_circle(&self, circle: &Circle) -> bool {
        self.top.intersects_circle(circle)
            || self.right.intersects_circle(circle)
            || self.bottom.intersects_circle(circle)
            || self.left.intersects_circle(circle)
    }

    pub fn overlaps_ellipse(&self, ellipse: &Ellipse) -> bool {
        self.top.intersects_ellipse(ellipse)
            || self.right.intersects_ellipse(ellipse)
            || self.bottom.intersects_ellipse(ellipse)
            || self.left.intersects_ellipse(ellipse)
    }
}

#[derive(Debug)]
pub struct Circle {
    center: Point,
    r_squared: f64,
}

impl Circle {
    pub fn new(center: Point, r: f64) -> Circle {
        Circle {
            center,
            r_squared: r.powf(2.),
        }
    }
}

#[derive(Debug)]
pub struct Ellipse {
    center: Point,
    max_axis: f64,
    a_squared: f64,
    b_squared: f64,
    keel: f64,
}

impl Ellipse {
    pub fn new(center: Point, a: f64, b: f64, keel: f64) -> Self {
        Ellipse {
            center,
            max_axis: a.max(b),
            a_squared: a.powf(2.),
            b_squared: b.powf(2.),
            keel,
        }
    }
}
