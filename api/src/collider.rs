use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Collider {
    pub shape: Shape,
    pub category_bit_mask: CollisionMask,
    pub collision_bit_mask: CollisionMask,
}

pub enum CollisionMask {
    None,
    Tank = 0x0001,
    Wall = 0x0010,
    Bullet = 0x0100,
    All = 0x1111,
}

pub enum Shape {
    Circle { radius: u32 },
    Rect { width: u32, height: u32 },
}

trait MaxDiameter {
    fn get_max_diameter(&self) -> u32;
}

impl MaxDiameter for Shape {
    fn get_max_diameter(&self) -> u32 {
        match self {
            Shape::Circle { radius } => radius * 2,
            Shape::Rect { width, height } => {
                f32::sqrt((u32::pow(*width, 2) + u32::pow(*height, 2)) as f32) as u32
            }
        }
    }
}

pub trait TankCollider {
    fn tank() -> Collider;
}

impl TankCollider for Collider {
    fn tank() -> Collider {
        Collider {
            shape: Shape::Rect {
                width: 5,
                height: 5,
            },
            category_bit_mask: CollisionMask::Tank,
            collision_bit_mask: CollisionMask::All,
        }
    }
}
