mod movement;
mod rotation;

pub use self::movement::MyFlyMovement;
pub use self::rotation::MyFreeRotation;

use amethyst::core::SystemBundle;
use amethyst::error::Error;
use amethyst::shred::DispatcherBuilder;
use derivative::Derivative;

/// SystemBundle for controlling the player (rotation and movement)
#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct MyControlBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MyControlBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        use self::movement::MyFlyMovementSystem;
        use self::rotation::MyFreeRotationSystem;
        use amethyst::controls::{CursorHideSystem, MouseFocusUpdateSystem};

        // Both systems read from the `InputHandler` resource, so the `InputSystem` should first
        // handle the inputs and write the result to the resource before these systems run.

        builder.add(MouseFocusUpdateSystem::new(), "mouse_focus", &[]);
        builder.add(CursorHideSystem::new(), "cursor_hide", &["mouse_focus"]);
        builder.add(
            MyFreeRotationSystem::new(),
            "my_control_rotation",
            &["input_system", "cursor_hide"],
        );
        builder.add(
            MyFlyMovementSystem::new(),
            "my_control_movement",
            &["input_system", "cursor_hide"],
        );

        Ok(())
    }
}
