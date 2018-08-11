use ggez::*;
use specs::*;
use specs::world::EntitiesRes;
use ggez::graphics::*;
use ggez::graphics::spritebatch::*;
use state::*;
use render::*;

const BAR_SIZE: (i32, i32) = (128, 32);
const INNER_BAR_SIZE: (i32, i32) = (54, 6);

const ALLY_COUNT: usize = 1;
const ALLY_BARS: [(i32, i32); ALLY_COUNT] = [
(
    SCREEN_SIZE.0 as i32 - BAR_SIZE.0 - 16,
    SCREEN_SIZE.1 as i32 - BAR_SIZE.1 - 64 - 16
),
];
const ALLY_BAR_INNER_OFFSET: (i32, i32) = (62, 16);
const ALLY_BAR_AREA: (i32, i32, i32, i32) = (
    SCREEN_SIZE.0 as i32 - BAR_SIZE.0 - 16,
    SCREEN_SIZE.1 as i32 - 72 - 64 - BAR_SIZE.1 * 4,
    16 + BAR_SIZE.0,
    64 + BAR_SIZE.1 * 4,
);

const ENEMY_COUNT: usize = 3;
const ENEMY_BARS: [(i32, i32); ENEMY_COUNT] = [
    (8, 8),
    (8, BAR_SIZE.1 + 24),
    (8, BAR_SIZE.1 + BAR_SIZE.1 + 40),
];
const ENEMY_BAR_AREA: (i32, i32, i32, i32) = (
    0,
    0,
    16 + BAR_SIZE.0,
    64 + BAR_SIZE.1 * 4,
);
const ENEMY_BAR_INNER_OFFSET: (i32, i32) = (34, 16);
const MOVE_AREAS: [(f32, f32); 4] = [
    (SCREEN_SIZE.0 as f32 - 256.0, SCREEN_SIZE.1 as f32 - 64.0),
    (SCREEN_SIZE.0 as f32 - 128.0, SCREEN_SIZE.1 as f32 - 64.0),
    (SCREEN_SIZE.0 as f32 - 256.0, SCREEN_SIZE.1 as f32 - 32.0),
    (SCREEN_SIZE.0 as f32 - 128.0, SCREEN_SIZE.1 as f32 - 32.0),
];

fn render_health_bar(ctx: &mut Context, location: (i32, i32), percent: f32) -> GameResult<()> {
    set_color(ctx, [1.0, 0.0, 0.0, 1.0].into())?;
    rectangle(ctx, DrawMode::Fill, Rect::new_i32(
        location.0,
        location.1,
        (INNER_BAR_SIZE.0 as f32 * percent) as i32,
        INNER_BAR_SIZE.1,
    ))?;
    Ok(())
}

pub fn render_combat(ctx: &mut Context, world: &World) -> GameResult<()> {
    let battle_state = world.read_resource::<BattleState>();
    let entities = world.read_resource::<EntitiesRes>();
    let spirits = world.read_storage::<Spirit>();
    let player_spirits = world.read_storage::<PlayerSpirit>();
    let mut spritebatch = world.write_resource::<SpriteBatch>();
    let mut enemy_count = 0;
    set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;
    rectangle(ctx, DrawMode::Fill, Rect::new_i32(
        ENEMY_BAR_AREA.0,
        ENEMY_BAR_AREA.1,
        ENEMY_BAR_AREA.2,
        ENEMY_BAR_AREA.3,
    ))?;
    for (entity, enemy, ()) in (&*entities, &spirits, !&player_spirits).join() {
        if enemy_count < ENEMY_COUNT && enemy.health > 0 {
            let enemy_bar = ENEMY_BARS[enemy_count];
            render_health_bar(ctx, (
                enemy_bar.0 + ENEMY_BAR_INNER_OFFSET.0,
                enemy_bar.1 + ENEMY_BAR_INNER_OFFSET.1,
            ), enemy.health as f32 / enemy.max_health as f32)?;
            spritebatch.add(enemy_bar_sprite(enemy_bar.0, enemy_bar.1, BAR_SIZE.0, BAR_SIZE.1));
            enemy_count += 1;
        }
    }
    set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;
    rectangle(ctx, DrawMode::Fill, Rect::new_i32(
        ALLY_BAR_AREA.0,
        ALLY_BAR_AREA.1,
        ALLY_BAR_AREA.2,
        ALLY_BAR_AREA.3,
    ))?;
    let mut ally_count = 0;
    for (entity, ally, player) in (&*entities, &spirits, &player_spirits).join() {
        if player.active {
            let ally_bar = ALLY_BARS[ally_count];
            render_health_bar(ctx, (
                ally_bar.0 + ALLY_BAR_INNER_OFFSET.0,
                ally_bar.1 + ALLY_BAR_INNER_OFFSET.1,
            ), ally.health as f32 / ally.max_health as f32)?;
            spritebatch.add(ally_bar_sprite(ally_bar.0, ally_bar.1, BAR_SIZE.0, BAR_SIZE.1));
            ally_count += 1;
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
