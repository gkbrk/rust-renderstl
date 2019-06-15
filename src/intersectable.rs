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

use crate::ray::{Hit, Ray};
use crate::vector::Vector3;

pub trait Intersectable {
    fn ray_intersects(&self, ray: Ray) -> Option<Hit>;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub origin: Vector3<f64>,
    pub radius: f64,
}

impl Intersectable for Sphere {
    fn ray_intersects(&self, ray: Ray) -> Option<Hit> {
        let sray = self.origin - ray.origin;
        let det = ray.direction.dot(sray).powi(2) - sray.len().powi(2) + self.radius.powi(2);

        if det >= 0.0 {
            let t1 = ray.direction.dot(sray) - det.sqrt();
            let t2 = ray.direction.dot(sray) + det.sqrt();

            if t2 > 0.0 {
                let t = if t1 >= 0.0 { t1 } else { t2 };

                let point = ray.origin + ray.direction * t;
                let normal = (point - self.origin).norm();

                return Some(Hit {
                    point: point + normal * 0.001,
                    normal: normal,
                });
            }
        }
        None
    }
}
