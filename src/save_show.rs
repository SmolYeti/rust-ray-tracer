use image::RgbImage;
use softbuffer;
use std::num::NonZeroU32;
use std::path::Path;
use std::rc::Rc;
use winit::dpi::{PhysicalSize, Size};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

// Code modified from the example of https://github.com/rust-windowing/softbuffer/blob/master/README.md
pub fn render_buffer(buffer: Vec<u32>, buffer_width: u32, buffer_height: u32) {
    let event_loop = EventLoop::new().unwrap();
    let window_size = Size::Physical(PhysicalSize {
        width: buffer_width,
        height: buffer_height,
    });
    let window = Rc::new(
        WindowBuilder::new()
            .with_inner_size(window_size)
            .build(&event_loop)
            .unwrap(),
    );
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
                    let (width, height) = {
                        let size = window.inner_size();
                        (size.width, size.height)
                    };
                    surface
                        .resize(
                            NonZeroU32::new(width).unwrap(),
                            NonZeroU32::new(height).unwrap(),
                        )
                        .unwrap();

                    let mut render_buffer = surface.buffer_mut().unwrap();
                    for index in 0..(width * height) {
                        render_buffer[index as usize] = buffer[index as usize];
                    }

                    render_buffer.present().unwrap();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => {
                    elwt.exit();
                }
                _ => {}
            }
        })
        .unwrap();
}

pub fn save_image(name: &str, buffer: &Vec<u32>, width: u32, height: u32, samples: i32, depth: i32) {
    // Load the buffer into an image
    let image = RgbImage::from_fn(width, height, |x, y| {
        let index = x + (y * width);
        let value = buffer[index as usize];
        let red = ((value >> 16) & 0x000000FF) as u8;
        let green = ((value >> 8) & 0x000000FF) as u8;
        let blue = (value & 0x000000FF) as u8;
        image::Rgb([red, green, blue])
    });

    let _ = std::fs::create_dir("renders");  

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