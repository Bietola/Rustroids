extern crate bit_set;

use crate::entity as ent;
use crate::sf;
use ent::flags;
use ent::Entity;

type EntityIndex = usize;

pub struct GameState {
    pub entities: Vec<Entity>,
}

impl GameState {
    /// Creates new empty world with player ship
    pub fn new() -> Self {
        // Create empty world
        let mut world = GameState { entities: vec![] };

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

    /// Filter only entities with given flags
    pub fn ents_with_flags<'a>(
        &'a self,
        flags: flags::Flags,
    ) -> impl Iterator<Item = EntityIndex> + 'a {
        self.entities
            .iter()
            .enumerate()
            .filter(move |(_, ent)| !(ent.flags & flags).is_empty())
            .map(|(idx, _)| idx)
    }

    /// Get a mutable reference to entity at a given entity index
    pub fn ent_at_mut(&mut self, idx: EntityIndex) -> &mut Entity {
        &mut self.entities[idx]
    }

    /// Get a reference to entity at a given entity index
    pub fn ent_at(&self, idx: EntityIndex) -> &Entity {
        &self.entities[idx]
    }
}

/// Draws all the entities contained in the game state
impl sf::Drawable for GameState {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut sf::RenderTarget,
        _states: sf::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        for ent in &self.entities {
            target.draw(&ent)
        }
    }
}
