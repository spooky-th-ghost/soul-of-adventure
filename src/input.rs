use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, *};
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Default, Reflect)]
pub enum PlayerAction {
    #[default]
    Jump,
    Move,
}

#[derive(Bundle)]
pub struct InputListenerBundle {
    input_manager: InputManagerBundle<PlayerAction>,
}

impl InputListenerBundle {
    pub fn input_map() -> InputListenerBundle {
        use PlayerAction::*;

        let input_map = input_map::InputMap::new([(KeyCode::Space, Jump)])
            .insert(DualAxis::left_stick(), Move)
            .insert(VirtualDPad::wasd(), Move)
            .set_gamepad(Gamepad { id: 0 })
            .build();

        InputListenerBundle {
            input_manager: InputManagerBundle {
                input_map,
                ..Default::default()
            },
        }
    }
}
