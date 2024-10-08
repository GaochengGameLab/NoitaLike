mod game;
mod renderer;
mod world;

pub use game::*;
pub use renderer::*;

use sdl2::{rect::Rect, render::Canvas, video::Window};

pub type RenderCanvas = Canvas<Window>;

pub struct TransformStack {
    stack: Vec<Transform>
}

impl TransformStack {
    pub fn new() -> Self {
        TransformStack {
            stack: vec![
                Transform {
                    translate_x: 0.0,
                    translate_y: 0.0,
                    scale_x: 1.0,
                    scale_y: 1.0,
                }
            ]
        }
    }

    pub fn push(&mut self){
        self.stack.push(self.stack.last().unwrap().clone());
    }

    pub fn pop(&mut self){
        self.stack.pop();
    }

    pub fn translate<T: Into<f64>>(&mut self, x: T, y: T){
        self.stack.last_mut().unwrap().translate_x += x.into() / self.stack.last_mut().unwrap().scale_x;
        self.stack.last_mut().unwrap().translate_y += y.into() / self.stack.last_mut().unwrap().scale_y;
    }

    pub fn scale<T: Into<f64>>(&mut self, x: T, y: T){
        self.stack.last_mut().unwrap().scale_x *= x.into();
        self.stack.last_mut().unwrap().scale_y *= y.into();
    }

    pub fn transform<T: Into<f64>>(&self, point: (T, T)) -> (f64, f64) {
        let t = self.stack.last().unwrap();
        ((point.0.into() + t.translate_x) * t.scale_x,
         (point.1.into() + t.translate_y) * t.scale_y)
    }

    pub fn transform_int<T: Into<f64>>(&self, point: (T, T)) -> (i32, i32) {
        let t = self.stack.last().unwrap();
        (((point.0.into() + t.translate_x) * t.scale_x) as i32,
         ((point.1.into() + t.translate_y) * t.scale_y) as i32)
    }

    pub fn transform_rect(&self, rect: Rect) -> Rect {/**/
        let pos = self.transform_int((rect.x, rect.y));

        let t = self.stack.last().unwrap();
        Rect::new(pos.0, pos.1, (rect.w as f64 * t.scale_x) as u32, (rect.h as f64 * t.scale_y) as u32)
    }

    pub fn inv_transform<T: Into<f64>>(&self, point: (T, T)) -> (f64, f64) {
        let t = self.stack.last().unwrap();
        (point.0.into() / t.scale_x - t.translate_x,
         point.1.into() / t.scale_y - t.translate_y)
    }

    pub fn inv_transform_int<T: Into<f64>>(&self, point: (T, T)) -> (i32, i32) {
        let t = self.stack.last().unwrap();
        ((point.0.into() / t.scale_x - t.translate_x) as i32,
         (point.1.into() / t.scale_y - t.translate_y) as i32)
    }

    pub fn inv_transform_rect(&self, rect: Rect) -> Rect {
        let pos = self.inv_transform_int((rect.x, rect.y));

        let t = self.stack.last().unwrap();
        Rect::new(pos.0, pos.1, (rect.w as f64 / t.scale_x) as u32, (rect.h as f64 / t.scale_y) as u32)
    }

}

#[derive(Clone)]
struct Transform {
    translate_x: f64,
    translate_y: f64,
    scale_x: f64,
    scale_y: f64,
}

trait Renderable {
    fn render(&self, canvas : &mut Canvas<Window>, transform: &mut TransformStack, game: &Game);
}