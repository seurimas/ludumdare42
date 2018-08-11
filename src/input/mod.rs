mod world;
mod combat;

use state::PlayState;
use ggez::*;
use specs::*;
pub use self::world::HandleMove;
pub use self::combat::HandleBattleMenu;
use self::combat::select_target;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn handle_select(ctx: &mut Context, world: &mut World) {
    let play_state = world.read_resource::<PlayState>().clone();
    match play_state {
        PlayState::InBattle => {
            select_target(ctx, world);
        },
        _ => {

        }
    }
}
