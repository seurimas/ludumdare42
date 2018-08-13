use ggez::*;
use specs::*;
use state::*;

pub struct HandleLootMenu;
impl<'a> System<'a> for HandleLootMenu {
    type SystemData = (
        WriteExpect<'a, PlayState>,
        WriteExpect<'a, InputState>,
        ReadExpect<'a, Sounds>,
    );
    fn run(&mut self, (mut play_state, mut input_state, sounds): Self::SystemData) {
        match (play_state.clone(), input_state.clone()) {
            (PlayState::Looting { captured, lost }, InputState::Select) => {
                sounds.play(&sounds.confirm);
                *input_state = InputState::Rest;
                *play_state = PlayState::InWorld;
            },
            (PlayState::Looting { captured, lost }, InputState::Escape) => {
                sounds.play(&sounds.confirm);
                *input_state = InputState::Rest;
                *play_state = PlayState::InWorld;
            },
            _ => {}
        }
    }
}
