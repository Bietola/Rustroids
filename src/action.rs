use crate::entity::Entity;
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
        use Action::*;
        match e {
            // Down presses are trated as a desire to change acceleration direction
            // TODO: research Either-List monad conversions
            KeyPressed { code, .. } => match sf::utils::to_angle(*code) {
                None => vec![],
                Some(new_angle) => vec![
                    SetThrustMagnitude(Entity::PLAYER_ACC),
                    SetThrustAngle(new_angle),
                ],
            },

            // Up presses represent stop acceleration
            KeyReleased { .. } => vec![SetThrustMagnitude(0.)],

            _ => vec![],
        }
    }

    /// Make given player perform given action
    pub fn perform(&self, game_state: &mut GameState, ent_idx: EntityIndex) {
        use Action::*;
        match self {
            // Set new thrust direction
            SetThrustAngle(new_acc_angle) => {
                let ent = game_state.ent_at_mut(ent_idx);

                ent.acc = ent.acc.change_dir_angle(*new_acc_angle);
                debug!("player now accelerating at: {:?}", ent.acc);
            }

            // Set new thurst amount
            SetThrustMagnitude(new_acc_amm) => {
                let ent = game_state.ent_at_mut(ent_idx);

                ent.acc = ent.acc.change_mag(*new_acc_amm);
            }
        }
    }
}
