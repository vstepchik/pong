use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;
use crate::components::Ball;
use rand::random;
use winit::VirtualKeyCode;

pub struct LauncherSystem;

impl<'s> System<'s> for LauncherSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut balls, input): Self::SystemData) {
        let launch = input.key_is_down(VirtualKeyCode::Space);
        for ball in (&mut balls).join() {
            if launch && ball.velocity.0.abs() <= 0.1 {
                ball.velocity.0 = -1.0 + random::<f32>() * 2.0;
                ball.velocity.1 = -1.0 + random::<f32>() * 2.0;
            }
        }
    }
}
