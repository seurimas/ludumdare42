mod debug;
mod world;
mod combat;
mod inventory;
mod text;
mod looting;
mod sprites;

use state::*;
use ggez::*;
use specs::*;
use ggez::graphics::*;
pub use self::text::*;
use self::world::*;
use self::debug::*;
use self::combat::*;
use self::inventory::*;
use self::looting::*;
use self::sprites::*;

const START_BUTTON: (i32, i32, i32, i32) = (
    SCREEN_SIZE.0 as i32 / 2 - 64, SCREEN_SIZE.1 as i32 / 2 - 128,
    128, 64,
);
const INFO_BUTTON: (i32, i32, i32, i32) = (
    SCREEN_SIZE.0 as i32 / 2 - 64, SCREEN_SIZE.1 as i32 / 2 + 64,
    128, 64,
);
const GAME_OVER: (i32, i32, i32, i32) = (
    SCREEN_SIZE.0 as i32 / 2 - 32, SCREEN_SIZE.1 as i32 / 2 - 16,
    128, 32,
);
const INFO_AREA: (i32, i32, i32, i32) = (
    0, SCREEN_SIZE.1 as i32 / 2,
    SCREEN_SIZE.0 as i32, SCREEN_SIZE.1 as i32 / 2,
);

pub fn render_world(ctx: &mut Context, world: &mut World) -> GameResult<()> {
    let play_state = world.read_resource::<PlayState>().clone();
    // println!("{:?}", play_state);
    match play_state {
        PlayState::InWorld => {
            render_in_world(ctx, world)
        },
        PlayState::InBattle => {
            let battle_state = world.read_resource::<BattleState>().clone();
            if battle_state.retreating {
                render_inventory(ctx, world, true)
            } else {
                render_combat(ctx, world)
            }
        },
        PlayState::Combining => {
            render_inventory(ctx, world, false)
        },
        PlayState::Looting { captured, lost } => {
            render_looting(ctx, world, &captured, &lost)
        },
        PlayState::MainMenu(val) => {
            text_outline_color(
                ctx,
                START_BUTTON,
                match val {
                    0 => [0.0, 1.0, 1.0, 1.0].into(),
                    _ => [1.0, 1.0, 1.0, 1.0].into(),
                },
            )?;
            text_in_box(
                ctx,
                &format!("Enter Semb"),
                (START_BUTTON.0 + 8, START_BUTTON.1 + 8, START_BUTTON.2 - 16),
            )?;
            text_outline_color(
                ctx,
                INFO_AREA,
                [0.0, 1.0, 1.0, 1.0].into(),
            )?;
            text_in_box(
                ctx,
                &format!("{}\n{}\n{}\n{}\n{}",
                "Use arrow keys or WASD to navigate the world and menus.",
                "Press Space to select menu options.",
                "Press Backspace to enter and exit your inventory.",
                "Battle spirits in an endless dungeon. Collect enough to create stronger spirits.",
                "Select a spirit in your inventory to combine it with others"),
                (INFO_AREA.0 + 8, INFO_AREA.1 + 8, INFO_AREA.2 - 16),
            );
            Ok(())
        },
        PlayState::GameOver => {
            text_outline_color(
                ctx,
                GAME_OVER,
                [1.0, 0.0, 0.0, 1.0].into(),
            )?;
            text_in_box(
                ctx,
                &format!("Game Over"),
                (GAME_OVER.0 + 8, GAME_OVER.1 + 8, GAME_OVER.2 - 16),
            )?;
            Ok(())
        },
        _ => {
            Ok(())
        }
    }
}
