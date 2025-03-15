use js_sys::Math;

pub fn rand() -> f64 {
    Math::random()
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    rand() * (max - min) + min
}
