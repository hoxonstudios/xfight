use super::helpers::ComponentStore;

#[derive(Copy, Clone)]
pub struct ShapeComponent {
    pub entity: usize,
    pub action: ShapeAction,
    pub texture: usize,
    pub sprite: Sprite,
    pub flipped: (bool, bool),
}
#[derive(Copy, Clone)]
pub enum ShapeAction {
    None,
    Update {
        sprite: Sprite,
        flipped: (bool, bool),
    },
}
#[derive(Copy, Clone)]
pub struct Sprite {
    pub center: (i32, i32),
    pub area: (i32, i32, i32, i32),
}

pub struct ShapeSystem {
    pub store: ComponentStore<ShapeComponent>,
}
impl ShapeSystem {
    pub fn init() -> ShapeSystem {
        ShapeSystem {
            store: ComponentStore::<ShapeComponent>::init(),
        }
    }
    pub fn update(&mut self) {
        for shape in self.store.data_mut() {
            match shape.action {
                ShapeAction::None => {}
                ShapeAction::Update { sprite, flipped } => {
                    shape.sprite = sprite;
                    shape.flipped = flipped;
                }
            }
            shape.action = ShapeAction::None;
        }
    }
}

impl Sprite {
    pub fn rect(&self, flipped: (bool, bool)) -> (i32, i32, i32, i32) {
        let x1 = self.area.0 - self.center.0;
        let y1 = self.area.1 - self.center.1;
        let x2 = self.area.2 - self.center.0;
        let y2 = self.area.3 - self.center.1;

        match flipped {
            (false, false) => (x1, y1, x2, y2),
            (true, false) => (-x2, y1, -x1, y2),
            (false, true) => (x1, -y2, x2, -y1),
            (true, true) => (-x2, -y2, -x1, -y1),
        }
    }
    pub fn size(&self) -> (i32, i32) {
        (self.area.2 - self.area.0, self.area.3 - self.area.1)
    }
}
