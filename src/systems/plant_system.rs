use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

pub struct Living {
    pub age: u32,
}

impl Component for Living {
    type Storage = DenseVecStorage<Self>;
}

pub struct Plant;

impl Component for Plant {
    type Storage = DenseVecStorage<Self>;
}

pub struct PlantSystem;
impl<'s> System<'s> for PlantSystem {
    type SystemData = (
        ReadStorage<'s, Plant>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (plants, mut transforms, time): Self::SystemData) {
        for (plant, transform) in (&plants, &mut transforms).join() {
            let value = time.absolute_time().as_secs() as f32;
            transform.prepend_rotation_z_axis(2.0 * time.delta_seconds());
        }
    }
}
