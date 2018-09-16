use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Ball {
    pub radius: f32,
    pub velocity: (f32, f32),
}

impl Ball {
    pub fn new(diameter: f32) -> Self {
        Ball { radius: diameter / 2.0, velocity: (0.0, 0.0) }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
