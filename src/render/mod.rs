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

pub fn render_world(ctx: &mut Context, world: &World) -> GameResult<()> {
    let play_state = world.read_resource::<PlayState>();
    match *play_state {
        PlayState::InWorld => {
            render_in_world(ctx, world)
        },
        PlayState::InBattle => {
            render_combat(ctx, world)
        }
        _ => {
            Ok(())
        }
    }
}
