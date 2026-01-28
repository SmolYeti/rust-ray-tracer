mod aabb;
mod affine_transforms;
mod bvh_node;
mod camera;
mod checker_texture;
mod color;
mod constant_medium;
mod cosine_pdf;
mod demo_render;
mod dielectric;
mod diffuse_light;
mod hittable;
mod hittable_list;
mod hittable_pdf;
mod image_texture;
mod interval;
mod isotropic;
mod lambertian;
mod material;
mod metal;
mod mixture_pdf;
mod noise_texture;
mod orthonormal_basis;
mod pdf;
mod perlin;
mod quad;
mod ray;
mod rtweekend;
mod solid_texture;
mod sphere;
mod sphere_pdf;
mod texture;
mod vector_3;

use crate::demo_render::final_render_book1;
use crate::demo_render::checker_spheres;
use crate::demo_render::earth;
use crate::demo_render::two_perlin_sphere;
use crate::demo_render::quads;
use crate::demo_render::simple_light;
use crate::demo_render::cornell_box;
use crate::demo_render::cornell_smoke;
use crate::demo_render::final_scene_book2;
use crate::demo_render::cornell_box_metal;
use crate::demo_render::cornell_box_glass_sphere;

fn main() {
    let run_single = true;
    if run_single {
        let width = 800;
        let samples = 10;
        let depth = 50;
        let show = true;
        let save = true;
        let input = 9;
        println!("Input: {}", &input);
        match input {
            1 => final_render_book1(width, samples, depth, show, save),
            2 => checker_spheres(width, samples, depth, show, save),
            3 => earth(width, samples, depth, show, save),
            4 => two_perlin_sphere(width, samples, depth, show, save),
            5 => quads(width, samples, depth, show, save),
            6 => simple_light(width, samples, depth, show, save),
            7 => cornell_box(width, samples, depth, show, save),
            8 => cornell_smoke(width, samples, depth, show, save),
            9 => final_scene_book2(width, samples, depth, show, save),
            10 => cornell_box_metal(width, samples, depth, show, save),
            11 => cornell_box_glass_sphere(width, samples, depth, show, save),
            _ => final_scene_book2(width, samples, depth, show, save),
        };
    } else {
        let width = 400;
        let samples = 10;
        let depth = 10;
        let show = false;
        let save = true;
        for index in 1..11 {
            match index {
                1 => final_render_book1(width, samples, depth, show, save),
                2 => checker_spheres(width, samples, depth, show, save),
                3 => earth(width, samples, depth, show, save),
                4 => two_perlin_sphere(width, samples, depth, show, save),
                5 => quads(width, samples, depth, show, save),
                6 => simple_light(width, samples, depth, show, save),
                7 => cornell_box(width, samples, depth, show, save),
                8 => cornell_smoke(width, samples, depth, show, save),
                9 => final_scene_book2(width, samples, depth, show, save),
                10 => cornell_box_metal(width, samples, depth, show, save),
                11 => cornell_box_glass_sphere(width, samples, depth, show, save),
                _ => println!("Bad index in full loop: {}", index),
            };
        }
    }
}
