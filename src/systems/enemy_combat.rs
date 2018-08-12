use ggez::*;
use specs::*;
use state::*;

fn act<'a>(spirit: &Spirit, player_entity: &Entity, spirits: &mut WriteStorage<'a, Spirit>) {
    
}

pub struct EnemyCombat;
impl<'a> System<'a> for EnemyCombat {
    type SystemData = (
        Write<'a, BattleState>,
        Entities<'a>,
        WriteStorage<'a, Spirit>,
        ReadStorage<'a, PlayerSpirit>,
    );

    fn run(&mut self, (mut battle_state, entities, mut spirits, player_spirits): Self::SystemData) {
        if battle_state.enemy_attacking {
            let mut player_entity = None;
            for (entity, spirit, player_spirite) in (&*entities, &spirits, &player_spirits).join() {
                player_entity = Some(entity);
            }
            if let Some(player_entity) = player_entity {
                let mut attacking_spirits = Vec::new();
                for (spirit, ()) in (&spirits, !&player_spirits).join() {
                    if spirit.health > 0 {
                        attacking_spirits.push(spirit.clone());
                    }
                }
                for spirit in attacking_spirits.iter() {
                    act(spirit, &player_entity, &mut spirits);
                }
                battle_state.enemy_attacking = false;
            }
        }
    }
}
