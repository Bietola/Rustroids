pub mod flags;

use crate::entity::flags::Flags;
use crate::sf;

use crate::sf::vec::Vec2;

/// Index uniquely identifying an entity
pub type EntityIndex = usize;


/// Entity
#[derive(Debug)]
pub struct Entity {
    pub texture: sf::Texture,
    pub state: sf::Transform,
    pub vel: Vec2,
    pub acc: Vec2,
    pub flags: Flags,
}

impl Entity {
    /// TODO: do not rely on this
    pub const SPRITE_SIZE: u32 = 100;
    /// Player acceleration set by commands
    pub const PLAYER_ACC: f32 = 0.02;
    /// Maximum and minimum velocities reachable by player
    pub const PLAYER_MAX_VEL: f32 = 10.;
    pub const PLAYER_MIN_VEL: f32 = 0.5;

    /// Make player ship
    pub fn make_player_ship() -> Self {
        let texture =
            sf::Texture::from_file("assets/ship.png").expect("Could not load spaceship texture");
        let tsize = texture.size();

        Self {
            texture,
            state: sf::transform::scale(
                sf::Transform::IDENTITY,
                Self::SPRITE_SIZE as f32 / tsize.x as f32,
                Self::SPRITE_SIZE as f32 / tsize.y as f32,
            ),
            vel: Vec2::new(0.0, 0.0),
            acc: Vec2::new(0.0, 0.0),
            flags: Flags::PLAYER,
        }
    }

    /// Update physics
    pub fn update(&mut self) {
        self.state.translate(self.vel.x, self.vel.y);
    }
}

/// Draw entity
impl sf::Drawable for &Entity {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn sf::RenderTarget,
        states: sf::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_with_renderstates(
            &sf::Sprite::with_texture(&self.texture),
            sf::RenderStates {
                transform: sf::transform::combine(self.state, states.transform),

                ..states
            },
        );
    }
}
