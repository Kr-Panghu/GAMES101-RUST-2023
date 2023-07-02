mod triangle;
mod rasterizer;
mod utils;
extern crate opencv;
use std::env;
use nalgebra::{Vector3};
use opencv::core::Vector;
use opencv::highgui::{imshow, wait_key};
use opencv::imgcodecs::imwrite;
use crate::rasterizer::{Primitive, Rasterizer};
use utils::*;

fn main() {
    let mut angle = 0.0;
    let mut command_line = false;
    let mut filename = "output.png";
    let argv: Vec<String> = env::args().collect();
    if argv.len() >= 2 {
        command_line = true;
        angle = argv[1].parse().unwrap();
        if argv.len() == 3 {
            filename = &argv[2];
        }
    }

    let mut r = Rasterizer::new(700, 700);
    let eye_pos = Vector3::new(0.0, 0.0, 5.0);
    let pos = vec![Vector3::new(2.0, 0.0, -2.0),
                   Vector3::new(0.0, 2.0, -2.0),
                   Vector3::new(-2.0, 0.0, -2.0)];
    let ind = vec![Vector3::new(0, 1, 2)];

    let pos_id = r.load_position(&pos);
    let ind_id = r.load_indices(&ind);

    let mut k = 0;
    let mut frame_count = 0;
    if command_line {
        r.clear(rasterizer::Buffer::Both);
        r.set_model(get_model_matrix(angle));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw_triangle(pos_id, ind_id, Primitive::Triangle);

        let frame_buffer = r.frame_buffer();
        let image = frame_buffer2cv_mat(frame_buffer);

        imwrite(filename, &image, &Vector::default()).unwrap();
        return;
    }
    while k != 27 {
        r.clear(rasterizer::Buffer::Both);
        r.set_model(get_model_matrix(angle));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw_triangle(pos_id, ind_id, Primitive::Triangle);

        let frame_buffer = r.frame_buffer();
        let image = frame_buffer2cv_mat(frame_buffer);
        imshow("image", &image).unwrap();

        k = wait_key(80).unwrap();
        println!("frame count: {}", frame_count);
        if k == 'a' as i32 {
            angle += 10.0;
        } else if k == 'd' as i32 {
            angle -= 10.0;
        } 
        frame_count += 1;
    }
}