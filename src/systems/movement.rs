use super::{
    collision::CollisionSystem,
    helpers::ComponentStore,
    position::PositionSystem,
    shape::{ShapeAction, ShapeSystem},
};

#[derive(Copy, Clone)]
pub struct MovementComponent {
    pub entity: usize,
    pub action: Option<MovementAction>,
    pub direction: AimDirection,
    pub grounded: bool,
    pub attacking: bool,
    pub stunt: bool,
}
#[derive(Copy, Clone)]
pub enum MovementAction {
    WalkRight,
    WalkLeft,
    LightPunch,
    StrongPunch,
    LightKick,
    StrongKick,
    JumpStraight,
}
#[derive(Copy, Clone)]
pub enum AimDirection {
    Left,
    Right,
}

pub struct MovementSystem {
    pub store: ComponentStore<MovementComponent>,
}
impl MovementSystem {
    pub fn init() -> MovementSystem {
        MovementSystem {
            store: ComponentStore::<MovementComponent>::init(),
        }
    }
    pub fn update<'a>(
        &mut self,
        shape_system: &mut ShapeSystem,
        position_system: &PositionSystem,
        collision_system: &CollisionSystem,
    ) {
        let max_position = self.get_max_position(position_system);
        for movement in self.store.data_mut() {
            let entity = movement.entity;
            movement.grounded = MovementSystem::is_grounded(entity, collision_system);

            if let Some(shape) = shape_system.store.get_mut_component(entity) {
                if let Some(position) = position_system.store.get_component(entity) {
                    let (flipped_x, direction) = if position.x < max_position {
                        (false, AimDirection::Right)
                    } else {
                        (true, AimDirection::Left)
                    };
                    movement.direction = direction;
                    match shape.action {
                        ShapeAction::None => {
                            shape.action = ShapeAction::Update {
                                sprite: shape.sprite,
                                flipped: (flipped_x, shape.flipped.1),
                            };
                        }
                        ShapeAction::Update { sprite, flipped } => {
                            shape.action = ShapeAction::Update {
                                sprite,
                                flipped: (flipped_x, flipped.1),
                            };
                        }
                    }
                }
            }
        }
    }
    fn is_grounded(entity: usize, collision_system: &CollisionSystem) -> bool {
        collision_system
            .collisions
            .iter()
            .find(|&(c1, c2)| {
                (c1.entity == entity && c1.sides[3]) || (c2.entity == entity && c2.sides[3])
            })
            .is_some()
    }
    fn get_max_position(&self, position_system: &PositionSystem) -> f32 {
        self.store
            .data()
            .iter()
            .map(|m| {
                if let Some(position) = position_system.store.get_component(m.entity) {
                    position.x
                } else {
                    0.0
                }
            })
            .reduce(f32::max)
            .unwrap_or_default()
    }
}
