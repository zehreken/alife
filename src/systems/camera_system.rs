use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    renderer::camera::Camera,
};

pub struct RotateCameraSystem;

impl<'s> System<'s> for RotateCameraSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (cameras, mut locals, time): Self::SystemData) {
        for (camera, local) in (&cameras, &mut locals).join() {
            let value = time.absolute_time().as_secs() as f32;
            local.prepend_translation_x(value.cos() / 100.0);
            local.prepend_translation_y(value.sin() / 100.0);
        }
    }
}
