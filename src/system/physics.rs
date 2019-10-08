use crate::entity::Entity;
use crate::game_state::GameState;
use crate::sf;

/// Update all things related to physics
pub fn update_all(game_state: &mut GameState, delta: sf::Time) {
    update_kinematics(game_state, delta);
    // update_collisions(&mut game_state delta); TODO: implement
}

/// Handle position, velocity and acceleration update
fn update_kinematics(game_state: &mut GameState, delta: sf::Time) {
    for ent in &mut game_state.entities {
        // Update velocity
        ent.vel = sf::vectors::clamp_magnitude(
            sf::vectors::add(&ent.acc, &ent.vel),
            0.,
            Entity::PLAYER_MAX_VEL,
        );

        // Update position
        ent.state.translate(ent.vel.x, ent.vel.y);
    }
}

/// Check if any two entities are colliding and record it
fn update_collision_data(game_state: &mut GameState, delta: sf::Time) {
    // TODO
    unimplemented!();
}
