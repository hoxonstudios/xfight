#[derive(Copy, Clone)]
pub struct ShapeComponent {
    pub position: (f32, f32),
    pub size: (u32, u32),
    pub flipped: (bool, bool),
    pub texture: ShapeTexture,
}

#[derive(Copy, Clone, Debug)]
pub struct ShapeTexture {
    pub texture_index: usize,
    pub position: (i32, i32),
    pub size: (u32, u32),
}
