use crate::entity::Entity;
use crate::game_state::GameState;
use crate::sf;

/// Physics system constants
const FRICTION_AMOUNT: f32 = 0.95;

/// Update all things related to physics
pub fn update_all(game_state: &mut GameState, delta: sf::Time) {
    update_kinematics(game_state, delta);
    // update_collisions(&mut game_state delta); TODO: implement
}

/// Handle position, velocity and acceleration update
fn update_kinematics(game_state: &mut GameState, delta: sf::Time) {
    for ent in &mut game_state.entities {
        // Update velocity
        ent.vel = ent
            .vel
            // Entity acceleration
            .vec_add(ent.acc * delta.as_milliseconds() as f32)
            // Apply friction
            .scalar_mul(FRICTION_AMOUNT)
            // Clamp velocity to maximium allowed for player
            .clamp_on(|vec| vec.x.abs(), 0., Entity::PLAYER_MAX_VEL)
            .clamp_on(|vec| vec.y.abs(), 0., Entity::PLAYER_MAX_VEL)
            // Convert to sfml vector
            .into();

        // DB logging
        debug!(
            "delta_vel from acc {:?}",
            ent.acc * delta.as_milliseconds() as f32
        );
        debug!("delta_vel from friction {:?}", ent.acc.dir());
        debug!("Updated vel: {:?}", ent.vel);
        debug!(
            "Updated vel mag: {:?}/{:?}",
            ent.vel.mag(),
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
