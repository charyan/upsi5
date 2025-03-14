use meshtext::MeshGenerator;

pub const MONOGRAM: &[u8] = include_bytes!("../resources/fonts/monogram-extended.ttf");

pub type Font = MeshGenerator<meshtext::Face<'static>>;

#[must_use]
pub fn from_bytes(bytes: &'static [u8]) -> Font {
    MeshGenerator::new(bytes)
}
