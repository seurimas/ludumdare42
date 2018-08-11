use ggez::*;
use specs::*;
use state::*;

type WorldEntities<'a> = (
    Entities<'a>,
    ReadStorage<'a, WorldEntity>,
);
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
                for (world_entity, encounter) in (&world_entities, &encounters).join() {
                    if world_entity.location == player_loc {
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
