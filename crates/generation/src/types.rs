use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> Vector2<U> {
        Vector2 {
            x: f(self.x),
            y: f(self.y),
        }
    }
}

impl<T: Add> Add<Self> for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Mul + Copy> Mul<T> for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Div + Copy> Div<T> for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn div(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Mul> Mul<Self> for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Div> Div<Self> for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn div(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Area<T> {
    pub start: Vector2<T>,
    pub end: Vector2<T>,
}

impl<T: Add + Copy> Area<T> {
    pub fn translate(self, translation: Vector2<T>) -> Area<T::Output> {
        Area {
            start: self.start + translation,
            end: self.end + translation,
        }
    }
}

impl<T: Sub> Area<T> {
    pub fn width(self) -> T::Output {
        self.end.x - self.start.x
    }

    pub fn height(self) -> T::Output {
        self.end.y - self.start.y
    }
}

impl<T: Div + Copy> Div<Vector2<T>> for Area<T> {
    type Output = Area<T::Output>;

    fn div(self, rhs: Vector2<T>) -> Self::Output {
        Area {
            start: self.start / rhs,
            end: self.end / rhs,
        }
    }
}
