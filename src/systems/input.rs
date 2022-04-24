use sdl2::{keyboard::Keycode, EventPump};

use super::{
    helpers::component_store::ComponentStore,
    movement::{system::MovementSystem, MovementAction},
};

#[derive(Copy, Clone)]
pub struct InputComponent {
    pub entity: usize,
    pub controller: Controller,
}

#[derive(Copy, Clone)]
pub struct Controller {
    pub keys: &'static [ControllerKey],
}
#[derive(Copy, Clone)]
pub struct ControllerKey {
    pub code: Keycode,
    pub action: MovementAction,
}

pub struct InputSystem<'a> {
    pub event_pump: &'a mut EventPump,
    pub store: ComponentStore<InputComponent>,
}
impl<'a> InputSystem<'a> {
    pub fn init(event_pump: &'a mut EventPump) -> InputSystem {
        InputSystem {
            event_pump,
            store: ComponentStore::<InputComponent>::init(),
        }
    }
    pub fn update(&self, movement_system: &mut MovementSystem) {
        let pressed_keys = self.get_pressed_keys();

        for input in self.store.data() {
            let entity = input.entity;

            if let Some(movement) = movement_system.store.get_mut_component(entity) {
                let mut action = match movement.action {
                    None => 0,
                    Some(action) => action.0,
                };

                for key in input.controller.keys {
                    if pressed_keys.contains(&key.code) {
                        action |= key.action.0;
                    }
                }

                let none_action = action == 0;
                movement.action = if none_action {
                    None
                } else {
                    Some(MovementAction(action))
                };
            }
        }
    }

    fn get_pressed_keys(&self) -> Vec<Keycode> {
        self.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect()
    }
}
