use crate::entity::Entity;
use crate::entity::EntityIndex;
use crate::entity::all_entities::*;

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
        world.spawn(Entity::Spaceship(Spaceship::new()));

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
