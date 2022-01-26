use crate::components::{animation::AnimationComponent, shape::ShapeComponent};

pub struct AnimationSystem;
impl AnimationSystem {
    pub fn run<'a>(
        entities: impl Iterator<Item = (&'a mut ShapeComponent, &'a mut AnimationComponent)>,
    ) {
        for (shape, animation) in entities {
            let actual = animation.transitions[animation.state];
            let sprite = actual.sprites[animation.sprite];

            let has_more_frames = animation.frame < sprite.duration as usize;
            let has_more_sprites = animation.sprite < actual.sprites.len() - 1;

            if has_more_frames {
                shape.texture = sprite.texture;
                animation.frame += 1;
            } else if has_more_sprites {
                animation.frame = 0;
                animation.sprite += 1;
                let next_sprite = actual.sprites[animation.sprite];
                shape.texture = next_sprite.texture;
            } else if animation.infinite {
                animation.frame = 0;
                animation.sprite = 0;
                let next_sprite = actual.sprites[animation.sprite];
                shape.texture = next_sprite.texture;
            }
        }
    }
}
