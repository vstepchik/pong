use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use crate::components::Ball;

const BALL_SPEED: f32 = 50.0;

pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, balls, time): Self::SystemData) {
        for (ball, transform) in (&balls, &mut transforms).join() {
            transform.translation[0] += ball.velocity.0 * BALL_SPEED * time.delta_seconds();
            transform.translation[1] += ball.velocity.1 * BALL_SPEED * time.delta_seconds();
        }
    }
}
