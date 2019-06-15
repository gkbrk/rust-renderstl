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
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub struct Triangle {
    pub normal: Vector3<f64>,
    pub points: [Vector3<f64>; 3],
}

impl Intersectable for Triangle {
    fn ray_intersects(&self, ray: Ray) -> Option<Hit> {
        let epsilon = 0.0000001;

        let vertex0 = self.points[0];
        let vertex1 = self.points[1];
        let vertex2 = self.points[2];

        let edge1 = vertex1 - vertex0;
        let edge2 = vertex2 - vertex0;

        let h = ray.direction.cross_prod(edge2);
        let a = edge1.dot(h);

        if a > -epsilon && a < epsilon {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - vertex0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross_prod(edge1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);

        if t > epsilon {
            return Some(Hit {
                point: ray.origin + ray.direction * t,
                normal: self.normal,
            });
        }

        return None;
    }
}

impl Intersectable for Vec<Triangle> {
    fn ray_intersects(&self, ray: Ray) -> Option<Hit> {
        let mut nearest_intersection = Vector3::new(9999.0, 9999.0, 9999.0);
        let mut nearest_normal = Vector3::new(0.0, 0.0, 0.0);

        for t in self {
            if let Some(hit) = t.ray_intersects(ray) {
                if (hit.point - ray.origin).len() < (nearest_intersection - ray.origin).len() {
                    nearest_intersection = hit.point;
                    nearest_normal = t.normal;
                }
            }
        }

        if nearest_intersection.x != 9999.0 {
            return Some(Hit {
                point: nearest_intersection,
                normal: nearest_normal,
            });
        }

        None
    }
}

pub fn read_vec3(r: &mut BufRead) -> std::io::Result<Vector3<f64>> {
    Ok(Vector3::new(
        r.read_f32::<LittleEndian>()? as f64,
        r.read_f32::<LittleEndian>()? as f64,
        r.read_f32::<LittleEndian>()? as f64,
    ))
}

pub fn read_stl<T: Read>(r: T) -> std::io::Result<Vec<Triangle>> {
    let mut bufread = BufReader::new(r);
    bufread.read_exact(&mut [0; 80])?;
    let num_triangles = bufread.read_u32::<LittleEndian>()?;

    let mut triangles = Vec::with_capacity(num_triangles as usize);

    for _ in 0..num_triangles {
        triangles.push(Triangle {
            normal: read_vec3(&mut bufread)?,
            points: [
                read_vec3(&mut bufread)?,
                read_vec3(&mut bufread)?,
                read_vec3(&mut bufread)?,
            ],
        });
        bufread.read_u16::<LittleEndian>()?;
    }
    Ok(triangles)
}

pub fn read_stl_file(path: &str) -> std::io::Result<Vec<Triangle>> {
    read_stl(File::open(path)?)
}
