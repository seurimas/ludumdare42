use ggez::*;
use specs::*;
use state::*;
use input::Direction;

pub struct HandleInventory;

fn move_cursor<'a>(inventory_state: &mut WriteExpect<'a, InventoryState>, direction: Direction) -> bool {
    match direction {
        Direction::Up => {
            if inventory_state.index < 5 {
                false
            } else {
                inventory_state.index -= 5;
                true
            }
        },
        Direction::Left => {
            if inventory_state.index <= 0 {
                false
            } else {
                inventory_state.index -= 1;
                true
            }
        },
        Direction::Right => {
            if inventory_state.index >= 24 {
                false
            } else {
                inventory_state.index += 1;
                true
            }
        },
        Direction::Down => {
            if inventory_state.index >= 20 {
                false
            } else {
                inventory_state.index += 5;
                true
            }
        },
    }
}
fn next_spirit(spirit: Spirit) -> Spirit {
    match spirit.element {
        SpiritType::Fire(level) => {
            Spirit {
                element: SpiritType::Fire(level + 1),
                max_health: spirit.max_health * 4,
                health: spirit.max_health * 4,
                moves: spirit.moves,
                attack: spirit.attack,
                base_defense: spirit.base_defense,
                stamina: spirit.stamina,
                defense: spirit.base_defense,
            }
        },
        SpiritType::Water(level) => {
            Spirit {
                element: SpiritType::Water(level + 1),
                max_health: spirit.max_health * 4,
                health: spirit.max_health * 4,
                moves: spirit.moves,
                attack: spirit.attack,
                base_defense: spirit.base_defense,
                stamina: spirit.stamina,
                defense: spirit.base_defense,
            }
        },
        SpiritType::Slime(level) => {
            Spirit {
                element: SpiritType::Slime(level + 1),
                max_health: spirit.max_health * 4,
                health: spirit.max_health * 4,
                moves: spirit.moves,
                attack: spirit.attack,
                base_defense: spirit.base_defense,
                stamina: spirit.stamina,
                defense: spirit.base_defense,
            }
        },
        SpiritType::Light(level) => {
            Spirit {
                element: SpiritType::Light(level + 1),
                max_health: spirit.max_health * 4,
                health: spirit.max_health * 4,
                moves: spirit.moves,
                attack: spirit.attack,
                base_defense: spirit.base_defense,
                stamina: spirit.stamina,
                defense: spirit.base_defense,
            }
        },
        SpiritType::Dark(level) => {
            Spirit {
                element: SpiritType::Dark(level + 1),
                max_health: spirit.max_health * 4,
                health: spirit.max_health * 4,
                moves: spirit.moves,
                attack: spirit.attack,
                base_defense: spirit.base_defense,
                stamina: spirit.stamina,
                defense: spirit.base_defense,
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
) -> bool {
    let mut idx = 0;
    let mut found = false;
    for (entity, spirit, player_spirit) in (&**entities, spirits, player_spirits).join() {
        if idx == inventory_state.index && spirit.health > 0 {
            battle_state.active_entity = Some(entity);
            battle_state.retreating = false;
            battle_state.enemy_attacking = None;
            battle_state.notification = None;
            player_spirit.active = true;
            found = true;
        } else {
            player_spirit.active = false;
        }
        idx += 1;
    }
    found
}
fn combine_spirits(player: &mut Player, index: usize) -> bool {
    let mut compatriots = Vec::new();
    let mut collided = false;
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
            collided = true;
        }
    }
    collided
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
        ReadExpect<'a, Sounds>,
    );
    fn run(&mut self, (entities, mut play_state, mut input_state, mut battle_state, mut inventory_state, mut players, mut spirits, mut player_spirits, sounds): Self::SystemData) {
        match (play_state.clone(), battle_state.retreating) {
            (PlayState::Combining, _) => {
                match input_state.clone() {
                    InputState::Move(direction) => {
                        if move_cursor(&mut inventory_state, direction) {
                            sounds.play(&sounds.blip);
                        }
                        *input_state = InputState::Rest;
                    },
                    InputState::Select => {
                        for player in (&mut players).join() {
                            if combine_spirits(player, inventory_state.index) {
                                sounds.play(&sounds.collide);
                            }
                        }
                        *input_state = InputState::Rest;
                    }
                    InputState::Escape => {
                        sounds.play(&sounds.cancel);
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
                        if move_cursor(&mut inventory_state, direction) {
                            sounds.play(&sounds.blip);
                        }
                        *input_state = InputState::Rest;
                    },
                    InputState::Select => {
                        if select_fighter(&inventory_state, &mut battle_state, &entities, &spirits, &mut player_spirits) {
                            sounds.play(&sounds.confirm);
                        }
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
