use amethyst::assets::AssetLoaderSystemData;
use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use amethyst::prelude::Builder;
use amethyst::prelude::World;
use amethyst::prelude::*;
use amethyst::renderer::camera::Camera;
use amethyst::renderer::light::{Light, PointLight};
use amethyst::renderer::mtl::{Material, MaterialDefaults};
use amethyst::renderer::palette::{rgb::Rgb, LinSrgba};
use amethyst::renderer::rendy::mesh::{Normal, Position, Tangent, TexCoord};
use amethyst::renderer::rendy::texture::palette::load_from_linear_rgba;
use amethyst::renderer::shape::Shape;
use amethyst::renderer::Mesh;
use amethyst::renderer::Texture;
use amethyst::ui::{Anchor, FontAsset, TtfFormat, UiText, UiTransform};
use amethyst::GameData;
use amethyst::SimpleState;
use amethyst::StateData;

use crate::systems::plant_system::*;

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

        data.world.register::<Living>(); // Necessary to be able to use the component, https://book.amethyst.rs/master/pong-tutorial/pong-tutorial-02.html

        initialize_camera(data.world);
        initialize_shapes(data.world);
        initialize_light(data.world, -2.0, 2.0, 20.0);
        initialize_ui(data.world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 3.0, 20.0);

    let width = 960.0;
    let height = 540.0;
    let camera = Camera::standard_3d(width, height);
    // camera.set_projection(Projection::orthographic(-9.6, 9.6, -5.4, 5.4, 0.0, 20.0));

    world.create_entity().with(camera).with(transform).build();
}

fn initialize_shapes(world: &mut World) {
    // create_cone(world);
    // create_sphere(world);
    // create_cube(world);
    // create_cylinder(world);
    create_plane(world);
    for i in 0..10 {
        for j in 0..10 {
            create_sphere(
                world,
                Vector3::new(i as f32 * 2.0 - 9.0, j as f32 * 2.0 - 9.0, 0.0),
            );
        }
    }
}

fn create_material(world: &mut World, albedo: LinSrgba) -> Handle<Material> {
    let mat_defaults = world.read_resource::<MaterialDefaults>().0.clone();

    let albedo = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
        loader.load_from_data(load_from_linear_rgba(albedo).into(), ())
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

    mtl
}

fn create_cone(world: &mut World) {
    let cone_mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Cone(100)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let mtl = create_material(world, LinSrgba::new(0.1, 0.0, 0.0, 1.0));

    let mut cone_transform = Transform::default();
    cone_transform.set_translation_xyz(-2.0, 2.0, 0.0);
    // cone_transform.set_rotation_x_axis(-std::f32::consts::PI / 3.0);

    world
        .create_entity()
        .with(cone_mesh.clone())
        .with(mtl.clone())
        .with(cone_transform)
        .build();
}

fn create_sphere(world: &mut World, position: Vector3<f32>) {
    let sphere_mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Sphere(10, 10)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let mtl = create_material(world, LinSrgba::new(0.0, 0.01, 0.0, 1.0));

    let mut sphere_transform = Transform::default();
    sphere_transform.set_translation_xyz(position.x, position.y, position.z);
    sphere_transform.set_scale(Vector3::new(0.5, 0.5, 0.5));
    // cone_transform.set_rotation_x_axis(-std::f32::consts::PI / 3.0);

    world
        .create_entity()
        .with(sphere_mesh.clone())
        .with(mtl.clone())
        .with(sphere_transform)
        .with(Plant {})
        .with(Living { age: 0 })
        .build();
}

fn create_cube(world: &mut World, position: Vector3<f32>) {
    let cube_mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Cube
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let mtl = create_material(world, LinSrgba::new(0.1, 0.0, 0.0, 1.0));

    let mut cube_transform = Transform::default();
    cube_transform.set_translation_xyz(position.x, position.y, position.z);
    cube_transform.set_scale(Vector3::new(0.5, 0.5, 0.5));
    // cube_transform.set_rotation_x_axis(-std::f32::consts::PI / 3.0);

    world
        .create_entity()
        .with(cube_mesh.clone())
        .with(mtl.clone())
        .with(cube_transform)
        .build();
}

fn create_cylinder(world: &mut World) {
    let cylinder_mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Cylinder(100, None)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let mtl = create_material(world, LinSrgba::new(0.1, 0.0, 0.0, 1.0));

    let mut cylinder_transform = Transform::default();
    cylinder_transform.set_translation_xyz(0.0, 2.0, 0.0);
    // cylinder_transform.set_rotation_x_axis(-std::f32::consts::PI / 3.0);

    world
        .create_entity()
        .with(cylinder_mesh.clone())
        .with(mtl.clone())
        .with(cylinder_transform)
        .build();
}

fn create_plane(world: &mut World) {
    let plane_mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Plane(Some((1, 1)))
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let mtl = create_material(world, LinSrgba::new(0.0, 0.1, 0.0, 1.0));

    let mut plane_transform = Transform::default();
    plane_transform.set_translation_xyz(0.0, 0.0, 0.0);
    plane_transform.set_scale(Vector3::new(10.0, 10.0, 10.0));
    // plane_transform.set_rotation_x_axis(-std::f32::consts::PI / 2.0);

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

fn initialize_ui(world: &mut World) {
    // let font = world.read_resource::<Loader>().load(
    //     "font/square.ttf",
    //     TtfFormat,
    //     (),
    //     &world.read_resource::<AssetStorage<FontAsset>>(),
    // );

    // let text_transform = UiTransform::new(
    //     "info".to_string(),
    //     Anchor::TopLeft,
    //     Anchor::TopLeft,
    //     0.0,
    //     0.0,
    //     1.0,
    //     200.0,
    //     200.0,
    // );

    // let text_entity = world
    //     .create_entity()
    //     .with(text_transform)
    //     .with(UiText::new(
    //         font.clone(),
    //         "info text".to_string(),
    //         [1.0, 1.0, 1.0, 1.0],
    //         20.0,
    //     ))
    //     .build();
}
