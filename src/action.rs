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

    pub fn perform(&self, game_state: &mut GameState, ent_idx: EntityIndex) {
        use Action::*;
        match self {
            SetVelDir(new_vel_dir) => {
                let ent = game_state.ent_at_mut(ent_idx);

                use sf::vectors as v;
                println!("old vel: {:?}", ent.vel); // DB
                let speed: f32 = 5.; // TODO: let speed: f32 = v::magnitude(&ent.vel);
                ent.vel = v::scalar_mul(speed, &new_vel_dir);
                println!("setting speed to {:?} [{:?}]", ent.vel, speed); // DB
            }
        }
    }
}
