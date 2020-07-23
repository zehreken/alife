use amethyst::core::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::renderer::{
    plugins::{RenderPbr3D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
};
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::utils::application_root_dir;
use amethyst::Application;
use amethyst::GameDataBuilder;

mod alife;
mod systems;
use alife::Alife;

fn main() -> amethyst::Result<()> {
    // Set up the Amethyst logger
    amethyst::start_logger(Default::default());

    // Set up the assets directory (PathBuf)
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    let binding_config_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_config_path)
        .unwrap();

    // Set up the display configuration
    let display_config_path = app_root.join("config").join("display.ron");
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .unwrap()
                        .with_clear([0.0, 0.25, 0.55, 1.0]),
                )
                .with_plugin(RenderPbr3D::default()), // .with_plugin(RenderUi::default()),
        )?
        // .with_bundle(UiBundle::<StringBindings>::new())?;
        .with(systems::RotateCameraSystem, "rotate_camera_system", &[])
        .with(systems::PlantSystem, "plant_system", &[]);

    // Run the game!
    let mut game = Application::new(assets_dir, Alife::default(), game_data)?;
    game.run();

    Ok(())
}
