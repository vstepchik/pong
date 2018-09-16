use amethyst::core::Transform;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};
use crate::components::{Ball, Paddle, Side};
use crate::pong::ARENA_HEIGHT;

pub struct BounceSystem;

const BOUNCE_SPEEDUP: f32 = 0.2;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, paddles, transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation[0];
            let ball_y = transform.translation[1];

            if ball_y <= ball.radius && ball.velocity.1 < 0.0 {
                ball.velocity.1 = -ball.velocity.1;
            } else if ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity.1 > 0.0 {
                ball.velocity.1 = -ball.velocity.1;
            }

            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation[0] - paddle.width * 0.5;
                let paddle_y = paddle_transform.translation[1] - paddle.height * 0.5;

                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius,
                    paddle_y - ball.radius,
                    paddle_x + paddle.width + ball.radius,
                    paddle_y + paddle.height + ball.radius,
                ) {
                    if paddle.side == Side::Left && ball.velocity.0 < 0.0 {
                        ball.velocity.0 = -(ball.velocity.0 - BOUNCE_SPEEDUP);
                    } else if paddle.side == Side::Right && ball.velocity.0 > 0.0 {
                        ball.velocity.0 = -(ball.velocity.0 + BOUNCE_SPEEDUP);
                    }
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
