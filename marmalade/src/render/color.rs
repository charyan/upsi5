use glam::Vec4;

#[must_use]
pub const fn rgb(r: f32, g: f32, b: f32) -> Vec4 {
    rgba(r, g, b, 1.)
}

#[must_use]
pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Vec4 {
    Vec4::new(r, g, b, a)
}

pub const WHITE: Vec4 = rgb(1., 1., 1.);
