use amethyst::assets::{PrefabLoader, PrefabLoaderSystem, RonFormat};
use amethyst::prelude::*;
use amethyst::renderer::rendy::mesh::{Normal, Position, TexCoord};
use amethyst::utils::{application_root_dir, scene::BasicScenePrefab};

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Default)]
pub struct Alife {}

impl SimpleState for Alife {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let handle = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/plane.ron", RonFormat, ())
        });
        data.world.create_entity().with(handle).build();

        let handle = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/camera.ron", RonFormat, ())
        });
        data.world.create_entity().with(handle).build();
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}
