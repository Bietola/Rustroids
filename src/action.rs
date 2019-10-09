use crate::entity::EntityIndex;
use crate::game_state::GameState;
use crate::sf;
use crate::entity::Entity;

/// Action that can be performed in a game
pub enum Action {
    SetAccDir(sf::Vector2f),
}

impl Action {
    /// Create an action from an sfml event
    pub fn from_event(e: &sf::Event) -> Option<Action> {
        use sf::Event::*;
        match e {
            // Down presses are trated as a desire to changea acceleration direction
            KeyPressed { code, .. } => sf::utils::to_dir(*code).map(Self::SetAccDir),

            // Up presses represent stop acceleration
            KeyReleased { .. } => Some(Self::SetAccDir(sf::Vector2f::from((0., 0.)))),

            _ => None,
        }
    }

    /// Make given player perform given action
    pub fn perform(&self, game_state: &mut GameState, ent_idx: EntityIndex) {
        use Action::*;
        match self {
            // Set new velocity direction
            SetAccDir(new_acc_dir) => {
                let ent = game_state.ent_at_mut(ent_idx);

                ent.acc = sf::vectors::scalar_mul(Entity::PLAYER_ACC, new_acc_dir);
                debug!("player now accelerating at: {:?}", ent.acc);
            }
        }
    }
}
