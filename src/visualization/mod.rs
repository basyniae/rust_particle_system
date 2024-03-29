use std::fs::File;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, ImageBuffer};

/// Color trait to be implemented on a particle system enum. Implements the `get_color` trait.
pub trait Coloration {
    /// For the purpose of visualization, which color should the state `self` be represented by?
    /// Returns a `[u8; 4]` in the format `[r,g,b,a]`. Ordinarily we want `a=255`.
    fn get_color(&self, state: usize) -> [u8; 4];
}

/// Visualize the input solution as a graph over time. Best suited for 1D graphs (lines or circles).
///
/// # Parameters
/// * `solution`: Vector containing the state record. Format should be the same as the output of
/// `particle_system_solver`.
/// * `img_name`: &str of the image to be saved. Should end in ".png".
/// * `img_x`: Width of the simulation, i.e., number of points in the graph.
pub fn save_as_growth_img(coloration: Box<dyn Coloration>, solution: Vec<usize>, img_name: &str, img_x: u32) {
    // y is the time axis, so the y-range should be the length of the simulation
    let img_y = (solution.len() as u32) / img_x;

    let mut img_buf = image::ImageBuffer::new(img_x, img_y);

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        *pixel = image::Rgba(coloration.get_color(*solution.get((x + img_x * y) as usize).unwrap()))
    }

    img_buf.save(img_name).unwrap(); // Unwrap to make sure it panics on errors
}

/// Visualize the input solution as a graph over time. Best suited for 2D graphs (rectangles,
/// torii, or thin cylinder walls).
///
/// # Parameters
/// * `solution`: Vector containing the state record. Format should be the same as the output of
/// `particle_system_solver`.
/// * `img_name`: &str of the image to be saved. Should end in ".gif".
/// * `img_x`: Width of the graph.
/// * `img_y`: Height of the graph.
/// * `ms_per_frame`: Number of milliseconds each frame (i.e., snapshot) should be displayed in the
/// output gif.
pub fn save_as_gif(coloration: Box<dyn Coloration>, solution: Vec<usize>, img_name: &str, img_x: u32, img_y: u32, ms_per_frame: u32) {
    let file_out = File::create(img_name).unwrap();

    let mut encoder = GifEncoder::new_with_speed(file_out, 30);

    // Gif config: repeat once (does not work)
    encoder.set_repeat(Repeat::Finite(1)).unwrap();

    let nr_frames = solution.len() / (img_x * img_y) as usize;

    // convert solution into color frames
    let mut frames: Vec<Frame> = Vec::new();
    for frame_index in 0..nr_frames {
        let mut buffer = ImageBuffer::new(img_x, img_y);
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            *pixel = image::Rgba(coloration.get_color(*solution.get((x + img_x * y + (frame_index as u32 * img_x * img_y)) as usize).unwrap()))
        }
        let frame = Frame::from_parts(buffer, img_x, img_x, Delay::from_numer_denom_ms(ms_per_frame, 1));
        frames.push(frame);
    }

    // finally encode
    encoder.encode_frames(&mut frames.into_iter()).unwrap();
}