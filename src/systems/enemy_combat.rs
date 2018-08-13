use ggez::*;
use specs::*;
use state::*;
use rand::*;
use render::*;

fn act<'a>(
    battle_state: &mut BattleState,
    spirit: &Spirit,
    player_spirit: &Spirit,
    self_entity: &Entity,
    player_entity: &Entity,
    updater: &mut LazyUpdate,
    sounds: &Sounds,
) {
    let mut rng = thread_rng();
    if let Some(action) = rng.choose(&spirit.moves) {
        match action.effect {
            MoveType::DamageOne(amount) => {
                let amount = action.effect.actual_amount(spirit, player_spirit);
                updater.insert(
                    *player_entity,
                    CombatEffects::new(vec![CombatEffect::Damage(amount), CombatEffect::ShedDefense(1)])
                );
                sounds.sound_for_attack(spirit);
                battle_state.notify(damage_one_text(&action, &spirit, &player_spirit, amount, true));
            },
            MoveType::DamageMany(amount) => {
                let amount = action.effect.actual_amount(spirit, player_spirit);
                updater.insert(
                    *player_entity,
                    CombatEffects::new(vec![CombatEffect::Damage(amount)])
                );
                sounds.sound_for_attack(spirit);
                battle_state.notify(damage_one_text(&action, &spirit, &player_spirit, amount, true));
            },
            MoveType::Heal(amount) => {
                println!("{:?}", *self_entity);
                println!("{:?}", *spirit);
                let amount = action.effect.actual_amount(spirit, spirit);
                updater.insert(
                    *self_entity,
                    CombatEffects::new(vec![CombatEffect::Heal(amount)])
                );
                battle_state.notify(heal_text(&action, &spirit, amount));
            },
            MoveType::Defend(amount) => {
                let amount = action.effect.actual_amount(spirit, spirit);
                updater.insert(
                    *self_entity,
                    CombatEffects::new(vec![CombatEffect::Defense(amount)])
                );
                battle_state.notify(defense_text(&action, &spirit, amount));
            },
        }
    }
}

pub struct EnemyCombat;
impl<'a> System<'a> for EnemyCombat {
    type SystemData = (
        Write<'a, BattleState>,
        Entities<'a>,
        ReadStorage<'a, Spirit>,
        ReadStorage<'a, PlayerSpirit>,
        Write<'a, LazyUpdate>,
        ReadExpect<'a, Sounds>,
        WriteExpect<'a, InputState>,
    );

    fn run(&mut self, (mut battle_state, entities, spirits, player_spirits, mut updater, sounds, mut input_state): Self::SystemData) {
        if !battle_state.animating() && !battle_state.retreating {
            if let Some(attacking) = battle_state.enemy_attacking {
                println!("{}", attacking);
                let mut player = None;
                let mut attacking = attacking;
                for (entity, spirit, player_spirit) in (&*entities, &spirits, &player_spirits).join() {
                    if player_spirit.active {
                        player = Some((entity, spirit));
                    }
                }
                if let Some((player_entity, player_spirit)) = player {
                    let mut attacking_spirits = Vec::new();
                    for (entity, spirit, ()) in (&*entities, &spirits, !&player_spirits).join() {
                        if spirit.health > 0 {
                            attacking_spirits.push((entity, spirit.clone()));
                        }
                    }
                    if attacking_spirits.len() > 0 {
                        if attacking as usize >= attacking_spirits.len() {
                            attacking = attacking_spirits.len() as u32 - 1;
                        }
                        for (idx, (entity, spirit)) in attacking_spirits.iter().enumerate() {
                            if idx as u32 == attacking {
                                act(&mut battle_state, spirit, player_spirit, &entity, &player_entity, &mut updater, &sounds);
                                *input_state = InputState::Rest;
                            }
                        }
                        if attacking > 0 {
                            battle_state.enemy_attacking = Some(attacking - 1);
                        } else {
                            battle_state.enemy_attacking = None;
                        }
                    }
                }
            }
        }
    }
}
