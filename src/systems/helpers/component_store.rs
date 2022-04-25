pub struct ComponentStore<T: Copy> {
    entities: Vec<Option<usize>>,
    components: Vec<T>,
}

impl<T: Copy> ComponentStore<T> {
    pub fn init() -> ComponentStore<T> {
        ComponentStore {
            entities: vec![],
            components: vec![],
        }
    }
    pub fn data(&self) -> &Vec<T> {
        &self.components
    }
    pub fn data_mut(&mut self) -> &mut Vec<T> {
        &mut self.components
    }
    pub fn get_component(&self, entity: usize) -> Option<T> {
        if entity < self.entities.len() {
            if let Some(index) = self.entities[entity] {
                Some(self.components[index])
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn get_component_ref(&self, entity: usize) -> Option<&T> {
        if entity < self.entities.len() {
            if let Some(index) = self.entities[entity] {
                Some(&self.components[index])
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn get_mut_component(&mut self, entity: usize) -> Option<&mut T> {
        if entity < self.entities.len() {
            if let Some(index) = self.entities[entity] {
                Some(&mut self.components[index])
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn insert_component(&mut self, entity: usize, component: T) {
        let last = self.entities.len();
        if entity >= self.entities.len() {
            for _ in last..entity {
                self.entities.push(Option::None);
            }
            self.entities.push(Some(self.components.len()));
        } else {
            self.entities[entity] = Some(self.components.len());
        }
        self.components.push(component);
    }
    pub fn delete_component(&mut self, entity: usize, get_entity: fn(&T) -> usize) {
        if entity < self.entities.len() {
            if let Some(index) = self.entities[entity] {
                self.components[index] = self.components[self.components.len() - 1];
                let move_entity = get_entity(&self.components[index]);
                self.entities[move_entity] = Some(index);
                self.entities[entity] = None;
                self.components.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone)]
    struct TestComponent {
        pub entity: usize,
    }

    #[test]
    fn it_should_init_store_empty() {
        // ACT
        let store = ComponentStore::<TestComponent>::init();
        // ASSERT
        assert_eq!(store.entities.len(), 0);
        assert_eq!(store.components.len(), 0);
    }
    #[test]
    fn store_should_increase_spare_set_on_component_insertion_with_higher_entity_id() {
        // ARRANGE
        let entity: usize = 3;
        let mut store = ComponentStore::<TestComponent> {
            entities: vec![],
            components: vec![],
        };
        // ACT
        store.insert_component(entity, TestComponent { entity });
        // ASSERT
        assert_eq!(store.entities.len(), 4);
        assert_eq!(store.entities[entity], Some(0));
        assert_eq!(store.entities[0], None);
        assert_eq!(store.components.len(), 1);
        assert_eq!(store.components[0].entity, 3);
    }

    #[test]
    fn store_should_replace_spare_set_entity_if_exists_on_component_insertion() {
        // ARRANGE
        let entity: usize = 2;
        let mut store = ComponentStore::<TestComponent> {
            entities: vec![None, None, None, Some(0)],
            components: vec![TestComponent { entity: 3 }],
        };
        // ACT
        store.insert_component(entity, TestComponent { entity });
        // ASSERT
        assert_eq!(store.entities.len(), 4);
        assert_eq!(store.entities[entity], Some(1));
        assert_eq!(store.components.len(), 2);
        assert_eq!(store.components[1].entity, 2);
    }

    #[test]
    fn store_should_replace_component_with_the_last_one_on_deletion_in_the_middle_of_spare_set() {
        // ARRANGE
        let entity: usize = 1;
        let mut store = ComponentStore::<TestComponent> {
            entities: vec![None, Some(0), Some(1), Some(2)],
            components: vec![
                TestComponent { entity },
                TestComponent { entity: 2 },
                TestComponent { entity: 3 },
            ],
        };
        // ACT
        store.delete_component(entity, |e| e.entity);
        // ASSERT
        assert_eq!(store.entities.len(), 4);
        assert_eq!(store.entities[entity], None);
        assert_eq!(store.components.len(), 2);
        assert_eq!(store.components[0].entity, 3);
    }

    #[test]
    fn store_should_delete_the_entity_from_spare_set_if_is_the_last_one() {
        // ARRANGE
        let entity: usize = 3;
        let mut store = ComponentStore::<TestComponent> {
            entities: vec![None, Some(0), Some(1), Some(2)],
            components: vec![
                TestComponent { entity: 1 },
                TestComponent { entity: 2 },
                TestComponent { entity },
            ],
        };
        // ACT
        store.delete_component(entity, |e| e.entity);
        // ASSERT
        assert_eq!(store.entities.len(), 3);
        assert_eq!(store.components.len(), 2);
        assert_eq!(store.components[0].entity, 1);
    }
    #[test]
    fn store_should_return_existing_component_by_entity() {
        // ARRANGE
        let entity: usize = 3;
        let store = ComponentStore::<TestComponent> {
            entities: vec![None, Some(1), Some(2), Some(0)],
            components: vec![
                TestComponent { entity },
                TestComponent { entity: 1 },
                TestComponent { entity: 2 },
            ],
        };
        // ACT
        let component = store.get_component_ref(entity);
        // ASSERT
        assert!(matches!(component, Some(TestComponent { entity: 3 })));
    }
    #[test]
    fn store_should_return_none_if_component_does_not_exist() {
        // ARRANGE
        let entity: usize = 0;
        let store = ComponentStore::<TestComponent> {
            entities: vec![None, Some(1), Some(2), Some(0)],
            components: vec![
                TestComponent { entity: 3 },
                TestComponent { entity: 1 },
                TestComponent { entity: 2 },
            ],
        };
        // ACT
        let component = store.get_component_ref(entity);
        // ASSERT
        assert!(matches!(component, None));
    }
    #[test]
    fn store_should_return_mut_existing_component_by_entity() {
        // ARRANGE
        let entity: usize = 3;
        let mut store = ComponentStore::<TestComponent> {
            entities: vec![None, Some(1), Some(2), Some(0)],
            components: vec![
                TestComponent { entity },
                TestComponent { entity: 1 },
                TestComponent { entity: 2 },
            ],
        };
        // ACT
        let component: Option<&mut TestComponent> = store.get_mut_component(entity);
        // ASSERT
        assert!(matches!(component, Some(TestComponent { entity: 3 })));
    }
    #[test]
    fn store_should_not_return_mut_component_if_it_does_not_exist() {
        // ARRANGE
        let entity: usize = 0;
        let mut store = ComponentStore::<TestComponent> {
            entities: vec![None, Some(1), Some(2), Some(0)],
            components: vec![
                TestComponent { entity: 3 },
                TestComponent { entity: 1 },
                TestComponent { entity: 2 },
            ],
        };
        // ACT
        let component = store.get_mut_component(entity);
        // ASSERT
        assert!(matches!(component, None));
    }
}
