use meshtext::{MeshGenerator, QualitySettings};

pub const MONOGRAM: &[u8] = include_bytes!("../resources/fonts/monogram-extended.ttf");

pub type Font = MeshGenerator<meshtext::Face<'static>>;

#[must_use]
pub fn from_bytes(bytes: &'static [u8]) -> Font {
    let mut settings = QualitySettings::default();

    settings.cubic_interpolation_steps = 1;
    settings.quad_interpolation_steps = 1;

    MeshGenerator::new_with_quality(bytes, settings)
}
