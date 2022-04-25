use super::helpers::component_store::ComponentStore;

#[derive(Copy, Clone)]
pub struct TagComponent {
    pub entity: usize,
    pub kind: KindTag,
    pub next_state: StateTag,
    pub actual_state: StateTag,
}
#[derive(Copy, Clone)]
pub struct StateTag(pub u128);
#[derive(Copy, Clone)]
pub struct KindTag(pub u128);

pub struct TagSystem {
    pub store: ComponentStore<TagComponent>,
}
impl TagSystem {
    pub fn init() -> TagSystem {
        TagSystem {
            store: ComponentStore::<TagComponent>::init(),
        }
    }
    pub fn update(&mut self) {
        for tag in self.store.data_mut() {
            tag.actual_state = tag.next_state;
            tag.next_state = StateTag(0);
        }
    }
}
