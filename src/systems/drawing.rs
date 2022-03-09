use std::{path::Path, time::Duration};

use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use crate::components::shape::{ShapeComponent, ShapeTexture};

pub struct DrawingSystem<'a> {
    canvas: &'a mut Canvas<Window>,
    textures: Vec<Texture<'a>>,
}
impl<'a> DrawingSystem<'a> {
    pub fn init(canvas: &'a mut Canvas<Window>) -> Result<DrawingSystem<'a>, String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Ok(DrawingSystem {
            canvas,
            textures: vec![],
        })
    }
    pub fn load_textures(
        &mut self,
        creator: &'a TextureCreator<WindowContext>,
        paths: &[&'static str],
    ) -> Result<(), String> {
        for &path in paths {
            let texture = creator.load_texture(Path::new(path))?;
            self.textures.push(texture);
        }

        return Ok(());
    }
    pub fn run<'b>(
        &mut self,
        entities: impl Iterator<Item = &'b ShapeComponent>,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        for shape in entities {
            let (x, y) = shape.position;
            let (width, height) = shape.size;
            let (flip_horizontal, flip_vertical) = shape.flipped;
            let view_x = x as i32 - (width as i32 / 2);
            let view_y = y as i32 - (height as i32 / 2);

            match shape.texture {
                ShapeTexture {
                    texture_index,
                    position,
                    size,
                } => {
                    let texture = &self.textures[texture_index];

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

        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        Ok(())
    }
}
