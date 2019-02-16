use amethyst::assets::PrefabData;
use amethyst::controls::{HideCursor, WindowFocus};
use amethyst::core::{Time, Transform};
use amethyst::derive::PrefabData;
use amethyst::ecs::{
    Component, DenseVecStorage, Entity, Read, ReadStorage, Resources, System, WriteStorage,
};
use amethyst::error::Error;
use amethyst::input::InputHandler;
use amethyst::renderer::{DeviceEvent, Event};
use amethyst::shrev::{EventChannel, ReaderId};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// `Component` (and also `PrefabData`) for entities that are rotated by `MyFreeRotationSystem`
#[derive(Derivative, Clone, Debug, Component, PrefabData, Deserialize, Serialize)]
#[derivative(Default)]
#[storage(DenseVecStorage)]
#[prefab(Component)]
#[serde(default)]
pub struct MyFreeRotation {
    #[derivative(Default(value = "2.0"))]
    pub sensitivity_side: f32,

    #[derivative(Default(value = "2.0"))]
    pub sensitivity_updown: f32,

    #[derivative(Default(value = "2.0"))]
    pub roll_speed: f32,
}

/// System that pitches, yaws, and rolls locally
#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct MyFreeRotationSystem {
    event_reader: Option<ReaderId<Event>>,
}

impl<'a> System<'a> for MyFreeRotationSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'a, EventChannel<Event>>,
        Read<'a, InputHandler<String, String>>,
        Read<'a, WindowFocus>,
        Read<'a, HideCursor>,
        Read<'a, Time>,
        ReadStorage<'a, MyFreeRotation>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use amethyst::ecs::Join;

        let (
            event_channel,
            input_handler,
            window_focus,
            hide_cursor,
            time,
            free_rotations,
            mut transforms,
        ) = data;

        // iterator through the event queue
        let iter = event_channel.read(
            self.event_reader
                .as_mut()
                .expect("'MyFreeRotationSystem::setup' not run"),
        );

        // check if the window is focused, and the cursor is hidden
        // if not, then this system does nothing
        if !window_focus.is_focused || !hide_cursor.hide {
            // the event queue should be read anyways
            for _ in iter {}
            return;
        }

        // the roll value times the delta time if any, 0.0 if none
        let roll = input_handler
            .axis_value("roll_clock_counterclock")
            .map(|r| r as f32 * time.delta_seconds())
            .unwrap_or_default();

        // an array of delta values of mouse move events
        let mouse_moves: Vec<_> = iter
            .filter_map(|event| {
                if let Event::DeviceEvent { event, .. } = event {
                    if let DeviceEvent::MouseMotion { delta } = *event {
                        Some(delta)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // iterate through the storages only if any changes are required
        if !mouse_moves.is_empty() || roll.abs() > 0.0 {
            for (transform, free_rotation) in (&mut transforms, &free_rotations).join() {
                for (x, y) in mouse_moves.iter().cloned() {
                    transform.yaw_local((free_rotation.sensitivity_side * -x as f32).to_radians());
                    transform
                        .pitch_local((free_rotation.sensitivity_updown * -y as f32).to_radians());
                }

                transform.roll_local((free_rotation.roll_speed * -roll).to_radians());
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        use amethyst::ecs::SystemData;
        Self::SystemData::setup(res);
        self.event_reader = Some(res.fetch_mut::<EventChannel<Event>>().register_reader());
    }
}
