use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;
use crate::components::Ball;
use rand::random;
use std::f32::consts::PI;
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
                let x_speed = (PI * random::<f32>()).sin();
                let y_speed = (PI * random::<f32>()).cos();
                ball.velocity.0 = x_speed;
                ball.velocity.1 = y_speed;
            }
        }
    }
}
