use ggez::*;
use specs::*;
use state::*;
use input::Direction;

pub fn move_cursor(ctx: &mut Context, world: &mut World, direction: &Direction) {
    let mut battle_state = world.write_resource::<BattleState>();
    match (battle_state.active_entity, battle_state.combat_move) {
        (Some(entity), Some(index)) => {
            let spirit = world.read_storage::<Spirit>();
            match spirit.get(entity) {
                Some(spirit) => {
                },
                None => {

                }
            }
        },
        _ => {

        }
    }
}

pub fn select_target(ctx: &mut Context, world: &mut World) {
    world.write_resource::<BattleState>().want_attack();
}

pub struct HandleBattleMenu;
impl<'a> System<'a> for HandleBattleMenu {
    type SystemData = (
        ReadExpect<'a, PlayState>,
        WriteExpect<'a, InputState>,
        WriteExpect<'a, BattleState>,
    );
    fn run(&mut self, (play_state, mut input_state, mut battle_state): Self::SystemData) {
        match (play_state.clone(), input_state.clone(), battle_state.retreating) {
            (PlayState::InBattle, InputState::Move(direction), false) => {
                if let Some(index) = battle_state.combat_move {
                    let next_index = (match direction {
                        Direction::Up => index + 4 - 2,
                        Direction::Down => index + 2,
                        Direction::Left => index + 4 - 1,
                        Direction::Right => index + 1,
                    }) % 4;
                    battle_state.combat_move = Some(
                        next_index,
                    );
                }
                *input_state = InputState::Rest;
            },
            (PlayState::InBattle, InputState::Select, false) => {
                battle_state.want_attack();
                *input_state = InputState::Rest;
            }
            _ => {}
        }
    }
}
