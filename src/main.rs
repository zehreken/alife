use amethyst::core::{Transform, TransformBundle};
use amethyst::prelude::Builder;
use amethyst::prelude::World;
use amethyst::renderer::Camera;
use amethyst::renderer::{
    plugins::{RenderPbr3D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
};
use amethyst::utils::application_root_dir;
use amethyst::window::DisplayConfig;
use amethyst::Application;
use amethyst::GameData;
use amethyst::GameDataBuilder;
use amethyst::SimpleState;
use amethyst::StateData;

use amethyst::assets::AssetLoaderSystemData;
use amethyst::core::timing::Time;
use amethyst::prelude::*;
use amethyst::renderer::light::{Light, PointLight};
use amethyst::renderer::mtl::{Material, MaterialDefaults};
use amethyst::renderer::palette::{rgb::Rgb, LinSrgba};
use amethyst::renderer::rendy::mesh::{Normal, Position, Tangent, TexCoord};
use amethyst::renderer::rendy::texture::palette::load_from_linear_rgba;
use amethyst::renderer::shape::Shape;
use amethyst::renderer::Mesh;
use amethyst::renderer::Texture;

mod alife;
mod systems;
use crate::alife::Alife;

fn main() -> amethyst::Result<()> {
    // Set up the Amethyst logger
    amethyst::start_logger(Default::default());

    // Set up the assets directory (PathBuf)
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    // Set up the display configuration
    let display_config = DisplayConfig {
        title: "Alife".to_string(),
        dimensions: Some((960, 540)),
        ..Default::default()
    };

    // Set up the GameDataBuilder
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(display_config).with_clear([0.55, 0.55, 0.55, 1.0]),
                )
                .with_plugin(RenderPbr3D::default()),
        )?;

    // Run the game!
    let mut game = Application::new(assets_dir, Alife::default(), game_data)?;
    game.run();

    Ok(())
}
