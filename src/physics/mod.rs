pub(crate) mod effect;
pub(crate) mod hitbox;
pub(crate) mod vec3;

#[derive(Debug, Copy, Clone)]
pub struct Vec3(f64, f64, f64);

#[derive(Debug, Copy, Clone)]
pub struct Hitbox {
    min: Vec3,
    max: Vec3,
}

#[derive(Debug, Clone)]
pub struct Effect {
    force: Vec3,
}
