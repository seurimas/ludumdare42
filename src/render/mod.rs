mod debug;
mod world;
mod combat;
mod inventory;
mod text;
mod looting;
mod sprites;

use state::PlayState;
use ggez::*;
use specs::*;
use ggez::graphics::*;
use self::text::*;
use self::world::*;
use self::debug::*;
use self::combat::*;
use self::inventory::*;
use self::looting::*;
use self::sprites::*;

pub fn render_world(ctx: &mut Context, world: &mut World) -> GameResult<()> {
    let play_state = world.read_resource::<PlayState>().clone();
    println!("{:?}", play_state);
    match play_state {
        PlayState::InWorld => {
            render_in_world(ctx, world)
        },
        PlayState::InBattle => {
            render_combat(ctx, world)
        },
        PlayState::Combining => {
            render_combining(ctx, world)
        },
        PlayState::Looting { captured, lost } => {
            render_looting(ctx, world, &captured)
        },
        _ => {
            Ok(())
        }
    }
}
