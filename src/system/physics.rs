use crate::entity::Entity;
use crate::game_state::GameState;
use crate::sf;

/// Physics system constants
const FRICTION_AMOUNT: f32 = 10.;

/// Update all things related to physics
pub fn update_all(game_state: &mut GameState, delta: sf::Time) {
    update_kinematics(game_state, delta);
    // update_collisions(&mut game_state delta); TODO: implement
}

/// Handle position, velocity and acceleration update
fn update_kinematics(game_state: &mut GameState, delta: sf::Time) {
    for ent in &mut game_state.entities {
        // Update velocity
        use sf::vec::Vec2;
        ent.vel = *(Vec2(ent.vel) +
            // Entity acceleration
            &Vec2(ent.acc) * delta.as_milliseconds() as f32 +
            // Acceleration due to friction
            &Vec2(ent.acc).dir() * -FRICTION_AMOUNT)
            // Clamp velocity to maximium allowed for player
            .clamp_mag(0., Entity::PLAYER_MAX_VEL)
            // Convert to sfml vector
            .as_ref();

        // DB logging
        debug!(
            "delta_vel from acc {:?}",
            &Vec2(ent.acc) * delta.as_milliseconds() as f32
        );
        debug!("delta_vel from friction {:?}", &Vec2(ent.acc).dir());
        debug!("Updated vel: {:?}", ent.vel);
        debug!(
            "Updated vel mag: {:?}/{:?}",
            sf::vectors::magnitude(&ent.vel),
            Entity::PLAYER_MAX_VEL
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
