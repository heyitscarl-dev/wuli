use glam::{DVec2, dvec2};
use macroquad::{
    camera::Camera2D,
    math::Vec2,
    window::{screen_height, screen_width},
};

pub struct View {
    pub centre: DVec2,
    pub width: f64,
}

impl View {
    pub fn new(cx: f64, cy: f64, width: f64) -> Self {
        Self {
            centre: dvec2(cx, cy),
            width,
        }
    }

    pub fn camera(&self) -> Camera2D {
        let w = self.width as f32;
        let h = w * screen_height() / screen_width();
        Camera2D {
            target: macroquad::math::vec2(self.centre.x as f32, self.centre.y as f32),
            zoom: macroquad::math::vec2(2.0 / w, -2.0 / h),
            ..Default::default()
        }
    }

    pub fn to_camera(&self, p: DVec2) -> Vec2 {
        let r = p - self.centre;
        macroquad::math::vec2(r.x as f32, r.y as f32)
    }
}
