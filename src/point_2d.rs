use std::ops;

#[derive(Copy, Clone)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn distance_to(&self, other: &Point2D) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn vector_to(&self, other: &Point2D) -> Vector2D {
        Vector2D::new(other.x - self.x, other.y - self.y)
    }
}

impl ops::Add<Point2D> for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Point2D) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Point2D> for Point2D {
    type Output = Point2D;

    fn sub(self, rhs: Point2D) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f32> for Point2D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub struct Vector2D {
    pub x: f32,
    pub y: f32,
    pub length: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            length: Vector2D::length(x, y),
        }
    }

    fn length(x: f32, y: f32) -> f32 {
        (x.powi(2) + y.powi(2)).sqrt()
    }

    pub fn to_normalized(&self) -> Self {
        Self {
            x: self.x / self.length,
            y: self.y / self.length,
            length: 1.0,
        }
    }
}
