use amethyst::assets::{PrefabLoader, PrefabLoaderSystem, RonFormat};
use amethyst::core::{Transform, TransformBundle};
use amethyst::prelude::Builder;
use amethyst::prelude::World;
use amethyst::prelude::*;
use amethyst::renderer::rendy::mesh::{Normal, Position, Tangent, TexCoord};
use amethyst::renderer::Camera;
use amethyst::renderer::{
    plugins::{RenderPbr3D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
};
use amethyst::utils::{application_root_dir, scene::BasicScenePrefab};
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
use amethyst::renderer::rendy::texture::palette::load_from_linear_rgba;
use amethyst::renderer::shape::Shape;
use amethyst::renderer::Mesh;
use amethyst::renderer::Texture;
use amethyst::core::math::Vector3;

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Default)]
pub struct Alife {}

impl SimpleState for Alife {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // let handle = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
        //     loader.load("prefab/plane.ron", RonFormat, ())
        // });
        // data.world.create_entity().with(handle).build();

        // let handle = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
        //     loader.load("prefab/camera.ron", RonFormat, ())
        // });
        // data.world.create_entity().with(handle).build();
        initialize_camera(data.world);
        initialize_shapes(data.world);
        initialize_light(data.world, -2.0, 2.0, 20.0);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    world
        .create_entity()
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

    let plane_mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Plane(Some((1, 1)))
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
    });

    let mut cone_transform = Transform::default();
    cone_transform.set_translation_xyz(-2.0, -2.0, 0.0);
    cone_transform.set_rotation_x_axis(-std::f32::consts::PI / 3.0);

    world
        .create_entity()
        .with(cone_mesh.clone())
        .with(mtl.clone())
        .with(cone_transform)
        .build();

    let mut sphere_transform = Transform::default();
    sphere_transform.set_translation_xyz(-2.0, 2.0, 0.0);
    sphere_transform.set_rotation_x_axis(-std::f32::consts::PI / 3.0);

    world
        .create_entity()
        .with(sphere_mesh.clone())
        .with(mtl.clone())
        .with(sphere_transform)
        .build();

    let mut cube_transform = Transform::default();
    cube_transform.set_translation_xyz(2.0, -2.0, 0.0);
    cube_transform.set_rotation_x_axis(-std::f32::consts::PI / 3.0);

    world
        .create_entity()
        .with(cube_mesh.clone())
        .with(mtl.clone())
        .with(cube_transform)
        .build();

    let mut cylinder_transform = Transform::default();
    cylinder_transform.set_translation_xyz(2.0, 2.0, 0.0);
    cylinder_transform.set_rotation_x_axis(-std::f32::consts::PI / 3.0);

    world
        .create_entity()
        .with(cylinder_mesh.clone())
        .with(mtl.clone())
        .with(cylinder_transform)
        .build();

    let mut plane_transform = Transform::default();
    plane_transform.set_translation_xyz(0.0, 0.0, 0.0);
    plane_transform.set_scale(Vector3::new(10.0, 10.0, 10.0));
    plane_transform.set_rotation_x_axis(-std::f32::consts::PI / 3.0);

    world
        .create_entity()
        .with(plane_mesh.clone())
        .with(mtl.clone())
        .with(plane_transform)
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
