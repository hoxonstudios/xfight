use std::{collections::HashMap, path::Path, time::Duration};

use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use super::{helpers::ComponentStore, physics::PhysicsSystem};

#[derive(Copy, Clone)]
pub struct ShapeComponent {
    pub entity: usize,
    pub size: (u32, u32),
    pub flipped: (bool, bool),
    pub texture: ShapeTexture,
}

#[derive(Copy, Clone)]
pub struct ShapeTexture {
    pub texture_index: usize,
    pub sprite: TextureSprite,
}

#[derive(Copy, Clone)]
pub struct TextureSprite {
    pub position: (i32, i32),
    pub size: (u32, u32),
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

    pub fn update(&mut self, physics_system: &PhysicsSystem) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        for shape in self.store.data().iter() {
            if let Some(physics) = physics_system.store.get_component(shape.entity) {
                let (x, y) = physics.position;
                let (width, height) = shape.size;
                let (flip_horizontal, flip_vertical) = shape.flipped;
                let view_x = x as i32 - (width as i32 / 2);
                let view_y = y as i32 - (height as i32 / 2);

                match shape.texture {
                    ShapeTexture {
                        texture_index,
                        sprite: TextureSprite { position, size },
                    } => {
                        let texture = &self.texture_store.textures[texture_index];

                        self.canvas.copy_ex(
                            texture,
                            Rect::new(position.0, position.1, size.0, size.1),
                            Some(Rect::new(view_x, view_y, width, height)),
                            0.0,
                            None,
                            flip_horizontal,
                            flip_vertical,
                        )?;
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
