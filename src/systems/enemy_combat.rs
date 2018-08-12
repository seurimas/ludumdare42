use ggez::*;
use specs::*;
use state::*;
use rand::*;

fn act<'a>(spirit: &Spirit, self_entity: &Entity, player_entity: &Entity, updater: &mut LazyUpdate) {
    let mut rng = thread_rng();
    if let Some(action) = rng.choose(&spirit.moves) {
        match action.effect {
            MoveType::DamageOne(amount) => {
                updater.insert(
                    *player_entity,
                    CombatEffects::new(vec![CombatEffect::Damage(amount)])
                );
            },
            MoveType::DamageMany(amount) => {
                updater.insert(
                    *player_entity,
                    CombatEffects::new(vec![CombatEffect::Damage(amount)])
                );
            },
            MoveType::Heal(amount) => {
                updater.insert(
                    *self_entity,
                    CombatEffects::new(vec![CombatEffect::Heal(amount)])
                );
            },
            MoveType::Defend(amount) => {
                updater.insert(
                    *self_entity,
                    CombatEffects::new(vec![CombatEffect::Defense(amount)])
                );
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
    );

    fn run(&mut self, (mut battle_state, entities, spirits, player_spirits, mut updater): Self::SystemData) {
        if battle_state.enemy_attacking {
            let mut player_entity = None;
            for (entity, spirit, player_spirit) in (&*entities, &spirits, &player_spirits).join() {
                if player_spirit.active {
                    player_entity = Some(entity);
                }
            }
            if let Some(player_entity) = player_entity {
                let mut attacking_spirits = Vec::new();
                for (entity, spirit, ()) in (&*entities, &spirits, !&player_spirits).join() {
                    if spirit.health > 0 {
                        attacking_spirits.push((entity, spirit.clone()));
                    }
                }
                for (entity, spirit) in attacking_spirits.iter() {
                    act(spirit, &entity, &player_entity, &mut updater);
                }
                battle_state.enemy_attacking = false;
            }
        }
    }
}
