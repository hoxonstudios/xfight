use super::{
    collision::CollisionSystem,
    helpers::ComponentStore,
    movement::{MovementAction, MovementSystem},
};

#[derive(Copy, Clone)]
pub struct GroundComponent {
    pub entity: usize,
    pub grounded: bool,
}

pub struct GroundSystem {
    pub store: ComponentStore<GroundComponent>,
}
impl GroundSystem {
    pub fn init() -> GroundSystem {
        GroundSystem {
            store: ComponentStore::<GroundComponent>::init(),
        }
    }
    pub fn update(
        &mut self,
        movement_system: &mut MovementSystem,
        collision_system: &CollisionSystem,
    ) {
        for ground in self.store.data_mut() {
            let entity = ground.entity;
            if let Some(movement) = movement_system.store.get_mut_component(entity) {
                if let Some(MovementAction::JumpStraight) = movement.action {
                    ground.grounded = false;
                } else if !ground.grounded {
                    if GroundSystem::is_grounded(entity, collision_system) {
                        movement.action = Some(MovementAction::Land);
                        ground.grounded = true;
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
}
