mod aabb;
mod affine_transforms;
mod bvh_node;
mod camera;
mod checker_texture;
mod color;
mod constant_medium;
mod cosine_pdf;
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
use crate::affine_transforms::{RotateY, Translate};
use crate::bvh_node::BVHNode;
use crate::camera::Camera;
use crate::checker_texture::CheckerTexture;
use crate::constant_medium::ConstantMedium;
use crate::dielectric::Dielectric;
use crate::diffuse_light::DiffuseLight;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::image_texture::ImageTexture;
use crate::lambertian::Lambertian;
use crate::material::Material;
use crate::metal::Metal;
use crate::noise_texture::NoiseTexture;
use crate::quad::quad_box;
use crate::quad::Quad;
use crate::rtweekend::random_f64_range;
use crate::sphere::Sphere;
use crate::texture::Texture;
use crate::vector_3::Vec3;
use image::RgbImage;
use ray_tracing::render_buffer;
use std::path::Path;
use std::sync::Arc;

fn final_render_book1(width: i32, samples: i32, depth: i32, show: bool, save: bool) {
    // World
    let mut build_world = HittableList::new();

    // Ground
    let ground_mat = Arc::new(Lambertian::from_color(Vec3::new(0.5, 0.5, 0.5)));
    build_world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, -0.0),
        1000.0,
        ground_mat,
    )));

    //Create random spheres
    let compare_vec = Vec3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let rand_mat = rand::random::<f64>();
            let center = Vec3::new(
                a as f64 + (0.9 * rand::random::<f64>()),
                0.2,
                b as f64 + (0.9 * rand::random::<f64>()),
            );

            if (center - compare_vec).length() > 0.9 {
                let mat: Arc<dyn Material + Sync + Send>;
                if rand_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    mat = Arc::new(Lambertian::from_color(albedo));
                    let center_end = center + Vec3::new(0.0, rand::random::<f64>() * 0.25, 0.0);
                    build_world.add(Arc::new(Sphere::new_moving(center, 0.2, mat, center_end)));
                } else if rand_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);
                    mat = Arc::new(Metal::new(albedo, fuzz));
                    build_world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                } else {
                    mat = Arc::new(Dielectric::new(1.5));
                    build_world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }

    // Center spheres
    let mat = Arc::new(Dielectric::new(1.5));
    build_world.add(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat)));

    let mat = Arc::new(Lambertian::from_color(Vec3::new(0.4, 0.2, 0.1)));
    build_world.add(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat)));

    let mat = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    build_world.add(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat)));

    // Make the world into a bvh
    let mut world = HittableList::new();
    world.add(Arc::new(BVHNode::from_list(&build_world)));

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = width;
    cam.samples_per_pixel = samples;
    cam.max_depth = depth;

    cam.vfov = 20.0;
    cam.look_from = Vec3::new(13.0, 2.0, 3.0);
    cam.look_at = Vec3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    let world = Arc::new(world);
    let buffer = cam.render(world, None, true, 6);
    if save {
        save_image(
            "Book1_Final",
            &buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
            samples,
            depth,
        );
    }
    if show {
        render_buffer(
            buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
        );
    }
}

fn checker_spheres(width: i32, samples: i32, depth: i32, show: bool, save: bool) {
    // World
    let mut world = HittableList::new();

    let checker: Arc<dyn Texture + Sync + Send> = Arc::new(CheckerTexture::new(
        0.32,
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let ground_mat = Arc::new(Lambertian::new(Arc::clone(&checker)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, -0.0),
        10.0,
        ground_mat,
    )));
    let ground_mat = Arc::new(Lambertian::new(checker));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, -0.0),
        10.0,
        ground_mat,
    )));

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = width;
    cam.samples_per_pixel = samples;
    cam.max_depth = depth;

    cam.vfov = 20.0;
    cam.look_from = Vec3::new(13.0, 2.0, 3.0);
    cam.look_at = Vec3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    let world = Arc::new(world);
    let buffer = cam.render(world, None, true, 6);
    if save {
        save_image(
            "Checker_Spheres",
            &buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
            samples,
            depth,
        );
    }
    if show {
        render_buffer(
            buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
        );
    }
}

fn earth(width: i32, samples: i32, depth: i32, show: bool, save: bool) {
    let mut world = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::new("src/earthmap.jpg"));
    let ground_mat = Arc::new(Lambertian::new(earth_texture));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        ground_mat,
    )));

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = width;
    cam.samples_per_pixel = samples;
    cam.max_depth = depth;

    cam.vfov = 20.0;
    cam.look_from = Vec3::new(0.0, 0.0, 12.0);
    cam.look_at = Vec3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let world = Arc::new(world);
    let buffer = cam.render(world, None, false, 6);
    if save {
        save_image(
            "Earth",
            &buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
            samples,
            depth,
        );
    }
    if show {
        render_buffer(
            buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
        );
    }
}

