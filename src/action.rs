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
}
