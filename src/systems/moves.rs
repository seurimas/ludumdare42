use ggez::*;
use specs::*;
use state::*;
use std::time::Duration;

fn get_active_enemies<'a>(
    amount: usize,
    entities: &Entities<'a>,
    spirits: &WriteStorage<'a, Spirit>,
    player_spirits: &ReadStorage<'a, PlayerSpirit>
) -> Vec<Entity> {
    let mut affected = Vec::new();
    for (entity, spirit, ()) in (&**entities, spirits, !player_spirits).join() {
        if spirit.health > 0 && affected.len() < amount {
            affected.push(entity.clone());
        }
    }
    affected
}

fn get_active_ally<'a>(
    entities: &Entities<'a>,
    spirits: &WriteStorage<'a, Spirit>,
    player_spirits: &ReadStorage<'a, PlayerSpirit>
) -> Option<Entity> {
    let mut found = None;
    for (entity, spirit, player_spirit) in (&**entities, spirits, player_spirits).join() {
        if spirit.health > 0 && player_spirit.active {
            found = Some(entity.clone());
        }
    }
    found
}

pub struct WatchAttack;
impl<'a> System<'a> for WatchAttack {
    type SystemData = (
        Write<'a, BattleState>,
        Entities<'a>,
        WriteStorage<'a, Spirit>,
        ReadStorage<'a, PlayerSpirit>,
        WriteStorage<'a, CombatEffects>,
    );

    fn run(&mut self, (mut battle_state, mut entities, mut spirits, player_spirits, mut combat_effects): Self::SystemData) {
        if battle_state.activate {
            let my_move = battle_state.get_move(&spirits);
            match my_move.map(|v| v.effect) {
                Some(MoveType::DamageMany(amount)) => {
                    let affected = get_active_enemies(
                        3,
                        &entities,
                        &spirits,
                        &player_spirits,
                    );
                    for entity in affected.iter() {
                        combat_effects.insert(*entity, CombatEffects::new(vec![CombatEffect::Damage(amount)]));
                    }
                    battle_state.finish_attack();
                },
                Some(MoveType::DamageOne(amount)) => {
                    let affected = get_active_enemies(
                        1,
                        &entities,
                        &spirits,
                        &player_spirits,
                    );
                    for entity in affected.iter() {
                        combat_effects.insert(*entity, CombatEffects::new(vec![CombatEffect::Damage(amount)]));
                    }
                    battle_state.finish_attack();
                },
                Some(MoveType::Heal(amount)) => {
                    if let Some(player) = get_active_ally(&entities, &spirits, &player_spirits) {
                        combat_effects.insert(player, CombatEffects::new(vec![CombatEffect::Heal(amount)]));
                    }
                    battle_state.finish_attack();
                },
                _ => {

                }
            }
        }
    }
}

pub struct TickEffects;
impl<'a> System<'a> for TickEffects {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Spirit>,
        WriteStorage<'a, CombatEffects>,
        Read<'a, Duration>,
        Write<'a, BattleState>,
    );

    fn run(&mut self, (entities, mut spirits, mut combat_effects, delta_time, mut battle_state): Self::SystemData) {
        let mut completed = Vec::new();
        for (entity, spirit, combat_effect) in (&*entities, &mut spirits, &mut combat_effects).join() {
            if combat_effect.update(*delta_time) {
                combat_effect.apply_tick(spirit);
            }
            if !combat_effect.active() {
                completed.push(entity.clone());
            }
        }
        for complete in completed.iter() {
            combat_effects.remove(*complete);
        }
        let mut animating = false;
        for (entity, spirit, combat_effect) in (&*entities, &mut spirits, &mut combat_effects).join() {
            animating = true;
        }
        battle_state.animating = animating;
    }
}

pub struct WatchSpirits;
impl<'a> System<'a> for WatchSpirits {
    type SystemData = (
        Write<'a, BattleState>,
        WriteExpect<'a, PlayState>,
        Entities<'a>,
        ReadStorage<'a, Spirit>,
        WriteStorage<'a, PlayerSpirit>,
        WriteStorage<'a, Player>,
    );

    fn run(&mut self, (mut battle_state, mut play_state, entities, spirits, mut player_spirits, mut players): Self::SystemData) {
        if *play_state == PlayState::InBattle {
            let mut players_alive = false;
            let mut retreating = true;
            for (spirit, player_spirit) in (&spirits, &mut player_spirits).join() {
                if spirit.health > 0 {
                    players_alive = true;
                    if player_spirit.active {
                        retreating = false;
                    }
                } else if player_spirit.active {
                    player_spirit.active = false;
                }
            }
            if !players_alive {
                *play_state = PlayState::GameOver;
                battle_state.in_combat = false; // Leave combat.
                ()
            }
            if !battle_state.retreating && retreating {
                battle_state.retreat();
            }
            let mut enemies_alive = false;
            let mut captured_enemies = Vec::new();
            for (spirit, ()) in (&spirits, !&player_spirits).join() {
                if spirit.health > 0 {
                    enemies_alive = true;
                }
                captured_enemies.push(spirit.element.clone());
            }
            if !enemies_alive  {
                let mut captured = Vec::new();
                let mut lost = Vec::new();
                let mut new_spirits = Vec::new();
                for (spirit, _player_spirit) in (&spirits, &player_spirits).join() {
                    if spirit.health > 0 {
                        new_spirits.push(spirit.clone());
                    }
                }
                for player in (&mut players).join() {
                    for captured_spirit in captured_enemies.iter() {
                        if player.spirits.len() < 25 {
                            captured.push(captured_spirit.clone());
                            new_spirits.push(Spirit::new(captured_spirit.clone(), true));
                        } else {
                            lost.push(captured_spirit.clone());
                        }
                    }
                    player.spirits = new_spirits.clone();
                }
                *play_state = PlayState::Looting {
                    captured,
                    lost,
                };
                battle_state.in_combat = false;
                if let Some(encounter) = battle_state.encounter_entity {
                    (*entities).delete(encounter);
                }
                for (entity, spirit) in (&*entities, &spirits).join() {
                    (*entities).delete(entity);
                }
            }
        }
    }
}
