mod debug;
mod world;
mod combat;

use state::PlayState;
use ggez::*;
use specs::*;
use ggez::graphics::*;
use self::world::*;
use self::debug::*;
use self::combat::*;

pub fn render_world(ctx: &mut Context, world: &World, play_state: &PlayState) -> GameResult<()> {
    match play_state {
        InWorld => {
            render_in_world(ctx, world)
        },
        _ => {
            Ok(())
        }
    }
}
