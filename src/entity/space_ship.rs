use crate::sf;

// Spaceship
pub struct Spaceship {
    pub texture: sf::Texture,
    pub state: sf::Transform,
    pub vel: sf::Vector2f,
}

impl Spaceship {
    const SPRITE_SIZE: u32 = 100;

    pub fn new() -> Self {
        let texture = sf::Texture::from_file("assets/ship.png").expect("Could not load spaceship texture");
        let tsize = texture.size();

        Self {
            texture,
            state: sf::transform::scale(
                sf::Transform::IDENTITY,
                Self::SPRITE_SIZE as f32 / tsize.x as f32,
                Self::SPRITE_SIZE as f32 / tsize.y as f32
            ),
            vel: sf::Vector2::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        self.state.translate(self.vel.x, self.vel.y);
    }
}

impl sf::Drawable for Spaceship {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut sf::RenderTarget,
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
