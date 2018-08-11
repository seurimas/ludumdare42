use ggez::*;
use specs::*;
use specs::world::EntitiesRes;
use ggez::graphics::*;
use state::*;

const ALLY_BARS: (i32, i32) = (100, 100);
const ENEMY_BARS: (i32, i32) = (400, 100);
const BAR_SIZE: (i32, i32) = (96, 32);
const BAR_OFFSET: i32 = 8;
const MOVE_AREAS: [(f32, f32); 4] = [
    (200.0, 520.0),
    (300.0, 520.0),
    (200.0, 560.0),
    (300.0, 560.0),
];

fn render_health_bar(ctx: &mut Context, location: (i32, i32), percent: f32) -> GameResult<()> {
    set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;
    rectangle(ctx, DrawMode::Line(8.), Rect::new_i32(
        location.0,
        location.1,
        BAR_SIZE.0,
        BAR_SIZE.1,
    ))?;
    set_color(ctx, [1.0, 0.0, 0.0, 1.0].into())?;
    rectangle(ctx, DrawMode::Fill, Rect::new_i32(
        location.0,
        location.1,
        (BAR_SIZE.0 as f32 * percent) as i32,
        BAR_SIZE.1,
    ))?;
    Ok(())
}

pub fn render_combat(ctx: &mut Context, world: &World) -> GameResult<()> {
    let battle_state = world.read_resource::<BattleState>();
    let entities = world.read_resource::<EntitiesRes>();
    let spirits = world.read_storage::<Spirit>();
    let player_spirits = world.read_storage::<PlayerSpirit>();
    let mut y_offset = 0;
    for (entity, enemy, ()) in (&*entities, &spirits, !&player_spirits).join() {
        render_health_bar(ctx, (
            ENEMY_BARS.0,
            ENEMY_BARS.1 + y_offset,
        ), enemy.health as f32 / enemy.max_health as f32)?;
        y_offset += BAR_SIZE.1 + BAR_OFFSET;
    }
    let mut y_offset = 0;
    for (entity, ally, player) in (&*entities, &spirits, &player_spirits).join() {
        if player.active {
            render_health_bar(ctx, (
                ALLY_BARS.0,
                ALLY_BARS.1 + y_offset,
            ), ally.health as f32 / ally.max_health as f32)?;
            y_offset += BAR_SIZE.1 + BAR_OFFSET;
        }
    }
    let font = Font::default_font()?;
    match (battle_state.active_entity, battle_state.combat_move) {
        (Some(entity), Some(index)) => {
            match spirits.get(entity) {
                Some(spirit) => {
                    for move_index in 0..4 {
                        let combat_move = &spirit.moves[move_index];
                        let text = &Text::new(ctx, &combat_move.name, &font)?;
                        if index == move_index {
                            set_color(ctx, [0.0, 0.0, 1.0, 1.0].into())?;
                        } else {
                            set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;
                        }
                        draw(
                            ctx,
                            text,
                            Point2::new(MOVE_AREAS[move_index].0, MOVE_AREAS[move_index].1),
                            0.0,
                        )?;
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    Ok(())
}
