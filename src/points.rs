#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn point(x: f64, y: f64) -> Point { Point { x, y } }

pub trait Mean<T> {
    fn mean(&self) -> T;
}

pub trait Dist {
    fn dist(&self, other: &Self) -> f64;
    fn dist2(&self, other: &Self) -> f64;
}

pub trait Closest<T> {
    fn closest_idx(&self, value: &T) -> usize;
}

impl Closest<Point> for Vec<Point> {
    fn closest_idx(&self, value: &Point) -> usize {
        let mut min_idx: usize = 0;
        let mut min_dist: f64 = f64::MAX;
        for (i, centroid) in self.iter().enumerate() {
            let d2 = centroid.dist2(value);
            if d2 < min_dist {
                min_dist = d2;
                min_idx = i
            }
        }
        min_idx
    }
}

impl Mean<Point> for Vec<Point> {
    fn mean(&self) -> Point {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        self.iter().for_each(|&Point { x, y }| {
            sum_x += x;
            sum_y += y;
        });
        let count = self.len() as f64;
        let x = sum_x / count;
        let y = sum_y / count;
        Point { x, y }
    }
}

impl Dist for Point {
    fn dist(&self, other: &Self) -> f64 {
        self.dist2(other).sqrt()
    }

    fn dist2(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

