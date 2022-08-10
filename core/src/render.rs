use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Render {
    pub render_type: RenderType,
}

pub trait RenderAsTank {
    fn as_tank() -> Render;
}

impl RenderAsTank for Render {
    fn as_tank() -> Render {
        Render {
            render_type: RenderType::Tank,
        }
    }
}
#[derive(Debug)]
pub enum RenderType {
    Tank = 0,
    Bullet,
}
