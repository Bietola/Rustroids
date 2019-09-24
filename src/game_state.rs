use crate::entity::Entity;

use crate::sf;

type EntityIndex = usize;

pub struct GameState {
    pub entities: Vec<Entity>,
}

impl GameState {
    /// Creates new empty world with player ship
    pub fn new() -> Self {
        // Create empty world
        let mut world = GameState {
            entities: vec![],
        };

        // spawn player ship
        world.spawn(Entity::make_player_ship());

        // Return world with ship in it
        world
    }

    /// Spawns the given entity inside the world, returning its index
    pub fn spawn(&mut self, entity: Entity) -> EntityIndex {
        self.entities.push(entity);

        assert!(!self.entities.is_empty());

        self.entities.len() - 1
    }
}

/// Draws all the entities contained in the game state
impl sf::Drawable for GameState {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut sf::RenderTarget,
        states: sf::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        for ent in &self.entities {
            target.draw(&ent)
        }
    }
}