fn two_perlin_sphere(width: i32, samples: i32, depth: i32, show: bool, save: bool) {
    let mut world = HittableList::new();

    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(NoiseTexture::new(4.0));
    let mat_0 = Arc::new(Lambertian::new(Arc::clone(&pertext)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_0,
    )));
    let mat_1 = Arc::new(Lambertian::new(Arc::clone(&pertext)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, mat_1)));

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = width;
    cam.samples_per_pixel = samples;
    cam.max_depth = depth;

    cam.vfov = 20.0;
    cam.look_from = Vec3::new(13.0, 2.0, 3.0);
    cam.look_at = Vec3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let world = Arc::new(world);
    let buffer = cam.render(world, None, true, 6);
    if save {
        save_image(
            "Perlin_Sphere",
            &buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
            samples,
            depth,
        );
    }
    if show {
        render_buffer(
            buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
        );
    }
}

fn quads(width: i32, samples: i32, depth: i32, show: bool, save: bool) {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from_color(Vec3::new(1.0, 0.2, 0.2)));
    let green = Arc::new(Lambertian::from_color(Vec3::new(0.2, 1.0, 0.2)));
    let blue = Arc::new(Lambertian::from_color(Vec3::new(0.2, 0.2, 1.0)));
    let orange = Arc::new(Lambertian::from_color(Vec3::new(1.0, 0.5, 0.2)));
    let teal = Arc::new(Lambertian::from_color(Vec3::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        blue,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        orange,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        teal,
    )));

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width = width;
    cam.samples_per_pixel = samples;
    cam.max_depth = depth;

    cam.vfov = 80.0;
    cam.look_from = Vec3::new(0.0, 0.0, 9.0);
    cam.look_at = Vec3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let world = Arc::new(world);
    let buffer = cam.render(world, None, true, 6);
    if save {
        save_image(
            "Quads",
            &buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
            samples,
            depth,
        );
    }
    if show {
        render_buffer(
            buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
        );
    }
}

fn simple_light(width: i32, samples: i32, depth: i32, show: bool, save: bool) {
    let mut world = HittableList::new();

    let pertext: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(4.0))));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        -1000.0,
        Arc::clone(&pertext),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::clone(&pertext),
    )));

    let mut lights = HittableList::new();
    let diff_light: Arc<dyn Material + Sync + Send> =
        Arc::new(DiffuseLight::color(Vec3::new(4.0, 4.0, 4.0)));
    let light: Arc<dyn Hittable + Sync + Send> = Arc::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        Arc::clone(&diff_light),
    ));
    world.add(Arc::clone(&light));
    lights.add(light);
    let light: Arc<dyn Hittable + Sync + Send> = Arc::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        diff_light,
    ));
    world.add(Arc::clone(&light));
    lights.add(light);

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = width;
    cam.samples_per_pixel = samples;
    cam.max_depth = depth;
    cam.background = Vec3::new(0.0, 0.0, 0.0);

    cam.vfov = 20.0;
    cam.look_from = Vec3::new(26.0, 3.0, 6.0);
    cam.look_at = Vec3::new(0.0, 2.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let world = Arc::new(world);
    let lights = Arc::new(lights);
    let buffer = cam.render(world, Some(lights), true, 6);
    if save {
        save_image(
            "Simple_Light",
            &buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
            samples,
            depth,
        );
    }
    if show {
        render_buffer(
            buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
        );
    }
}

