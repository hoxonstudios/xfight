use std::{collections::HashMap, path::Path, time::Duration};

use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use super::physics::{PhysicsSystem, Shape};

const DEBUG_POSITION: bool = true;

pub struct DrawingSystem<'a> {
    canvas: &'a mut Canvas<Window>,
    pub texture_store: TextureStore<'a>,
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
        })
    }

    pub fn update(&mut self, physics_system: &PhysicsSystem) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        for physics in physics_system.store.data() {
            let shape = physics.shape;
            let (pos_x, pos_y) = physics.position;

            match shape {
                Shape {
                    texture_index,
                    sprite,
                    flipped,
                } => {
                    let texture = &self.texture_store.textures[texture_index];
                    let (width, height) = sprite.size();
                    let x = sprite.area.0 as i32;
                    let y = sprite.area.1 as i32;

                    let (view_x, view_y, _, _) = physics.absolute_position();
                    let src = Rect::new(x, y, width as u32, height as u32);
                    let dst = Some(Rect::new(view_x, view_y, width as u32, height as u32));

                    self.canvas
                        .copy_ex(texture, src, dst, 0.0, None, flipped.0, flipped.1)?;

                    if DEBUG_POSITION {
                        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                        self.canvas.draw_rect(dst.unwrap())?;
                        self.canvas.draw_line(
                            Point::new(view_x, pos_y as i32),
                            Point::new(view_x + width as i32, pos_y as i32),
                        )?;
                        self.canvas.draw_line(
                            Point::new(pos_x as i32, view_y),
                            Point::new(pos_x as i32, view_y + height as i32),
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
