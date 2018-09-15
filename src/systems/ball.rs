use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use crate::components::Ball;
use crate::pong::{ARENA_HEIGHT, ARENA_WIDTH, PADDLE_HEIGHT, PADDLE_WIDTH};

pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, balls, input): Self::SystemData) {
        for (ball, mut transform) in (&balls, &mut transforms).join() {
            transform.translation[0] += ball.velocity.0;
            transform.translation[1] += ball.velocity.1;
        }
    }
}
