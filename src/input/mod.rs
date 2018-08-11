mod world;
mod combat;

use state::PlayState;
use ggez::*;
use specs::*;
use self::world::move_player;
use self::combat::move_cursor;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn handle_arrow(ctx: &mut Context, world: &mut World, play_state: &PlayState, direction: Direction) {
    match play_state {
        PlayState::InWorld => {
            move_player(ctx, world, &direction);
        },
        PlayState::InBattle => {
            move_cursor(ctx, world, &direction);
        }
        _ => {

        }
    }
}
