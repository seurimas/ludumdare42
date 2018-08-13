mod world;
mod combat;
mod inventory;
mod looting;

use state::*;
use ggez::*;
use specs::*;
pub use self::world::HandleMove;
pub use self::combat::HandleBattleMenu;
pub use self::inventory::HandleInventory;
pub use self::looting::HandleLootMenu;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}


pub struct HandleMainMenu;
impl<'a> System<'a> for HandleMainMenu {
    type SystemData = (
        WriteExpect<'a, PlayState>,
        WriteExpect<'a, InputState>,
        ReadExpect<'a, Sounds>,
    );
    fn run(&mut self, (mut play_state, mut input_state, sounds): Self::SystemData) {
        match (play_state.clone(), input_state.clone()) {
            (PlayState::MainMenu(val), InputState::Move(dir)) => {
                sounds.play(&sounds.blip);
                *input_state = InputState::Rest;
                *play_state = PlayState::MainMenu(match dir {
                    Direction::Up => {
                        0
                    },
                    Direction::Right => {
                        1
                    },
                    Direction::Down => {
                        2
                    },
                    Direction::Left => {
                        3
                    },
                });
            },
            (PlayState::MainMenu(0), InputState::Select) => {
                sounds.play(&sounds.confirm);
                *input_state = InputState::Rest;
                *play_state = PlayState::Stairs(0);
            },
            (PlayState::GameOver, _) => {
                if *input_state != InputState::Rest {
                    sounds.play(&sounds.confirm);
                    *input_state = InputState::Rest;
                    *play_state = PlayState::MainMenu(0);
                }
            }
            _ => {}
        }
    }
}
