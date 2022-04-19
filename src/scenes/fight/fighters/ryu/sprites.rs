use crate::systems::{damage::DamagePoint, drawing::Sprite, health::Shield};

pub const RYU_TEXTURE_PATH: &'static str = "assets/ryu.png";
// STAND
pub const RYU_STAND: [Sprite; 4] = [
    Sprite {
        center: (19, 42),
        area: (4, 14, 40, 92),
    },
    Sprite {
        center: (68, 42),
        area: (52, 15, 89, 92),
    },
    Sprite {
        center: (118, 43),
        area: (103, 14, 138, 93),
    },
    Sprite {
        center: (167, 42),
        area: (152, 13, 187, 92),
    },
];

// STUNT
pub const RYU_STUNT: Sprite = Sprite {
    center: (14, 779),
    area: (0, 756, 41, 829),
};

// CROUCH
pub const RYU_CROUCH: Sprite = Sprite {
    center: (1181, 44),
    area: (1162, 40, 1199, 94),
};

// WALKING
pub const RYU_WALKING: [Sprite; 6] = [
    Sprite {
        center: (221, 43),
        area: (207, 20, 237, 93),
    },
    Sprite {
        center: (268, 44),
        area: (249, 15, 288, 94),
    },
    Sprite {
        center: (317, 43),
        area: (302, 14, 341, 93),
    },
    Sprite {
        center: (367, 43),
        area: (353, 15, 387, 93),
    },
    Sprite {
        center: (416, 44),
        area: (402, 15, 435, 94),
    },
    Sprite {
        center: (468, 43),
        area: (454, 20, 485, 93),
    },
];

// LIGHT PUNCH
pub const RYU_LIGHT_PUNCH: [(Sprite, Option<DamagePoint>); 3] = [
    (
        Sprite {
            center: (18, 159),
            area: (2, 130, 41, 209),
        },
        None,
    ),
    (
        Sprite {
            center: (67, 159),
            area: (51, 130, 106, 209),
        },
        Some(DamagePoint {
            point: (35, -11),
            power: 5,
        }),
    ),
    (
        Sprite {
            center: (132, 159),
            area: (116, 130, 155, 209),
        },
        None,
    ),
];

// STRONG_PUNCH
pub const RYU_STRONG_PUNCH: [(Sprite, Option<DamagePoint>); 5] = [
    (
        Sprite {
            center: (185, 159),
            area: (169, 130, 208, 209),
        },
        None,
    ),
    (
        Sprite {
            center: (244, 158),
            area: (220, 125, 261, 208),
        },
        None,
    ),
    (
        Sprite {
            center: (300, 158),
            area: (279, 126, 343, 208),
        },
        Some(DamagePoint {
            point: (39, -13),
            power: 5,
        }),
    ),
    (
        Sprite {
            center: (379, 158),
            area: (355, 126, 401, 208),
        },
        None,
    ),
    (
        Sprite {
            center: (426, 159),
            area: (410, 130, 449, 209),
        },
        None,
    ),
];

// LIGHT KICK
pub const RYU_LIGHT_KICK: [(Sprite, Option<DamagePoint>); 3] = [
    (
        Sprite {
            center: (523, 290),
            area: (511, 258, 542, 340),
        },
        None,
    ),
    (
        Sprite {
            center: (585, 290),
            area: (569, 260, 619, 340),
        },
        Some(DamagePoint {
            point: (27, 23),
            power: 5,
        }),
    ),
    (
        Sprite {
            center: (652, 290),
            area: (641, 258, 672, 340),
        },
        None,
    ),
];

// STRONG KICK
pub const RYU_STRONG_KICK: [(Sprite, Option<DamagePoint>); 3] = [
    (
        Sprite {
            center: (23, 288),
            area: (1, 257, 42, 338),
        },
        None,
    ),
    (
        Sprite {
            center: (84, 288),
            area: (57, 255, 126, 338),
        },
        Some(DamagePoint {
            point: (36, -27),
            power: 5,
        }),
    ),
    (
        Sprite {
            center: (152, 288),
            area: (130, 257, 171, 338),
        },
        None,
    ),
];

// CROUCH LIGHT PUNCH
pub const RYU_CROUCH_LIGHT_PUNCH: [(Sprite, Option<DamagePoint>); 3] = [
    (
        Sprite {
            center: (30, 419),
            area: (11, 415, 53, 469),
        },
        None,
    ),
    (
        Sprite {
            center: (82, 420),
            area: (63, 415, 120, 470),
        },
        Some(DamagePoint {
            point: (34, 11),
            power: 5,
        }),
    ),
    (
        Sprite {
            center: (30, 419),
            area: (11, 415, 53, 469),
        },
        None,
    ),
];

// CROUCH STRONG PUNCH
pub const RYU_CROUCH_STRONG_PUNCH: [(Sprite, Option<DamagePoint>); 5] = [
    (
        Sprite {
            center: (148, 419),
            area: (129, 415, 171, 469),
        },
        None,
    ),
    (
        Sprite {
            center: (202, 419),
            area: (183, 415, 223, 469),
        },
        None,
    ),
    (
        Sprite {
            center: (254, 419),
            area: (235, 414, 290, 469),
        },
        Some(DamagePoint {
            point: (33, 9),
            power: 5,
        }),
    ),
    (
        Sprite {
            center: (202, 419),
            area: (183, 415, 223, 469),
        },
        None,
    ),
    (
        Sprite {
            center: (148, 419),
            area: (129, 415, 171, 469),
        },
        None,
    ),
];

// CROUCH LIGHT KICK
pub const RYU_CROUCH_LIGHT_KICK: [(Sprite, Option<DamagePoint>); 3] = [
    (
        Sprite {
            center: (682, 421),
            area: (663, 414, 706, 471),
        },
        None,
    ),
    (
        Sprite {
            center: (735, 422),
            area: (716, 413, 782, 472),
        },
        Some(DamagePoint {
            point: (40, 42),
            power: 5,
        }),
    ),
    (
        Sprite {
            center: (682, 421),
            area: (663, 414, 706, 471),
        },
        None,
    ),
];

// CROUCH STRONG KICK
pub const RYU_CROUCH_STRONG_KICK: [(Sprite, Option<DamagePoint>); 3] = [
    (
        Sprite {
            center: (867, 421),
            area: (849, 414, 892, 471),
        },
        None,
    ),
    (
        Sprite {
            center: (929, 419),
            area: (902, 422, 986, 469),
        },
        Some(DamagePoint {
            point: (51, 45),
            power: 5,
        }),
    ),
    (
        Sprite {
            center: (867, 421),
            area: (849, 414, 892, 471),
        },
        None,
    ),
];

// BLOCK
pub const RYU_BLOCK: (Sprite, Shield) = (
    Sprite {
        center: (1229, 44),
        area: (1210, 12, 1249, 94),
    },
    Shield {
        x0: -19,
        y0: -32,
        x1: 20,
        y1: 0,
        reduction: 0.75,
    },
);

// CROUCH BLOCK
pub const RYU_CROUCH_BLOCK: (Sprite, Shield) = (
    Sprite {
        center: (1278, 45),
        area: (1259, 34, 1296, 95),
    },
    Shield {
        x0: -19,
        y0: -11,
        x1: 18,
        y1: 50,
        reduction: 0.75,
    },
);
