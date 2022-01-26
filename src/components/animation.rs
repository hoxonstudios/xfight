use super::shape::ShapeTexture;

#[derive(Copy, Clone)]
pub struct AnimationComponent {
    pub state: usize,
    pub frame: usize,
    pub sprite: usize,
    pub infinite: bool,
    pub transitions: &'static [AnimationTransition],
}

#[derive(Copy, Clone)]
pub struct AnimationTransition {
    pub sprites: &'static [AnimationSprite],
}

#[derive(Copy, Clone)]
pub struct AnimationSprite {
    pub texture: ShapeTexture,
    pub duration: u32,
}
