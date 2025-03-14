use meshtext::MeshGenerator;

pub const MONOGRAM: &[u8] = include_bytes!("../resources/fonts/monogram-extended.ttf");

#[must_use]
pub fn from_bytes(bytes: &'static [u8]) -> MeshGenerator<meshtext::Face<'static>> {
    MeshGenerator::new(bytes)
}
