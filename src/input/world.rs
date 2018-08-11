use ggez::*;
use specs::*;
use state::*;
use input::Direction;

fn move_by(loc: (u32, u32), direction: &Direction) -> Option<(u32, u32)> {
    match direction {
        Direction::Up => {
            if loc.1 > 0 {
                Some((loc.0, loc.1 - 1))
            } else {
                None
            }
        },
        Direction::Down => {
            Some((loc.0, loc.1 + 1))
        },
        Direction::Left => {
            if loc.0 > 0 {
                Some((loc.0 - 1, loc.1))
            } else {
                None
            }
        },
        Direction::Right => {
            Some((loc.0 + 1, loc.1))
        },
    }
}

fn move_in_level(loc: (u32, u32), direction: &Direction, level: &Level) -> Option<(u32, u32)> {
    match move_by(loc, direction) {
        Some(next) => {
            if level.has_tile(next) {
                Some(next)
            } else {
                None
            }
        },
        None => None,
    }
}

pub struct HandleMove;
impl<'a> System<'a> for HandleMove {
    type SystemData = (
        WriteExpect<'a, PlayState>,
        WriteExpect<'a, InputState>,
        ReadExpect<'a, Level>,
        WriteStorage<'a, WorldEntity>,
        ReadStorage<'a, Player>,
    );
    fn run(&mut self, (mut play_state, mut input_state, level, mut world_entities, players): Self::SystemData) {
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
                *input_state = InputState::Rest;
                *play_state = PlayState::Combining;
            }
            _ => {}
        }
    }
}
