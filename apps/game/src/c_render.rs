use bevy::prelude::*;

#[derive(Component)]
pub struct Render {
    pub render_type: RenderType,
}

impl Render {
    pub fn as_tank() -> Render {
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
