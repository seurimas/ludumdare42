use ggez::*;
use specs::*;
use state::*;
use input::Direction;

pub struct HandleInventory;

fn move_cursor<'a>(inventory_state: &mut WriteExpect<'a, InventoryState>, direction: Direction) {
    match direction {
        Direction::Up => {
            if inventory_state.index < 5 {
                ()
            } else {
                inventory_state.index -= 5;
            }
        },
        Direction::Left => {
            if inventory_state.index <= 0 {
                ()
            } else {
                inventory_state.index -= 1;
            }
        },
        Direction::Right => {
            if inventory_state.index >= 24 {
                ()
            } else {
                inventory_state.index += 1;
            }
        },
        Direction::Down => {
            if inventory_state.index >= 20 {
                ()
            } else {
                inventory_state.index += 5;
            }
        },
    }
}
fn next_spirit(spirit: Spirit) -> Spirit {
    match spirit.element {
        SpiritType::Fire(level) => {
            Spirit {
                element: SpiritType::Fire(level + 1),
                max_health: spirit.max_health * 2,
                health: spirit.max_health * 2,
                moves: spirit.moves,
                defense: 0,
            }
        },
        SpiritType::Water(level) => {
            Spirit {
                element: SpiritType::Water(level + 1),
                max_health: spirit.max_health * 2,
                health: spirit.max_health * 2,
                moves: spirit.moves,
                defense: 0,
            }
        },
        SpiritType::Slime(level) => {
            Spirit {
                element: SpiritType::Slime(level + 1),
                max_health: spirit.max_health * 2,
                health: spirit.max_health * 2,
                moves: spirit.moves,
                defense: 0,
            }
        },
        SpiritType::Light(level) => {
            Spirit {
                element: SpiritType::Light(level + 1),
                max_health: spirit.max_health * 2,
                health: spirit.max_health * 2,
                moves: spirit.moves,
                defense: 0,
            }
        },
        SpiritType::Dark(level) => {
            Spirit {
                element: SpiritType::Dark(level + 1),
                max_health: spirit.max_health * 2,
                health: spirit.max_health * 2,
                moves: spirit.moves,
                defense: 0,
            }
        },
    }
}
fn select_fighter<'a>(
    inventory_state: &InventoryState,
    battle_state: &mut BattleState,
    entities: &Entities<'a>,
    spirits: &WriteStorage<'a, Spirit>,
    player_spirits: &mut WriteStorage<'a, PlayerSpirit>,
) {
    let mut idx = 0;
    for (entity, spirit, player_spirit) in (&**entities, spirits, player_spirits).join() {
        if idx == inventory_state.index && spirit.health > 0 {
            battle_state.active_entity = Some(entity);
            battle_state.retreating = false;
            player_spirit.active = true;
        }
        idx += 1;
    }
}
fn combine_spirits(player: &mut Player, index: usize) {
    let mut compatriots = Vec::new();
    if let Some(spirit) = player.spirits.get(index).map(|s| s.clone()) {
        for (idx, other) in player.spirits.iter().enumerate() {
            if other.element == spirit.element {
                compatriots.push(idx);
            }
        }
        let used = required_spirits(&spirit.element) as usize;
        if compatriots.len() >= used {
            compatriots.reverse();
            for (count, idx) in compatriots.iter().enumerate() {
                if count < used {
                    player.spirits.remove(*idx);
                }
            }
            player.spirits.insert(0, next_spirit(spirit));
        }
    }
}
impl<'a> System<'a> for HandleInventory {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, PlayState>,
        WriteExpect<'a, InputState>,
        WriteExpect<'a, BattleState>,
        WriteExpect<'a, InventoryState>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Spirit>,
        WriteStorage<'a, PlayerSpirit>,
    );
    fn run(&mut self, (entities, mut play_state, mut input_state, mut battle_state, mut inventory_state, mut players, mut spirits, mut player_spirits): Self::SystemData) {
        match (play_state.clone(), battle_state.retreating) {
            (PlayState::Combining, _) => {
                match input_state.clone() {
                    InputState::Move(direction) => {
                        move_cursor(&mut inventory_state, direction);
                        *input_state = InputState::Rest;
                    },
                    InputState::Select => {
                        for player in (&mut players).join() {
                            combine_spirits(player, inventory_state.index);
                        }
                        *input_state = InputState::Rest;
                    }
                    InputState::Escape => {
                        *play_state = PlayState::InWorld;
                        *input_state = InputState::Rest;
                    },
                    _ => {

                    }
                }
            },
            (PlayState::InBattle, true) => {
                match input_state.clone() {
                    InputState::Move(direction) => {
                        move_cursor(&mut inventory_state, direction);
                        *input_state = InputState::Rest;
                    },
                    InputState::Select => {
                        select_fighter(&inventory_state, &mut battle_state, &entities, &spirits, &mut player_spirits);
                        *input_state = InputState::Rest;
                    }
                    _ => {

                    }
                }
            },
            _ => {}
        }
    }
}
