use std::os::raw::c_void;
use nalgebra::{Matrix4, Vector3};
use opencv::core::{Mat, MatTraitConst};
use opencv::imgproc::{COLOR_RGB2BGR, cvt_color};

pub(crate) fn get_view_matrix(eye_pos: Vector3<f64>) -> Matrix4<f64> {
    let mut view: Matrix4<f64> = Matrix4::identity();
    /*  implement what you've done in LAB1  */

    view
}

pub(crate) fn get_model_matrix(rotation_angle: f64) -> Matrix4<f64> {
    let mut model: Matrix4<f64> = Matrix4::identity();
    /*  implement what you've done in LAB1  */

    model
}

pub(crate) fn get_projection_matrix(eye_fov: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Matrix4<f64> {
    let mut projection: Matrix4<f64> = Matrix4::identity();
    /*  implement what you've done in LAB1  */

    projection
}


pub(crate) fn frame_buffer2cv_mat(frame_buffer: &Vec<Vector3<f64>>) -> opencv::core::Mat {
    let mut image = unsafe {
        Mat::new_rows_cols_with_data(
            700, 700,
            opencv::core::CV_64FC3,
            frame_buffer.as_ptr() as *mut c_void,
            opencv::core::Mat_AUTO_STEP,
        ).unwrap()
    };
    let mut img = Mat::copy(&image).unwrap();
    image.convert_to(&mut img, opencv::core::CV_8UC3, 1.0, 1.0).expect("panic message");
    cvt_color(&img, &mut image, COLOR_RGB2BGR, 0).unwrap();
    image
}