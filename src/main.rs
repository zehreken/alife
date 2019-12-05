mod pong;
use crate::pong::Pong;
use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    // We'll put the rest of the code here.
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let mut game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default()),
    )?;
    game_data = game_data.with_bundle(TransformBundle::new())?;

    let assets_dir = app_root.join("assets");
    let mut world = World::new();
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();

    std::thread::sleep(std::time::Duration::from_millis(20));

    Ok(())
}
