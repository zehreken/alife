// use amethyst::renderer::shape::Shape;
// use amethyst::renderer::Mesh;
// use amethyst::{
//     core::timing::Time,
//     core::transform::Transform,
//     ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
// };

pub struct RotateObjectSystem;

// impl<'s> System<'s> for RotateObjectSystem {
//     type SystemData = (
//         ReadStorage<'s, Mesh>,
//         WriteStorage<'s, Transform>,
//         Read<'s, Time>,
//     );

//     fn run(&mut self, (cameras, mut locals, time): Self::SystemData) {
//         for (camera, local) in (&cameras, &mut locals).join() {
//             local.prepend_rotation_x_axis(10.0 * time.delta_seconds());
//         }
//     }
// }
