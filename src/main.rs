use amethyst::utils::application_root_dir;
use amethyst::SimpleState;
use amethyst::GameDataBuilder;
use amethyst::Application;
use amethyst::renderer::{
    plugins::{RenderPbr3D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
};
use amethyst::window::DisplayConfig;
use amethyst::StateData;
use amethyst::GameData;
use amethyst::prelude::World;
use amethyst::prelude::Builder;
use amethyst::renderer::Camera;
use amethyst::core::{Transform, TransformBundle};

use amethyst::assets::AssetLoaderSystemData;
use amethyst::renderer::Mesh;
use amethyst::renderer::rendy::mesh::{Normal, Position, Tangent, TexCoord};
use amethyst::renderer::shape::Shape;
use amethyst::renderer::Texture;
use amethyst::renderer::palette::{LinSrgba, rgb::Rgb};
use amethyst::renderer::rendy::texture::palette::load_from_linear_rgba;
use amethyst::renderer::mtl::{Material, MaterialDefaults};
use amethyst::renderer::light::{Light, PointLight};
use amethyst::core::timing::Time;
use amethyst::prelude::*;

struct GameState;
impl SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        initialize_camera(state_data.world);
        initialize_shapes(state_data.world);
        initialize_light(state_data.world, -2.0, 2.0, 20.0);
    }
}

fn main() -> amethyst::Result<()> {
    // Set up the Amethyst logger
    amethyst::start_logger(Default::default());

    // Set up the assets directory (PathBuf)
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    // Set up the display configuration
    let display_config = DisplayConfig {
        title: "Amethyst".to_string(),
        dimensions: Some((960, 540)),
        ..Default::default()
    };

    // Set up the GameDataBuilder
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(display_config)
                        .with_clear([0.55, 0.55, 0.55, 1.0]),
                )
                .with_plugin(RenderPbr3D::default()),
        )?;

    // Run the game!
    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    world.create_entity()
        .with(Camera::standard_3d(960.0, 540.0))
        .with(transform)
        .build();
}

fn initialize_shapes(world: &mut World) {
    let mat_defaults = world.read_resource::<MaterialDefaults>().0.clone();

    let cone_mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Cone(100)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let sphere_mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Sphere(100, 100)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let cube_mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Cube
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let cylinder_mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Cylinder(100, None)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let albedo = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
        loader.load_from_data(
            load_from_linear_rgba(LinSrgba::new(0.0, 0.0, 1.0, 1.0)).into(),
            (),
        )
    });

    let metallic_roughness = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
        loader.load_from_data(
            load_from_linear_rgba(LinSrgba::new(0.0, 1.0, 0.1, 0.0)).into(),
            (),
        )
    });

    let mtl = world.exec(|mtl_loader: AssetLoaderSystemData<'_, Material>| {
            mtl_loader.load_from_data(
                Material {
                    albedo: albedo,
                    metallic_roughness,
                    ..mat_defaults.clone()
                },
                (),
            )
        },
    );


    let mut cone_transform = Transform::default();
    cone_transform.set_translation_xyz(-2.0, -2.0, 0.0);
    cone_transform.set_rotation_x_axis(-std::f32::consts::PI / 3.0);

    world.create_entity()
        .with(cone_mesh.clone())
        .with(mtl.clone())
        .with(cone_transform)
        .build();

    let mut sphere_transform = Transform::default();
    sphere_transform.set_translation_xyz(-2.0, 2.0, 0.0);
    sphere_transform.set_rotation_x_axis(-std::f32::consts::PI / 3.0);

    world.create_entity()
        .with(sphere_mesh.clone())
        .with(mtl.clone())
        .with(sphere_transform)
        .build();

    let mut cube_transform = Transform::default();
    cube_transform.set_translation_xyz(2.0, -2.0, 0.0);
    cube_transform.set_rotation_x_axis(-std::f32::consts::PI / 3.0);

    world.create_entity()
        .with(cube_mesh.clone())
        .with(mtl.clone())
        .with(cube_transform)
        .build();

    let mut cylinder_transform = Transform::default();
    cylinder_transform.set_translation_xyz(2.0, 2.0, 0.0);
    cylinder_transform.set_rotation_x_axis(-std::f32::consts::PI / 3.0);

    world.create_entity()
        .with(cylinder_mesh.clone())
        .with(mtl.clone())
        .with(cylinder_transform)
        .build();
}

fn initialize_light(world: &mut World, x: f32, y: f32, z: f32) {
    let light1: Light = PointLight {
        intensity: 50.0,
        color: Rgb::new(1.0, 1.0, 1.0),
        radius: 0.1,
        ..PointLight::default()
    }
        .into();

    let mut light1_transform = Transform::default();
    light1_transform.set_translation_xyz(x, y, z);

    world
        .create_entity()
        .with(light1)
        .with(light1_transform)
        .build();
}