use std::collections::HashMap;
use super::utils::V3d;

use nalgebra::{Matrix4, Vector3, Vector4};
use crate::triangle::Triangle;

type V4d = Vector4<f64>;

#[allow(dead_code)]
pub enum Buffer {
    Color,
    Depth,
    Both,
}

#[allow(dead_code)]
pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Default)]
pub struct Rasterizer {
    model: Matrix4<f64>,
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    pos_buf: HashMap<usize, Vec<V3d>>,
    ind_buf: HashMap<usize, Vec<Vector3<usize>>>,

    frame_buf: Vec<V3d>,
    depth_buf: Vec<f64>,
    width: u64,
    height: u64,
    next_id: usize,
}

#[derive(Clone, Copy)]
pub struct PosBufId(usize);

#[derive(Clone, Copy)]
pub struct IndBufId(usize);

#[derive(Clone, Copy)]
pub struct ColBufId(usize);

impl Rasterizer {
    pub fn new(w: u64, h: u64) -> Self {
        let mut r = Rasterizer::default();
        r.width = w;
        r.height = h;
        r.frame_buf.resize((w * h) as usize, Vector3::zeros());
        r.depth_buf.resize((w * h) as usize, 0.0);
        r
    }

    pub fn clear(&mut self, buff: Buffer) {
        match buff {
            Buffer::Color =>
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0)),
            Buffer::Depth =>
                self.depth_buf.fill(f64::MAX),
            Buffer::Both => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                self.depth_buf.fill(f64::MAX);
            }
        }
    }

    fn set_pixel(height: u64, width: u64, frame_buf: &mut Vec<V3d>, point: &V3d, color: &V3d) {
        if point.x < 0.0 || point.x >= width as f64 || point.y < 0.0 || point.y >= height as f64 {
            return;
        }
        let ind = (height as f64 - 1.0 - point.y) * width as f64 + point.x;
        let ind = ind as usize;
        frame_buf[ind] = *color;
    }

    fn draw_line(begin: &V3d, end: &V3d, height: u64, width: u64, frame_buf: &mut Vec<V3d>) {
        let (x1, y1) = (begin.x, begin.y);
        let (x2, y2) = (end.x, end.y);
        let line_color = Vector3::new(0.0, 255.0, 0.0);
        let (dx, dy, dx1, dy1, mut px, mut py): (f64, f64, f64, f64, f64, f64);

        dx = x2 - x1;
        dy = y2 - y1;
        dx1 = dx.abs();
        dy1 = dy.abs();
        px = 2.0 * dy1 - dx1;
        py = 2.0 * dx1 - dy1;

        if dy1 <= dx1 {
            let (mut x, mut y, xe) = if dx >= 0.0 {
                (x1, y1, x2)
            } else {
                (x2, y2, x1)
            };
            let point = V3d::new(x.round(), y.round(), 1.0);
            Self::set_pixel(height, width, frame_buf, &point, &line_color);
            while x < xe {
                x += 1.0;
                if px < 0.0 {
                    px += 2.0 * dy1;
                } else {
                    if (dx < 0.0 && dy < 0.0) || (dx > 0.0 && dy > 0.0) {
                        y += 1.0;
                    } else { y -= 1.0; }
                    px = px + 2.0 * (dy1 - dx1);
                }
                let point = V3d::new(x.round(), y.round(), 1.0);
                Self::set_pixel(height, width, frame_buf, &point, &line_color);
            }
        } else {
            let (mut x, mut y, ye) = if dy >= 0.0 {
                (x1, y1, y2)
            } else {
                (x2, y2, y1)
            };
            let point = V3d::new(x.round(), y.round(), 1.0);
            Self::set_pixel(height, width, frame_buf, &point, &line_color);
            while y < ye {
                y += 1.0;
                if py < 0.0 {
                    py += 2.0 * dx1;
                } else {
                    if (dx < 0.0 && dy < 0.0) || (dx > 0.0 && dy > 0.0) {
                        x += 1.0;
                    } else { x -= 1.0; }
                    py += 2.0 * (dx1 - dy1);
                }
                let point = V3d::new(x.round(), y.round(), 1.0);
                Self::set_pixel(height, width, frame_buf, &point, &line_color);
            }
        }
    }

    pub fn set_model(&mut self, model: Matrix4<f64>) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f64>) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Matrix4<f64>) {
        self.projection = projection;
    }
    fn get_next_id(&mut self) -> usize {
        let res = self.next_id;
        self.next_id += 1;
        res
    }
    pub fn load_position(&mut self, positions: &Vec<V3d>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.clone());
        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: &Vec<Vector3<usize>>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices.clone());
        IndBufId(id)
    }

    pub fn draw_triangle(&mut self, pos_buffer: PosBufId, ind_buffer: IndBufId, _typ: Primitive) {
        let buf = &self.pos_buf[&pos_buffer.0];
        let ind: &Vec<Vector3<usize>> = &self.ind_buf[&ind_buffer.0];

        let mvp = self.projection * self.view * self.model;

        for i in ind {
            let t = Rasterizer::get_triangle(self.width, self.height, buf, mvp, i);
            Self::draw_line(&t.v[2], &t.v[0], self.height, self.width, &mut self.frame_buf);
            Self::draw_line(&t.v[0], &t.v[1], self.height, self.width, &mut self.frame_buf);
            Self::draw_line(&t.v[1], &t.v[2], self.height, self.width, &mut self.frame_buf);
        }
    }

    fn get_triangle(width: u64, height: u64, buf: &Vec<V3d>, mvp: Matrix4<f64>, i: &Vector3<usize>) -> Triangle {
        let f1 = (50.0 - 0.1) / 2.0;
        let f2 = (50.0 + 0.1) / 2.0;

        let mut t = Triangle::new();
        let mut v =
            vec![mvp * to_vec4(buf[i[0]], Some(1.0)),
                 mvp * to_vec4(buf[i[1]], Some(1.0)),
                 mvp * to_vec4(buf[i[2]], Some(1.0))];

        for vec in v.iter_mut() {
            *vec = *vec / vec.w;
        }
        for vert in v.iter_mut() {
            vert.x = 0.5 * width as f64 * (vert.x + 1.0);
            vert.y = 0.5 * height as f64 * (vert.y + 1.0);
            vert.z = vert.z * f1 + f2;
        }
        for j in 0..3 {
            t.set_vertex(j, v[j].xyz());
        }

        t.set_color(0, 255.0, 0.0, 0.0);
        t.set_color(1, 0.0, 255.0, 0.0);
        t.set_color(2, 0.0, 0.0, 255.0);
        t
    }

    pub fn frame_buffer(&self) -> &Vec<V3d> {
        &self.frame_buf
    }
}

fn to_vec4(v3: V3d, w: Option<f64>) -> V4d {
    Vector4::new(v3.x, v3.y, v3.z, w.unwrap_or(1.0))
}