use amethyst::core::Transform;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};
use crate::components::{Ball, Paddle, Side};
use crate::pong::{ARENA_HEIGHT, ARENA_WIDTH};

pub struct WinConditionSystem;

impl<'s> System<'s> for WinConditionSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, mut transforms): Self::SystemData) {
        for (mut ball, mut transform) in (&mut balls, &mut transforms).join() {
            let ball_x = transform.translation[0];
            let ball_y = transform.translation[1];

            if ball_x <= ball.radius && ball.velocity.0 < 0.0 {
                reset_ball_velocity(&mut ball);
                reset_ball_position(transform);
            } else if ball_x >= ARENA_WIDTH - ball.radius && ball.velocity.0 > 0.0 {
                reset_ball_velocity(&mut ball);
                reset_ball_position(transform);
            }
        }
    }
}

fn reset_ball_velocity(ball: &mut Ball) {
    ball.velocity.0 = 0.0;
    ball.velocity.1 = 0.0;
}

fn reset_ball_position(trans: &mut Transform) {
    trans.translation[0] = ARENA_WIDTH * 0.5;
    trans.translation[1] = ARENA_HEIGHT * 0.5;
}