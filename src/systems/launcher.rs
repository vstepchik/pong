use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;
use crate::components::Ball;
use rand::random;
use winit::VirtualKeyCode;

const BALL_SPEED: f32 = 50.0;

pub struct LauncherSystem;

impl<'s> System<'s> for LauncherSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut balls, input): Self::SystemData) {
        let launch = input.key_is_down(VirtualKeyCode::Space);
        for mut ball in (&mut balls).join() {
            if launch && ball.velocity.0.abs() <= 0.1 {
                ball.velocity.0 = -1.0 + random::<f32>() * 2.0;
                ball.velocity.1 = -1.0 + random::<f32>() * 2.0;
            }
        }
    }
}