fn cornell_box(width: i32, samples: i32, depth: i32, show: bool, save: bool) {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from_color(Vec3::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Vec3::new(0.12, 0.45, 0.15)));
    let light_mat = Arc::new(DiffuseLight::color(Vec3::new(15.0, 15.0, 15.0)));

    // Scene
    let light: Arc<dyn Hittable + Sync + Send> = Arc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light_mat,
    ));
    world.add(Arc::clone(&light));
    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::clone(&white),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        Arc::clone(&white),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Arc::clone(&white),
    )));

    // Boxes
    let box_1 = quad_box(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        Arc::clone(&white),
    );
    let box_1 = Arc::new(RotateY::new(box_1, 15.0));
    let box_1 = Arc::new(Translate::new(box_1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box_1);

    let box_2 = quad_box(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        Arc::clone(&white),
    );
    let box_2 = Arc::new(RotateY::new(box_2, -18.0));
    let box_2 = Arc::new(Translate::new(box_2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box_2);

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width = width;
    cam.samples_per_pixel = samples;
    cam.max_depth = depth;
    cam.background = Vec3::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.look_from = Vec3::new(278.0, 278.0, -800.0);
    cam.look_at = Vec3::new(278.0, 278.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let world = Arc::new(world);
    let buffer = cam.render(world, Some(light), true, 6);

    if save {
        save_image(
            "Cornell_Box",
            &buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
            samples,
            depth,
        );
    }

    if show {
        render_buffer(
            buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
        );
    }
}

fn cornell_smoke(width: i32, samples: i32, depth: i32, show: bool, save: bool) {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();

    let red = Arc::new(Lambertian::from_color(Vec3::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Vec3::new(0.12, 0.45, 0.15)));
    let light_mat = Arc::new(DiffuseLight::color(Vec3::new(7.0, 7.0, 7.0)));

    // Scene
    let light: Arc<dyn Hittable + Sync + Send> = Arc::new(Quad::new(
        Vec3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light_mat,
    ));
    world.add(Arc::clone(&light));
    lights.add(light);
    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::clone(&white),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        Arc::clone(&white),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Arc::clone(&white),
    )));

    // Boxes
    let box_1 = quad_box(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        Arc::clone(&white),
    );
    let box_1 = Arc::new(RotateY::new(box_1, 15.0));
    let box_1 = Arc::new(Translate::new(box_1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(Arc::new(ConstantMedium::color(
        box_1,
        0.01,
        Vec3::new(0.0, 0.0, 0.0),
    )));

    let box_2 = quad_box(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        Arc::clone(&white),
    );
    let box_2 = Arc::new(RotateY::new(box_2, -18.0));
    let box_2 = Arc::new(Translate::new(box_2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(Arc::new(ConstantMedium::color(
        box_2,
        0.01,
        Vec3::new(1.0, 1.0, 1.0),
    )));

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width = width;
    cam.samples_per_pixel = samples;
    cam.max_depth = depth;
    cam.background = Vec3::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.look_from = Vec3::new(278.0, 278.0, -800.0);
    cam.look_at = Vec3::new(278.0, 278.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let world = Arc::new(world);
    let lights = Arc::new(lights);
    let buffer = cam.render(world, Some(lights), true, 6);

    if save {
        save_image(
            "Cornell_Smoke",
            &buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
            samples,
            depth,
        );
    }

    if show {
        render_buffer(
            buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
        );
    }
}

fn final_scene_book2(width: i32, samples: i32, depth: i32, show: bool, save: bool) {
    // Create the ground
    let mut boxes_1 = HittableList::new();
    let ground_mat: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_color(Vec3::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes_1.add(quad_box(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                Arc::clone(&ground_mat),
            ));
        }
    }

    // Create the scene
    let mut scene = HittableList::new();
    scene.add(Arc::new(BVHNode::from_list(&boxes_1)));

    // Light
    let diff_light = Arc::new(DiffuseLight::color(Vec3::new(7.0, 7.0, 7.0)));
    let light: Arc<dyn Hittable + Sync + Send> = Arc::new(Quad::new(
        Vec3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        diff_light,
    ));
    scene.add(Arc::clone(&light));

    // Spheres
    let center_1 = Vec3::new(400.0, 400.0, 200.0);
    let center_2 = center_1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_mat: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_color(Vec3::new(0.7, 0.3, 0.1)));
    scene.add(Arc::new(Sphere::new_moving(
        center_1, 50.0, sphere_mat, center_2,
    )));

    scene.add(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    scene.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)),
    )));

    // Fog
    let boundary: Arc<dyn Hittable + Sync + Send> = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    scene.add(Arc::clone(&boundary));
    scene.add(Arc::new(ConstantMedium::color(
        boundary,
        0.2,
        Vec3::new(0.2, 0.4, 0.9),
    )));
    let fog = Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    scene.add(Arc::new(ConstantMedium::color(
        fog,
        0.0001,
        Vec3::new(1.0, 1.0, 1.0),
    )));

    // Earth
    let earth_texture = Arc::new(ImageTexture::new("src/earthmap.jpg"));
    let emat = Arc::new(Lambertian::new(earth_texture));
    scene.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));

    // Perlin
    let pertext = Arc::new(NoiseTexture::new(0.1));
    let pmat = Arc::new(Lambertian::new(pertext));
    scene.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        pmat,
    )));

    // Sphere box
    let mut spheres = HittableList::new();
    let white: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        spheres.add(Arc::new(Sphere::new(
            Vec3::random_range(0.0, 165.0),
            10.0,
            Arc::clone(&white),
        )));
    }
    scene.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BVHNode::from_list(&spheres)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    // Render
    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width = width;
    cam.samples_per_pixel = samples;
    cam.max_depth = depth;
    cam.background = Vec3::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.look_from = Vec3::new(478.0, 278.0, -600.0);
    cam.look_at = Vec3::new(278.0, 278.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let world = Arc::new(scene);
    let buffer = cam.render(world, Some(light), true, 6);

    if save {
        save_image(
            "Book2_Final",
            &buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
            samples,
            depth,
        );
    }

    // Render to a window
    if show {
        render_buffer(
            buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
        );
    }
}

