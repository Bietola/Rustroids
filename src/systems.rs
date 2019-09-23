use crate::game_state::GameState;
use crate::entity::Entity;

pub fn update_physics(game_state: &mut GameState) {
    for mut entity in game_state.entities {
        if let Entity::Spaceship(ship) = entity {
            ship.vel += ship.acc;
        } else {
            panic!("WIP!");
        }
    }
}
