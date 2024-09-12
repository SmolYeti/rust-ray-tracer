use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::hittable_pdf::HittablePDF;
use crate::interval::Interval;
use crate::material::ScatterPDF;
use crate::material::ScatterRecord;
use crate::mixture_pdf::MixturePDF;
use crate::pdf::PDF;
use crate::ray::Ray3;
use crate::rtweekend::degree_to_radians;
use crate::vector_3::Vec3;
use scoped_threadpool::Pool;
use std::sync::Arc;
use std::time::Instant;

pub struct Camera {
    pub samples_per_pixel: i32,
    pub image_width: i32,
    pub aspect_ratio: f64,
    pub max_depth: i32,
    pub vfov: f64,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub v_up: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub background: Vec3,
    image_height: i32,
    camera_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

unsafe impl Sync for Camera {}
unsafe impl Send for Camera {}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            samples_per_pixel: 10,
            image_width: 1800,
            aspect_ratio: (16.0 / 9.0),
            max_depth: 1,
            vfov: 90.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
            look_from: Vec3::new(0.0, 0.0, -1.0),
            look_at: Vec3::new(0.0, 0.0, -1.0),
            v_up: Vec3::new(0.0, 1.0, 0.0),
            image_height: 0,
            camera_center: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel00_loc: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
            background: Vec3::new(0.7, 0.8, 1.0),
        }
    }

    fn self_clone(&self) -> Camera {
        Camera {
            samples_per_pixel: self.samples_per_pixel,
            image_width: self.image_width,
            aspect_ratio: self.aspect_ratio,
            max_depth: self.max_depth,
            vfov: self.vfov,
            look_from: self.look_from,
            look_at: self.look_at,
            v_up: self.v_up,
            defocus_angle: self.defocus_angle,
            focus_dist: self.focus_dist,
            image_height: self.image_height,
            camera_center: self.camera_center,
            pixel_delta_u: self.pixel_delta_u,
            pixel_delta_v: self.pixel_delta_v,
            pixel00_loc: self.pixel00_loc,
            u: self.u,
            v: self.v,
            w: self.w,
            defocus_disk_u: self.defocus_disk_u,
            defocus_disk_v: self.defocus_disk_v,
            background: self.background,
        }
    }

    pub fn render_width(&self) -> i32 {
        self.image_width
    }

    pub fn render_height(&self) -> i32 {
        self.image_height
    }

    pub fn render(
        &mut self,
        world: Arc<dyn Hittable + Sync + Send>,
        lights: Option<Arc<dyn Hittable + Sync + Send>>,
        multi_thread: bool,
        threads: u32,
    ) -> Vec<u32> {
        self.initalize();

        let start_time = Instant::now();

        // Render to buffer
        let image_size = self.image_width * self.image_height;
        let mut buffer: Vec<u32> = Vec::with_capacity(image_size as usize);
        if multi_thread {
            let arc_cam = Arc::new(self.self_clone());
            for _ in 0..image_size {
                buffer.push(0);
            }
            {
                let mut pool = Pool::new(threads);
                pool.scoped(|scope| {
                    for (j, buffer_slice) in
                        buffer.chunks_mut(self.image_width as usize).enumerate()
                    {
                        let cam_clone = Arc::clone(&arc_cam);
                        let world_clone = Arc::clone(&world);
                        let lights_clone = match &lights {
                            None => None,
                            Some(unwrapped_lights) => Some(Arc::clone(&unwrapped_lights)),
                        };
                        scope.execute(move || {
                            Self::thread_render(
                                cam_clone,
                                world_clone,
                                lights_clone,
                                j as i32,
                                buffer_slice,
                            )
                        });
                    }
                });
            }
        } else {
            for j in 0..self.image_height {
                for i in 0..self.image_width {
                    let mut color_vec = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..self.samples_per_pixel {
                        let ray_sample = self.get_ray(i, j);
                        color_vec =
                            color_vec + self.ray_color(ray_sample, self.max_depth, &world, &lights);
                    }
                    buffer.push(crate::color::vec_to_val(&color_vec, self.samples_per_pixel));
                }
            }
        }
        let elapsed_time = start_time.elapsed();
        println!("\rDone! Took {} seconds", elapsed_time.as_secs());
        buffer
    }

    pub fn thread_render(
        cam: Arc<Camera>,
        world: Arc<dyn Hittable + Sync + Send>,
        lights: Option<Arc<dyn Hittable + Sync + Send>>,
        j_idx: i32,
        buffer: &mut [u32],
    ) {
        for (i, val) in buffer.iter_mut().enumerate() {
            let mut color_vec = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..cam.samples_per_pixel {
                let ray_sample = cam.get_ray(i as i32, j_idx);
                color_vec = color_vec + cam.ray_color(ray_sample, cam.max_depth, &world, &lights);
            }
            *val = crate::color::vec_to_val(&color_vec, cam.samples_per_pixel);
        }
    }

    fn initalize(&mut self) {
        // Calculate the height and ensure it is at least 1
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.camera_center = self.look_from;

        // Camera
        let theta = degree_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame.
        self.w = (self.look_from - self.look_at).unit_vector();
        self.u = self.v_up.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * (-self.v);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u.clone() / self.image_width as f64;
        self.pixel_delta_v = viewport_v.clone() / self.image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = self.camera_center
            - (self.focus_dist * self.w)
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + (0.5 * (self.pixel_delta_u + self.pixel_delta_v));

        // Calculate the camera defocus disk basis vector
        let defocus_radius = self.focus_dist * degree_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray3 {
        // Get a randomly-sampled camera ray for the picel at location i,j, originating the the camera defocus disk.
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center.clone()
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray3::new(ray_origin, ray_direction, rand::random::<f64>())
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rand::random::<f64>();
        let py = -0.5 + rand::random::<f64>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        // Returns a random point in the camera defocus disk
        let point = Vec3::random_in_unit_disk();
        self.camera_center + (point.x * self.defocus_disk_u) + (point.y * self.defocus_disk_v)
    }

    fn ray_color(
        &self,
        r: Ray3,
        depth: i32,
        world: &Arc<dyn Hittable + Sync + Send>,
        lights: &Option<Arc<dyn Hittable + Sync + Send>>,
    ) -> Vec3 {
        let mut hit_record = HitRecord::new();
        if depth <= 0 {
            Vec3::new(0.0, 0.0, 0.0)
        } else if world.hit(&r, Interval::new(0.0001, f64::INFINITY), &mut hit_record) {
            let mut color_emission = hit_record.mat.emitted(
                &r,
                &hit_record,
                hit_record.u,
                hit_record.v,
                hit_record.point,
            );
            let mut color_scattered = Vec3::empty();
            let mut scatter_rec = ScatterRecord::new();
            if hit_record.mat.scatter(&r, &hit_record, &mut scatter_rec) {
                match scatter_rec.pdf {
                    ScatterPDF::PDF(surface_pdf) => {
                        let pdf: Box<dyn PDF> = match lights {
                            Some(unwrapped_lights) => {
                                let light_pdf = Box::new(HittablePDF::new(
                                    Arc::clone(unwrapped_lights),
                                    hit_record.point,
                                ));
                                Box::new(MixturePDF::new(light_pdf, surface_pdf))
                            }
                            None => surface_pdf,
                        };

                        let scattered = Ray3::new(hit_record.point, pdf.generate(), r.time());
                        let pdf_val = pdf.value(&scattered.direction());

                        let scattered_pdf =
                            hit_record.mat.scattering_pdf(&r, &hit_record, &scattered);
                        let sample_color = self.ray_color(scattered, depth - 1, &world, &lights);
                        color_scattered =
                            (scatter_rec.attenuation * scattered_pdf * sample_color) / pdf_val;
                    }
                    ScatterPDF::Skip(ray) => {
                        color_emission = Vec3::empty();
                        let sample_color = self.ray_color(ray, depth - 1, &world, &lights);
                        color_scattered = scatter_rec.attenuation * sample_color;
                    }
                }
            }
            color_emission + color_scattered
        } else {
            self.background
        }
    }
}
