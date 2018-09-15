use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Ball {
    pub diameter: f32,
    pub velocity: (f32, f32),
}

impl Ball {
    pub fn new(diameter: f32) -> Self {
        Ball { diameter, velocity: (0.0, 0.0) }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
