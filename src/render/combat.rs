use ggez::*;
use specs::*;
use specs::world::EntitiesRes;
use ggez::graphics::*;
use ggez::graphics::spritebatch::*;
use state::*;
use render::*;

const SPRITE_SIZE: (i32, i32) = (64, 64);
const BAR_SIZE: (i32, i32) = (128, 32);
const INNER_BAR_SIZE: (i32, i32) = (74, 8);
const INNER_DEF_BAR_SIZE: (i32, i32) = (34, 4);

const ALLY_COUNT: usize = 1;
const ALLY_BARS: [(i32, i32); ALLY_COUNT] = [
(
    SCREEN_SIZE.0 as i32 - BAR_SIZE.0 - 16,
    SCREEN_SIZE.1 as i32 - BAR_SIZE.1 - 64 - 16
),
];
const ALLY_LOCATION: (i32, i32) = (
    (SCREEN_SIZE.0 / 2) as i32 - 32,
    SCREEN_SIZE.1 as i32 - 128,
);
const ALLY_BAR_INNER_OFFSET: (i32, i32) = (44, 6);
const ALLY_DEF_BAR_INNER_OFFSET: (i32, i32) = (44, 18);
const ALLY_BAR_AREA: (i32, i32, i32, i32) = (
    SCREEN_SIZE.0 as i32 - BAR_SIZE.0 - 16,
    SCREEN_SIZE.1 as i32 - 120,
    16 + BAR_SIZE.0,
    16 + BAR_SIZE.1,
);

const ENEMY_COUNT: usize = 3;
const LINEUP_COUNT: usize = 8;
const ENEMY_LOCATIONS: [(i32, i32); ENEMY_COUNT] = [
    ((SCREEN_SIZE.0 / 2) as i32 - 64, 8),
    ((SCREEN_SIZE.0 / 2) as i32, 8),
    ((SCREEN_SIZE.0 / 2) as i32 - 32, 80),
];
const ENEMY_BARS: [(i32, i32); ENEMY_COUNT] = [
    (ENEMY_LOCATIONS[0].0 - BAR_SIZE.0 - 8, 8),
    (ENEMY_LOCATIONS[1].0 + 64 + 8, 8),
    ((SCREEN_SIZE.0 / 2) as i32 - BAR_SIZE.0 / 2, 152),
];
const ENEMY_BAR_AREA: (i32, i32, i32, i32) = (
    0,
    0,
    16 + BAR_SIZE.0,
    48 + BAR_SIZE.1 * 3,
);
const ENEMY_LINEUP: (i32, i32, i32, i32) = (
    8, 56 + BAR_SIZE.1 * 3,
    128, 64,
);
const ENEMY_LINEUP_SIZE: (i32, i32) = (
    128 / 4, 64 / 2,
);
const ENEMY_BAR_INNER_OFFSET: (i32, i32) = (10, 6);
const ENEMY_DEF_BAR_INNER_OFFSET: (i32, i32) = (10, 18);
const NOTIFICATION_AREA: (i32, i32, i32, i32) = (
    0, SCREEN_SIZE.1 as i32 - 64,
    SCREEN_SIZE.0 as i32 - 256, 64,
);
const MOVE_AREAS: [(f32, f32); 4] = [
    (SCREEN_SIZE.0 as f32 - 248.0, SCREEN_SIZE.1 as f32 - 56.0),
    (SCREEN_SIZE.0 as f32 - 124.0, SCREEN_SIZE.1 as f32 - 56.0),
    (SCREEN_SIZE.0 as f32 - 248.0, SCREEN_SIZE.1 as f32 - 28.0),
    (SCREEN_SIZE.0 as f32 - 124.0, SCREEN_SIZE.1 as f32 - 28.0),
];
const MOVE_REGION: (i32, i32, i32, i32) = (
    SCREEN_SIZE.0 as i32 - 256,  SCREEN_SIZE.1 as i32 - 64,
    256, 64,
);

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

fn render_defense_bar(ctx: &mut Context, location: (i32, i32), percent: f32) -> GameResult<()> {
    set_color(ctx, [0.0, 0.0, 1.0, 1.0].into())?;
    rectangle(ctx, DrawMode::Fill, Rect::new_i32(
        location.0,
        location.1,
        (INNER_DEF_BAR_SIZE.0 as f32 * percent) as i32,
        INNER_DEF_BAR_SIZE.1,
    ))?;
    Ok(())
}

