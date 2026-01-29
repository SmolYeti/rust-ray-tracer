use ray_tracing::demo_render::final_render_book1;
use ray_tracing::demo_render::checker_spheres;
use ray_tracing::demo_render::earth;
use ray_tracing::demo_render::two_perlin_sphere;
use ray_tracing::demo_render::quads;
use ray_tracing::demo_render::simple_light;
use ray_tracing::demo_render::cornell_box;
use ray_tracing::demo_render::cornell_smoke;
use ray_tracing::demo_render::final_scene_book2;
use ray_tracing::demo_render::cornell_box_metal;
use ray_tracing::demo_render::cornell_box_glass_sphere;

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
