mod triangle;
mod rasterizer;
mod utils;

extern crate opencv;

use nalgebra::{Vector3};
use opencv::{
    Result,
};
use opencv::highgui::{imshow, wait_key};
use crate::rasterizer::{Primitive, Rasterizer};
use utils::*;

fn main() -> Result<()> {
    let mut r = Rasterizer::new(700, 700);
    let eye_pos = Vector3::new(0.0, 0.0, 5.0);
    let pos = vec![Vector3::new(2.0, 0.0, -2.0),
                   Vector3::new(0.0, 2.0, -2.0),
                   Vector3::new(-2.0, 0.0, -2.0),
                   Vector3::new(3.5, -1.0, -5.0),
                   Vector3::new(2.5, 1.5, -5.0),
                   Vector3::new(-1.0, 0.5, -5.0),
                   Vector3::new(-3.5, -3.5, -6.0),
                   Vector3::new(3.5, 1.5, -6.0),
                   Vector3::new(-2.0, 2.5, -6.0)];
    let ind = vec![Vector3::new(0, 1, 2), Vector3::new(3, 4, 5), Vector3::new(6, 7, 8)];
    let cols = vec![Vector3::new(217.0, 238.0, 185.0),
                    Vector3::new(217.0, 238.0, 185.0),
                    Vector3::new(217.0, 238.0, 185.0),
                    Vector3::new(185.0, 217.0, 238.0),
                    Vector3::new(185.0, 217.0, 238.0),
                    Vector3::new(185.0, 217.0, 238.0),
                    Vector3::new(238.0, 185.0, 217.0),
                    Vector3::new(238.0, 185.0, 217.0),
                    Vector3::new(238.0, 185.0, 217.0)];
    let pos_id = r.load_position(&pos);
    let ind_id = r.load_indices(&ind);
    let col_id = r.load_colors(&cols);
    let mut k = 0;
    let mut frame_count = 0;

    while k != 27 {
        r.clear(rasterizer::Buffer::Both);
        r.set_model(get_model_matrix(0.0));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw(pos_id, ind_id, col_id, Primitive::Triangle);

        let frame_buffer = r.frame_buffer();
        let image = frame_buffer2cv_mat(frame_buffer);

        imshow("image", &image)?;
        k = wait_key(2000).unwrap();
        println!("frame count: {}", frame_count);
        frame_count += 1;
    }

    Ok(())
}