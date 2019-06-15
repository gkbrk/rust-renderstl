// renderstl - Renders an STL file into an image
// Copyright (C) 2019  Gokberk Yaltirakli

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
    }
}

pub trait Sqrt {
    fn sqrt(self) -> f64;
}

impl<T: Into<f64>> Sqrt for T {
    fn sqrt(self) -> f64 {
        self.into().sqrt()
    }
}

impl<T: Mul<Output = T> + Add<Output = T>> Vector3<T> {
    pub fn dot(self, other: Self) -> T {
        let result = self * other;
        result.x + result.y + result.z
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Copy> Vector3<T> {
    pub fn cross_prod(self, other: Self) -> Self {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T: Sqrt + Mul<Output = T> + Add<Output = T> + Copy> Vector3<T> {
    pub fn len(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl<T> Vector3<T>
where
    T: Sqrt + Mul<Output = T> + Add<Output = T> + Copy + Into<f64>,
{
    pub fn norm(self) -> Vector3<f64> {
        let len = self.len();

        Vector3 {
            x: self.x.into() / len,
            y: self.y.into() / len,
            z: self.z.into() / len,
        }
    }
}

impl<T: Add<Output = T>> Add for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, other: Self::Output) -> Self::Output {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, other: T) -> Self::Output {
        Vector3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, other: Self::Output) -> Self::Output {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, other: T) -> Self::Output {
        Vector3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl<T: Div<Output = T>> Div for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, other: Self::Output) -> Self::Output {
        Vector3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, other: T) -> Self::Output {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, other: Self::Output) -> Self::Output {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, other: T) -> Self::Output {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
