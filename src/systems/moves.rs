use ggez::*;
use specs::*;
use state::*;

pub struct WatchAttack;
impl<'a> System<'a> for WatchAttack {
    type SystemData = (
        Write<'a, BattleState>,
        WriteStorage<'a, Spirit>,
        ReadStorage<'a, PlayerSpirit>,
    );

    fn run(&mut self, (mut battle_state, mut spirits, player_spirits): Self::SystemData) {
        if battle_state.activate {
            let my_move = battle_state.get_move(&spirits);
            match my_move.map(|v| v.effect) {
                Some(MoveType::DamageMany(amount)) => {
                    let mut amount_left = amount;
                    for (spirit, ()) in (&mut spirits, !&player_spirits).join() {
                        let dealt = u32::min(spirit.health, amount_left);
                        amount_left -= dealt;
                        spirit.health -= dealt;
                    }
                    battle_state.finish_attack();
                },
                Some(MoveType::DamageOne(amount)) => {
                    let mut amount_left = amount;
                    for (spirit, ()) in (&mut spirits, !&player_spirits).join() {
                        let dealt = u32::min(spirit.health, amount_left);
                        if spirit.health > 0 {
                            amount_left = 0;
                            spirit.health -= dealt;
                        }
                    }
                    battle_state.finish_attack();
                },
                Some(MoveType::Heal(amount)) => {
                    for (spirit, player_spirit) in (&mut spirits, &player_spirits).join() {
                        if player_spirit.active && spirit.health > 0 {
                            spirit.health += amount;
                            if spirit.health > spirit.max_health {
                                spirit.health = spirit.max_health;
                            }
                        }
                    }
                    battle_state.finish_attack();
                },
                _ => {

                }
            }
        }
    }
}

pub struct WatchSpirits;
impl<'a> System<'a> for WatchSpirits {
    type SystemData = (
        Write<'a, BattleState>,
        WriteExpect<'a, PlayState>,
        Entities<'a>,
        ReadStorage<'a, Spirit>,
        ReadStorage<'a, PlayerSpirit>,
    );

    fn run(&mut self, (mut battle_state, mut play_state, entities, spirits, player_spirits): Self::SystemData) {
        if *play_state == PlayState::InBattle {
            let mut players_alive = false;
            for (spirit, _player_spirit) in (&spirits, &player_spirits).join() {
                if spirit.health > 0 {
                    players_alive = true;
                }
            }
            if !players_alive {
                *play_state = PlayState::GameOver;
                battle_state.in_combat = false; // Leave combat.
                ()
            }
            let mut enemies_alive = false;
            for (spirit, ()) in (&spirits, !&player_spirits).join() {
                if spirit.health > 0 {
                    enemies_alive = true;
                }
            }
            if !enemies_alive  {
                *play_state = PlayState::InWorld;
                battle_state.in_combat = false;
                if let Some(encounter) = battle_state.encounter_entity {
                    (*entities).delete(encounter);
                }
            }
        }
    }
}
