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

use crate::intersectable::Intersectable;
use crate::ray::{Hit, Ray};
use crate::vector::Vector3;

pub struct Scene<T> {
    pub obj: T,
    pub camera: Vector3<f64>,
}

impl<T: Intersectable> Scene<T> {
    pub fn hit(&self, ray: Ray) -> Option<Hit> {
        self.obj.ray_intersects(ray)
    }

    pub fn shot(&self, target: Vector3<f64>) -> Ray {
        Ray {
            origin: self.camera,
            direction: (target - self.camera).norm(),
        }
    }

    pub fn from_obj(o: T) -> Self {
        Self {
            obj: o,
            camera: Vector3::new(0.0, 0.0, -50.0),
        }
    }
}
