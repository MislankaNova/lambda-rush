// Vector helpers

extern crate sfml;

use self::sfml::system::Vector2f;

pub fn distance(p1 : Vector2f, p2 : Vector2f) -> f32 {
    let dp = p2 - p1;
    (dp.x * dp.x + dp.y * dp.y).sqrt()
}

pub fn dot_product(p1 : Vector2f, p2 : Vector2f) -> f32 {
    p1.x * p2.x + p1.y * p2.y
}

pub fn cross_product(p1 : Vector2f, p2 : Vector2f) -> f32 {
    p1.x * p2.y - p2.x * p1.y
}

pub fn polar_to_vector((r, t) : (f32, f32)) -> Vector2f {
    Vector2f::new(
        r * t.to_radians().sin(),
        r * t.to_radians().cos()
    )
}

pub fn normalise(vector : Vector2f, r : f32) -> Vector2f {
    let b = 1.0 / (vector.x * vector.x + vector.y * vector.y).sqrt();
    Vector2f::new(
        vector.x * b * r,
        vector.y * b * r
    )
}

pub fn times(vector : Vector2f, r : f32) -> Vector2f {
    Vector2f::new(
        vector.x * r,
        vector.y * r
    )
}