pub fn render_combat(ctx: &mut Context, world: &World) -> GameResult<()> {
    let battle_state = world.read_resource::<BattleState>();
    let entities = world.read_resource::<EntitiesRes>();
    let spirits = world.write_storage::<Spirit>();
    let player_spirits = world.read_storage::<PlayerSpirit>();
    let mut spritebatch = world.write_resource::<SpriteBatch>();
    let mut enemy_count = 0;
    let mut lineup_count = 0;
    set_color(ctx, [0.7, 0.7, 0.8, 1.0].into())?;
    rectangle(ctx, DrawMode::Fill, Rect::new_i32(
        0, 0,
        SCREEN_SIZE.0 as i32, SCREEN_SIZE.1 as i32,
    ))?;
    for (entity, enemy, ()) in (&*entities, &spirits, !&player_spirits).join() {
        if enemy_count < ENEMY_COUNT && enemy.health > 0 {
            let enemy_bar = ENEMY_BARS[enemy_count];
            let enemy_location = ENEMY_LOCATIONS[enemy_count];
            render_health_bar(ctx, (
                enemy_bar.0 + ENEMY_BAR_INNER_OFFSET.0,
                enemy_bar.1 + ENEMY_BAR_INNER_OFFSET.1,
            ), enemy.health as f32 / enemy.max_health as f32)?;
            render_defense_bar(ctx, (
                enemy_bar.0 + ENEMY_DEF_BAR_INNER_OFFSET.0,
                enemy_bar.1 + ENEMY_DEF_BAR_INNER_OFFSET.1,
            ), enemy.defense as f32 / 6.0)?;
            spritebatch.add(enemy_bar_sprite(enemy_bar.0, enemy_bar.1, BAR_SIZE.0, BAR_SIZE.1));
            spritebatch.add(battle_spirit_background(
                enemy_location.0,
                enemy_location.1,
                SPRITE_SIZE.0,
                SPRITE_SIZE.1,
                None,
            ));
            spritebatch.add(spirit_sprite(
                &enemy.element,
                enemy_location.0,
                enemy_location.1,
                SPRITE_SIZE.0,
                SPRITE_SIZE.1,
                None,
            ));
            enemy_count += 1;
        } else if lineup_count < LINEUP_COUNT {
            let x_index = lineup_count % 4;
            let y_index = lineup_count / 4;
            let x_offset = x_index as i32 * ENEMY_LINEUP_SIZE.0;
            let y_offset = y_index as i32 * ENEMY_LINEUP_SIZE.1;
            let color: Option<Color> = match enemy.health {
                0 => Some([0.5, 0.5, 0.5, 1.0].into()),
                _ => None,
            };
            spritebatch.add(spirit_sprite(&enemy.element,
                ENEMY_LINEUP.0 + x_offset + 4, ENEMY_LINEUP.1 + y_offset + 4,
                ENEMY_LINEUP_SIZE.0 - 8, ENEMY_LINEUP_SIZE.1 - 8,
                color,
            ));
            lineup_count += 1;
        }
    }
    let mut ally_count = 0;
    for (entity, ally, player) in (&*entities, &spirits, &player_spirits).join() {
        if player.active {
            let ally_bar = ALLY_BARS[ally_count];
            render_health_bar(ctx, (
                ally_bar.0 + ALLY_BAR_INNER_OFFSET.0,
                ally_bar.1 + ALLY_BAR_INNER_OFFSET.1,
            ), ally.health as f32 / ally.max_health as f32)?;
            render_defense_bar(ctx, (
                ally_bar.0 + ALLY_DEF_BAR_INNER_OFFSET.0,
                ally_bar.1 + ALLY_DEF_BAR_INNER_OFFSET.1,
            ), ally.defense as f32 / 6.0)?;
            spritebatch.add(ally_bar_sprite(ally_bar.0, ally_bar.1, BAR_SIZE.0, BAR_SIZE.1));
            spritebatch.add(battle_spirit_background(
                ALLY_LOCATION.0,
                ALLY_LOCATION.1,
                SPRITE_SIZE.0,
                SPRITE_SIZE.1,
                None,
            ));
            spritebatch.add(spirit_sprite(
                &ally.element,
                ALLY_LOCATION.0,
                ALLY_LOCATION.1,
                SPRITE_SIZE.0,
                SPRITE_SIZE.1,
                None,
            ));
            ally_count += 1;
        }
    }
    let font = Font::default_font()?;
    match (battle_state.active_entity, battle_state.combat_move) {
        (Some(entity), Some(index)) => {
            match spirits.get(entity) {
                Some(spirit) => {
                    text_outline(ctx, MOVE_REGION)?;
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
    text_outline(ctx, NOTIFICATION_AREA)?;
    let text_area = (
        NOTIFICATION_AREA.0 + 8,
        NOTIFICATION_AREA.1 + 8,
        NOTIFICATION_AREA.2 - 16,
    );
    if let Some(ref notification) = battle_state.notification {
        text_in_box(ctx, notification, text_area)?;
    } else if let Some(combat_move) = battle_state.get_move(&spirits) {
        let text = &move_text(&combat_move);
        text_in_box(ctx, text, text_area)?;
    }
    Ok(())
}
