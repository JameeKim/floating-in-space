use amethyst::assets::PrefabData;
use amethyst::core::{Time, Transform};
use amethyst::derive::PrefabData;
use amethyst::ecs::{Component, DenseVecStorage, Entity, Read, ReadStorage, System, WriteStorage};
use amethyst::error::Error;
use amethyst::input::InputHandler;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// `Component` (and also `PrefabData`) for entities that are moved by `MyFlyMovementSystem`
#[derive(Derivative, Clone, Debug, Component, PrefabData, Deserialize, Serialize)]
#[derivative(Default)]
#[storage(DenseVecStorage)]
#[prefab(Component)]
#[serde(default)]
pub struct MyFlyMovement {
    #[derivative(Default(value = "5.0"))]
    pub speed: f32,
}

/// System that moves entities in accordance with control inputs
#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct MyFlyMovementSystem;

impl<'a> System<'a> for MyFlyMovementSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'a, InputHandler<String, String>>,
        Read<'a, Time>,
        ReadStorage<'a, MyFlyMovement>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (input_handler, time, fly_movements, mut transforms): Self::SystemData) {
        use amethyst::core::nalgebra::{Unit, Vector3};
        use amethyst::ecs::Join;

        // get the x, y, and z values of the movement direction
        let x = input_handler
            .axis_value("move_right_left")
            .unwrap_or_default() as f32;
        let y = input_handler.axis_value("move_up_down").unwrap_or_default() as f32;
        let z = -input_handler
            .axis_value("move_front_back")
            .unwrap_or_default() as f32;

        // check if the movement actually has a direction (that is, not zero)
        if let Some(move_dir) = Unit::try_new(Vector3::new(x, y, z), 1.0e-6) {
            let delta_sec = time.delta_seconds();

            // iterate through entities with `MyFlyMovement` component and apply the movement
            for (transform, fly_movement) in (&mut transforms, &fly_movements).join() {
                transform.move_along_local(move_dir, fly_movement.speed * delta_sec);
            }
        }
    }
}
