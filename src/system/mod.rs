pub mod physics;

extern crate single;

use single::Single;

use crate::action::Action;
use crate::game_state::GameState;
use crate::entity::flags::Flags;


/// Subject entity to given game action
pub fn handle_player_action(game_state: &mut GameState, action: &Action) {
    action.perform(
        game_state,
        game_state
            .ents_with_flags(Flags::PLAYER)
            .single()
            .unwrap()
    );
}
