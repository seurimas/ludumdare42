use ggez::*;
use specs::*;
use state::*;
use input::*;
use std::time::Duration;
use rand::*;

pub struct WanderEncounters;
impl<'a> System<'a> for WanderEncounters {
    type SystemData = (
        WriteStorage<'a, Encounter>,
        WriteStorage<'a, WorldEntity>,
        ReadExpect<'a, Level>,
        ReadExpect<'a, Duration>,
    );
    fn run(&mut self, (mut encounters, mut world_entities, level, delta_time): Self::SystemData) {
        let mut rng = thread_rng();
        for (encounter, world_entity) in (&mut encounters, &mut world_entities).join() {
            if encounter.update(*delta_time) {
                let direction = rng.choose(&[
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                ]);
                if let Some(direction) = direction {
                    if let Some(moved) = move_in_level(world_entity.location, direction, &level) {
                        world_entity.location = moved;
                    }
                }
            }
        }
    }
}

pub struct FindEncounters;
impl<'a> System<'a> for FindEncounters {
    type SystemData = (
        WriteExpect<'a, PlayState>,
        Write<'a, BattleState>,
        Entities<'a>,
        ReadStorage<'a, WorldEntity>,
        ReadStorage<'a, Encounter>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Spirit>,
        WriteStorage<'a, PlayerSpirit>,
    );
    fn run(&mut self, (mut play_state, mut battle_state, entities, world_entities, encounters, player_store, mut spirits, mut player_spirits): Self::SystemData) {
        if *play_state == PlayState::InWorld {
            let mut player_loc = (0, 0);
            let mut player = None;
            for (world_entity, player_comp) in (&world_entities, &player_store).join() {
                player_loc = world_entity.location;
                player = Some(player_comp.clone());
            }
            if let Some(player) = player {
                for (entity, world_entity, encounter) in (&*entities, &world_entities, &encounters).join() {
                    if world_entity.location == player_loc {
                        battle_state.encounter_entity = Some(entity);
                        *play_state = PlayState::InBattle;
                        for spirit in encounter.spirits.clone() {
                            (*entities).build_entity()
                                .with(spirit, &mut spirits)
                                .build();
                        }
                        let mut active = true;
                        for spirit in player.spirits.clone() {
                            let entity = (*entities).build_entity()
                                .with(spirit, &mut spirits)
                                .with(PlayerSpirit { active }, &mut player_spirits)
                                .build();
                            if active {
                                battle_state.combat_move = Some(0);
                                battle_state.active_entity = Some(entity);
                                battle_state.in_combat = true;
                                battle_state.activate = false;
                            }
                            active = false;
                        }
                    }
                }
            }
        }
    }
}
