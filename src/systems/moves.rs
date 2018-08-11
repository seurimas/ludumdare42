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
        Entities<'a>,
        ReadStorage<'a, Spirit>,
        ReadStorage<'a, PlayerSpirit>,
    );

    fn run(&mut self, (mut battle_state, entities, spirits, player_spirits): Self::SystemData) {
        let mut players_alive = false;
        for (spirit, player_spirit) in (&spirits, &player_spirits).join() {
            if spirit.health > 0 {
                players_alive = true;
            }
        }
        if !players_alive {
            battle_state.in_combat = false; // Leave combat.
        }
    }
}
