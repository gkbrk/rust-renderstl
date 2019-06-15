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

mod intersectable;
mod ray;
mod scene;
mod stl;
mod vector;
use crate::scene::Scene;
use crate::stl::read_stl_file;
use crate::vector::Vector3;
use rayon::prelude::*;

fn main() {
    let left = -500;
    let right = 500;
    let bottom = -500;
    let top = 500;

    let scene = Scene::from_obj(read_stl_file("4.stl").unwrap());

    let pixels: Vec<Vec<(u8, u8, u8)>> = (bottom..top)
        .rev()
        .collect::<Vec<isize>>()
        .into_par_iter()
        .map(|y| {
            (left..right)
                .into_par_iter()
                .map(|x| {
                    let px = 0.01 * x as f64;
                    let py = 0.01 * y as f64;
                    let point = Vector3::new(px, py, -10.0);

                    let ray = scene.shot(point);
                    match scene.hit(ray) {
                        Some(h) => {
                            let shade = h.normal.dot(ray.direction * -1.0) * 255.0;

                            (shade as u8, shade as u8, shade as u8)
                        }
                        None => (0, 0, 0),
                    }
                })
                .collect()
        })
        .collect();

    println!("P3");
    println!("{} {}", right - left, top - bottom);
    println!("255");

    for row in pixels {
        for col in row {
            println!("{} {} {}", col.0, col.1, col.2);
        }
    }
}
