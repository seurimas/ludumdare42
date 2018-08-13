use ggez::*;
use specs::*;
use state::*;
use input::Direction;

pub struct HandleMove;
impl<'a> System<'a> for HandleMove {
    type SystemData = (
        WriteExpect<'a, PlayState>,
        WriteExpect<'a, InputState>,
        ReadExpect<'a, Level>,
        WriteStorage<'a, WorldEntity>,
        ReadStorage<'a, Player>,
        ReadExpect<'a, Sounds>,
    );
    fn run(&mut self, (mut play_state, mut input_state, level, mut world_entities, players, sounds): Self::SystemData) {
        match (play_state.clone(), input_state.clone()) {
            (PlayState::InWorld, InputState::Move(direction)) => {
                for (mut world_entity, player) in (&mut world_entities, &players).join() {
                    match move_in_level(world_entity.location, &direction, &level) {
                        Some(next) => { world_entity.location = next; },
                        _ => {}
                    }
                }
                *input_state = InputState::Rest;
            },
            (PlayState::InWorld, InputState::Escape) => {
                sounds.play(&sounds.cancel);
                *input_state = InputState::Rest;
                *play_state = PlayState::Combining;
            }
            _ => {}
        }
    }
}
