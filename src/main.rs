use amethyst::core::TransformBundle;
// use amethyst::prelude::Builder;
// use amethyst::prelude::World;
// use amethyst::renderer::Camera;
use amethyst::input::StringBindings;
use amethyst::renderer::{
    plugins::{RenderPbr3D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
};
use amethyst::utils::application_root_dir;
use amethyst::window::DisplayConfig;
use amethyst::Application;
// use amethyst::GameData;
use amethyst::GameDataBuilder;
// use amethyst::SimpleState;
// use amethyst::StateData;

// use amethyst::assets::AssetLoaderSystemData;
// use amethyst::core::timing::Time;
// use amethyst::renderer::light::{Light, PointLight};
// use amethyst::renderer::mtl::{Material, MaterialDefaults};
// use amethyst::renderer::palette::{rgb::Rgb, LinSrgba};
// use amethyst::renderer::rendy::mesh::{Normal, Position, Tangent, TexCoord};
// use amethyst::renderer::rendy::texture::palette::load_from_linear_rgba;
// use amethyst::renderer::shape::Shape;
// use amethyst::renderer::Mesh;
// use amethyst::renderer::Texture;
use amethyst::renderer::RenderFlat2D;

use amethyst::ui::{RenderUi, UiBundle};

pub struct Pong;

impl amethyst::prelude::SimpleState for Pong {}

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
    let display_config_path = app_root.join("config").join("display.ron");
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.25, 0.55, 1.0]),
                )
                .with_plugin(RenderPbr3D::default()), // .with_plugin(RenderUi::default()),
        )?
        // .with_bundle(UiBundle::<StringBindings>::new())?;
        .with(systems::RotateCameraSystem, "rotate_camera_system", &[])
        .with(systems::PlantSystem, "plant_system", &[]);

    // Run the game!
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();

    Ok(())
}
