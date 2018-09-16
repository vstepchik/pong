use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Event, PngFormat, Projection, Sprite, Texture, TextureHandle,
    VirtualKeyCode, WithSpriteRender,
};
use crate::components::{Paddle, Side};
use crate::components::Ball;


pub struct Pong;

pub const ARENA_WIDTH: f32 = 400.0;
pub const ARENA_HEIGHT: f32 = 300.0;

const SPRITESHEET_SIZE: (f32, f32) = (8.0, 16.0);

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub const BALL_SPRITE_SIZE: f32 = 4.0;

pub const BALL_DIAMETER: f32 = 3.0;

impl Pong {
    fn initialise_camera(world: &mut World) {
        world.create_entity()
            .with(Camera::from(Projection::orthographic(0.0, ARENA_WIDTH, ARENA_HEIGHT, 0.0)))
            .with(GlobalTransform(
                Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into()
            ))
            .build();
    }

    fn initialise_paddles(world: &mut World, spritesheet: &TextureHandle) {
        let paddle_sprite = Sprite {
            left: 0.0,
            right: PADDLE_WIDTH,
            top: 0.0,
            bottom: PADDLE_HEIGHT,
        };

        let mut left_transform = Transform::default();
        let mut right_transform = Transform::default();

        // Correctly position the paddles.
        let y = ARENA_HEIGHT / 2.0;
        left_transform.translation = Vector3::new(PADDLE_WIDTH * 0.5, y, 0.0);
        right_transform.translation = Vector3::new(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

        // Create a left plank entity.
        world
            .create_entity()
            .with_sprite(&paddle_sprite, spritesheet.clone(), SPRITESHEET_SIZE)
            .expect("Failed to add sprite render on left paddle")
            .with(Paddle::new(Side::Left))
            .with(GlobalTransform::default())
            .with(left_transform)
            .build();

        // Create right plank entity.
        world
            .create_entity()
            .with_sprite(&paddle_sprite, spritesheet.clone(), SPRITESHEET_SIZE)
            .expect("Failed to add sprite render on right paddle")
            .with(Paddle::new(Side::Right))
            .with(GlobalTransform::default())
            .with(right_transform)
            .build();
    }

    fn initialise_ball(world: &mut World, spritesheet: &TextureHandle) {
        let ball_sprite = Sprite {
            left: PADDLE_WIDTH,
            right: PADDLE_WIDTH + BALL_SPRITE_SIZE,
            top: 0.0,
            bottom: BALL_SPRITE_SIZE,
        };

        let mut ball_transform = Transform::default();

        ball_transform.translation = Vector3::new(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

        world
            .create_entity()
            .with_sprite(&ball_sprite, spritesheet.clone(), SPRITESHEET_SIZE)
            .expect("Failed to add sprite render on right paddle")
            .with(Ball::new(BALL_DIAMETER))
            .with(GlobalTransform::default())
            .with(ball_transform)
            .build();
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for Pong {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        world.register::<Paddle>();
        world.register::<Ball>();

        // Load the spritesheet necessary to render the graphics.
        let spritesheet = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "resources/pong_spritesheet.png",
                PngFormat,
                Default::default(),
                (),
                &texture_storage,
            )
        };

        Self::initialise_paddles(world, &spritesheet);
        Self::initialise_ball(world, &spritesheet);
        Self::initialise_camera(world);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}
