use crate::systems::physics::TextureSprite;

pub const RYU_TEXTURE_PATH: &'static str = "assets/ryu.png";
// STAND
pub const RYU_STAND_1: TextureSprite = TextureSprite {
    center: (19, 42),
    area: (0, 14, 40, 92),
};
pub const RYU_STAND_2: TextureSprite = TextureSprite {
    center: (68, 42),
    area: (52, 15, 89, 92),
};
pub const RYU_STAND_3: TextureSprite = TextureSprite {
    center: (118, 43),
    area: (103, 14, 138, 93),
};
pub const RYU_STAND_4: TextureSprite = TextureSprite {
    center: (167, 42),
    area: (152, 13, 187, 92),
};

// WALKING
pub const RYU_WALKING_1: TextureSprite = TextureSprite {
    center: (221, 43),
    area: (207, 20, 237, 93),
};
pub const RYU_WALKING_2: TextureSprite = TextureSprite {
    center: (268, 44),
    area: (249, 15, 288, 94),
};
pub const RYU_WALKING_3: TextureSprite = TextureSprite {
    center: (317, 43),
    area: (302, 14, 341, 93),
};
pub const RYU_WALKING_4: TextureSprite = TextureSprite {
    center: (367, 43),
    area: (353, 15, 387, 93),
};
pub const RYU_WALKING_5: TextureSprite = TextureSprite {
    center: (416, 44),
    area: (402, 15, 435, 94),
};
pub const RYU_WALKING_6: TextureSprite = TextureSprite {
    center: (468, 43),
    area: (454, 20, 485, 93),
};

// PUNCH
pub const RYU_LIGHT_PUNCH_1: TextureSprite = TextureSprite {
    center: (18, 159),
    area: (2, 130, 41, 209),
};
pub const RYU_LIGHT_PUNCH_2: TextureSprite = TextureSprite {
    center: (67, 159),
    area: (51, 130, 106, 209),
};
pub const RYU_LIGHT_PUNCH_3: TextureSprite = TextureSprite {
    center: (132, 159),
    area: (116, 130, 155, 209),
};
