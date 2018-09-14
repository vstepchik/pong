extern crate amethyst;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline, PosTex, RenderBundle, Stage};
use crate::pong::Pong;

mod pong;
mod systems;
mod components;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let config_path = format!("{}/resources/display_config.ron", env!("CARGO_MANIFEST_DIR"));
    let binding_path = format!("{}/resources/bindings_config.ron", env!("CARGO_MANIFEST_DIR"));

    let config = DisplayConfig::load(&config_path);
    let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]);
    let mut game = Application::new("./", Pong, game_data)?;
    game.run();
    Ok(())
}
