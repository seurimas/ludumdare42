use ggez::*;
use specs::*;
use state::*;
use std::time::Duration;

fn get_active_enemies<'a>(
    amount: usize,
    entities: &Entities<'a>,
    spirits: &WriteStorage<'a, Spirit>,
    player_spirits: &ReadStorage<'a, PlayerSpirit>
) -> Vec<(Entity, Spirit)> {
    let mut affected = Vec::new();
    for (entity, spirit, ()) in (&**entities, spirits, !player_spirits).join() {
        if spirit.health > 0 && affected.len() < amount {
            affected.push((entity.clone(), spirit.clone()));
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
        ReadExpect<'a, Sounds>,
    );

    fn run(&mut self, (mut battle_state, mut entities, mut spirits, player_spirits, mut combat_effects, sounds): Self::SystemData) {
        if battle_state.activate {
            let my_move = battle_state.get_move(&spirits);
            if let (Some(player), Some(my_move))
                = (get_active_ally(&entities, &spirits, &player_spirits), my_move) {
                if let Some(player_spirit) = spirits.get(player) {
                    match my_move.effect {
                        MoveType::DamageMany(amount) => {
                            let affected = get_active_enemies(
                                3,
                                &entities,
                                &spirits,
                                &player_spirits,
                            );
                            for (entity, enemy) in affected.iter() {
                                let amount = my_move.effect.actual_amount(&player_spirit, enemy);
                                combat_effects.insert(*entity, CombatEffects::new(vec![CombatEffect::Damage(amount)]));
                            }
                            sounds.sound_for_attack(player_spirit);
                            battle_state.finish_attack();
                        },
                        MoveType::DamageOne(amount) => {
                            let affected = get_active_enemies(
                                1,
                                &entities,
                                &spirits,
                                &player_spirits,
                            );
                            for (entity, enemy) in affected.iter() {
                                let amount = my_move.effect.actual_amount(&player_spirit, enemy);
                                combat_effects.insert(*entity, CombatEffects::new(vec![CombatEffect::Damage(amount), CombatEffect::ShedDefense(1)]));
                            }
                            sounds.sound_for_attack(player_spirit);
                            battle_state.finish_attack();
                        },
                        MoveType::Heal(amount) => {
                            let amount = my_move.effect.actual_amount(&player_spirit, &player_spirit);
                            combat_effects.insert(player, CombatEffects::new(vec![CombatEffect::Heal(amount)]));
                            battle_state.finish_attack();
                        },
                        MoveType::Defend(amount) => {
                            let amount = my_move.effect.actual_amount(&player_spirit, &player_spirit);
                            combat_effects.insert(player, CombatEffects::new(vec![CombatEffect::Defense(amount)]));
                            battle_state.finish_attack();
                        },
                    }
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
        ReadExpect<'a, Sounds>,
    );

    fn run(&mut self, (entities, mut spirits, mut combat_effects, delta_time, mut battle_state, sounds): Self::SystemData) {
        let mut completed = Vec::new();
        for (entity, spirit, combat_effect) in (&*entities, &mut spirits, &mut combat_effects).join() {
            if combat_effect.update(*delta_time) {
                println!("{:?}", combat_effect);
                combat_effect.apply_tick(spirit);
                sounds.play(&sounds.blip);
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
            println!("{:?}", *battle_state);
        }
        battle_state.set_animating(animating);
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
        ReadExpect<'a, Sounds>,
    );

    fn run(&mut self, (mut battle_state, mut play_state, entities, spirits, mut player_spirits, mut players, sounds): Self::SystemData) {
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
                sounds.lose.play();
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