fn cornell_box_metal(width: i32, samples: i32, depth: i32, show: bool, save: bool) {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from_color(Vec3::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Vec3::new(0.12, 0.45, 0.15)));
    let light_mat = Arc::new(DiffuseLight::color(Vec3::new(15.0, 15.0, 15.0)));
    let metal_mat: Arc<dyn Material + Sync + Send> =
        Arc::new(Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0));

    // Scene
    let light: Arc<dyn Hittable + Sync + Send> = Arc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light_mat,
    ));
    world.add(Arc::clone(&light));
    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::clone(&white),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        Arc::clone(&white),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Arc::clone(&white),
    )));

    // Boxes
    let box_1 = quad_box(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        metal_mat,
    );
    let box_1 = Arc::new(RotateY::new(box_1, 15.0));
    let box_1 = Arc::new(Translate::new(box_1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box_1);

    let box_2 = quad_box(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        Arc::clone(&white),
    );
    let box_2 = Arc::new(RotateY::new(box_2, -18.0));
    let box_2 = Arc::new(Translate::new(box_2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box_2);

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width = width;
    cam.samples_per_pixel = samples;
    cam.max_depth = depth;
    cam.background = Vec3::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.look_from = Vec3::new(278.0, 278.0, -800.0);
    cam.look_at = Vec3::new(278.0, 278.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let world = Arc::new(world);
    let buffer = cam.render(world, Some(light), true, 7);

    if save {
        save_image(
            "Cornell_Box_Metal",
            &buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
            samples,
            depth,
        );
    }

    if show {
        render_buffer(
            buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
        );
    }
}

fn cornell_box_glass_sphere(width: i32, samples: i32, depth: i32, show: bool, save: bool) {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from_color(Vec3::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Vec3::new(0.12, 0.45, 0.15)));
    let light_mat = Arc::new(DiffuseLight::color(Vec3::new(15.0, 15.0, 15.0)));
    let glass = Arc::new(Dielectric::new(1.5));

    // Scene
    let light: Arc<dyn Hittable + Sync + Send> = Arc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light_mat,
    ));
    world.add(Arc::clone(&light));
    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::clone(&white),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        Arc::clone(&white),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Arc::clone(&white),
    )));

    // Boxes
    let box_1 = quad_box(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        Arc::clone(&white),
    );
    let box_1 = Arc::new(RotateY::new(box_1, 15.0));
    let box_1 = Arc::new(Translate::new(box_1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box_1);

    let sphere = Arc::new(Sphere::new(Vec3::new(190.0, 90.0, 190.0), 90.0, glass));
    world.add(sphere);

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width = width;
    cam.samples_per_pixel = samples;
    cam.max_depth = depth;
    cam.background = Vec3::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.look_from = Vec3::new(278.0, 278.0, -800.0);
    cam.look_at = Vec3::new(278.0, 278.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let world = Arc::new(world);
    let buffer = cam.render(world, Some(light), true, 7);

    if save {
        save_image(
            "Cornell_Box_Glass_Sphere",
            &buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
            samples,
            depth,
        );
    }

    if show {
        render_buffer(
            buffer,
            cam.render_width() as u32,
            cam.render_height() as u32,
        );
    }
}

fn save_image(name: &str, buffer: &Vec<u32>, width: u32, height: u32, samples: i32, depth: i32) {
    // Load the buffer into an image
    let image = RgbImage::from_fn(width, height, |x, y| {
        let index = x + (y * width);
        let value = buffer[index as usize];
        let red = ((value >> 16) & 0x000000FF) as u8;
        let green = ((value >> 8) & 0x000000FF) as u8;
        let blue = (value & 0x000000FF) as u8;
        image::Rgb([red, green, blue])
    });

    let mut full_path = "renders/".to_owned();
    let path_end = ".png";

    // Add the file name
    full_path.push_str(name);

    // width, samples, depth
    full_path.push_str("_w");
    full_path.push_str(width.to_string().as_str());
    full_path.push_str("_s");
    full_path.push_str(samples.to_string().as_str());
    full_path.push_str("_d");
    full_path.push_str(depth.to_string().as_str());

    full_path.push_str(path_end);
    println!("Full Path: {}", &full_path);
    let path = Path::new(&full_path);
    let _ = image.save(path);
}

fn main() {
    let run_single = true;
    if run_single {
        let width = 800;
        let samples = 1000;
        let depth = 50;
        let show = true;
        let save = true;
        let input = 9;
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
