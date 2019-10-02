extern crate single;

use single::Single;

use crate::action::Action;
use crate::game_state::GameState;
use crate::entity::flags::Flags;

/// Update position with velocity
/// TODO: use acceleration to update velocity
pub fn update_physics(game_state: &mut GameState) {
    for ent in &mut game_state.entities {
        ent.state.translate(ent.vel.x, ent.vel.y);
    }
}

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
