use ggez::*;
use specs::*;
use state::*;

pub struct HandleLootMenu;
impl<'a> System<'a> for HandleLootMenu {
    type SystemData = (
        WriteExpect<'a, PlayState>,
        WriteExpect<'a, InputState>,
    );
    fn run(&mut self, (mut play_state, mut input_state): Self::SystemData) {
        match (play_state.clone(), input_state.clone()) {
            (PlayState::Looting { captured, lost }, InputState::Select) => {
                *input_state = InputState::Rest;
                *play_state = PlayState::InWorld;
            },
            (PlayState::Looting { captured, lost }, InputState::Escape) => {
                *input_state = InputState::Rest;
                *play_state = PlayState::InWorld;
            },
            _ => {}
        }
    }
}
