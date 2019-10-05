use crate::entity::EntityIndex;
use crate::game_state::GameState;
use crate::sf;

/// Action that can be performed in a game
pub enum Action {
    SetVelDir(sf::Vector2f),
}

impl Action {
    /// Create an action from an sfml event
    pub fn from_event(e: &sf::Event) -> Option<Self> {
        if let Some(new_dir) = sf::utils::to_dir(e) {
            // Move action
            Some(Action::SetVelDir(new_dir))
        } else {
            // WIP
            None
        }
    }

    /// Make given player perform given action
    pub fn perform(&self, game_state: &mut GameState, ent_idx: EntityIndex) {
        use Action::*;
        match self {
            // Set new velocity direction
            SetVelDir(new_vel_dir) => {
                let ent = game_state.ent_at_mut(ent_idx);

                // Scalar is used to maintain constant speed (changing only velocity direction)
                use sf::vectors as v;
                let speed: f32 = 5.; // TODO: let speed: f32 = v::magnitude(&ent.vel);
                ent.vel = v::scalar_mul(speed, &new_vel_dir);
            }
        }
    }
}
