use crate::entity::EntityIndex;
use crate::game_state::GameState;
use crate::sf;

/// Action that can be performed in a game
#[derive(Debug)]
pub enum Action {
    // Set acceleration angle in radiants
    SetThrustAngle(f32),

    // Set the magnitude of the thrusters
    SetThrustMagnitude(f32),
}

impl Action {
    /// Create an action from an sfml event
    pub fn from_event(e: &sf::Event) -> Vec<Action> {
        use sf::Event::*;
        match e {
            // Down presses are trated as a desire to change acceleration direction
            KeyPressed { code, .. } => match sf::utils::to_angle(*code) {
                None => vec![],
                Some(new_angle) => vec![Self::SetThrustAngle(new_angle)],
            },

            // Up presses represent stop acceleration
            KeyReleased { .. } => vec![Self::SetThrustMagnitude(0.)],

            _ => vec![],
        }
    }

    /// Make given player perform given action
    pub fn perform(&self, game_state: &mut GameState, ent_idx: EntityIndex) {
        use Action::*;
        use crate::sf::vec::Vec2;
        match self {
            // Set new velocity direction
            SetThrustAngle(new_acc_dir) => {
                let ent = game_state.ent_at_mut(ent_idx);

                ent.acc = Vec2(ent.acc).set_dir(Vec2::from_rads(new_acc_dir));
                debug!("player now accelerating at: {:?}", ent.acc);
            }
        }
    }
}
