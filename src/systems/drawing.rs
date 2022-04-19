use std::{collections::HashMap, path::Path, time::Duration};

use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use super::{helpers::ComponentStore, position::PositionSystem};

const DEBUG_POSITION: bool = false;

#[derive(Copy, Clone)]
pub struct ShapeComponent {
    pub entity: usize,
    pub texture: usize,
    pub sprite: Sprite,
    pub flipped: (bool, bool),
}
#[derive(Copy, Clone)]
pub struct Sprite {
    pub center: (i32, i32),
    pub area: (i32, i32, i32, i32),
}

pub struct DrawingSystem<'a> {
    canvas: &'a mut Canvas<Window>,
    pub texture_store: TextureStore<'a>,
    pub store: ComponentStore<ShapeComponent>,
}
impl<'a> DrawingSystem<'a> {
    pub fn init(
        canvas: &'a mut Canvas<Window>,
        creator: &'a TextureCreator<WindowContext>,
    ) -> Result<DrawingSystem<'a>, String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Ok(DrawingSystem {
            canvas,
            texture_store: TextureStore {
                index_map: HashMap::new(),
                textures: vec![],
                creator,
            },
            store: ComponentStore::<ShapeComponent>::init(),
        })
    }

    pub fn update(&mut self, position_system: &PositionSystem) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        for shape in self.store.data() {
            let entity = shape.entity;
            if let Some(position) = position_system.store.get_component(entity) {
                match shape {
                    ShapeComponent {
                        sprite, flipped, ..
                    } => {
                        let texture = &self.texture_store.textures[shape.texture];
                        let (x1, y1, _, _) = sprite.rect(shape.flipped);
                        let (width, height) = sprite.size();
                        let x = sprite.area.0 as i32;
                        let y = sprite.area.1 as i32;
                        let view_x = position.x as i32 + x1;
                        let view_y = position.y as i32 + y1;

                        let src = Rect::new(x, y, width as u32, height as u32);
                        let dst = Some(Rect::new(view_x, view_y, width as u32, height as u32));

                        self.canvas
                            .copy_ex(texture, src, dst, 0.0, None, flipped.0, flipped.1)?;

                        if DEBUG_POSITION {
                            self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                            self.canvas.draw_rect(dst.unwrap())?;
                            self.canvas.draw_line(
                                Point::new(view_x, position.y as i32),
                                Point::new(view_x + width as i32, position.y as i32),
                            )?;
                            self.canvas.draw_line(
                                Point::new(position.x as i32, view_y),
                                Point::new(position.x as i32, view_y + height as i32),
                            )?;
                        }
                    }
                }
            }
        }

        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        Ok(())
    }
}

pub struct TextureStore<'a> {
    index_map: HashMap<&'static str, usize>,
    textures: Vec<Texture<'a>>,
    creator: &'a TextureCreator<WindowContext>,
}
impl<'a> TextureStore<'a> {
    pub fn load_texture(&mut self, path: &'static str) -> Result<usize, String> {
        if let Some(&index) = self.index_map.get(path) {
            return Ok(index);
        } else {
            let texture = self.creator.load_texture(Path::new(path))?;
            self.textures.push(texture);

            return Ok(self.textures.len() - 1);
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
