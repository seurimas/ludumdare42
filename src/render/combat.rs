use ggez::*;
use specs::*;
use ggez::graphics::*;
use state::*;

const ENEMY_BARS: (i32, i32) = (400, 100);
const BAR_SIZE: (i32, i32) = (96, 32);

pub fn render_combat(ctx: &mut Context, world: &World) -> GameResult<()> {
    let battle_state = world.read_resource::<BattleState>();
    let enemies = &battle_state.enemies;
    let mut y_offset = 0;
    for enemy in enemies.iter() {
        set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;
        rectangle(ctx, DrawMode::Line(8.), Rect::new_i32(
            ENEMY_BARS.0,
            ENEMY_BARS.1 + y_offset,
            BAR_SIZE.0,
            BAR_SIZE.1,
        ))?;
        set_color(ctx, [0.0, 1.0, 0.0, 1.0].into())?;
        rectangle(ctx, DrawMode::Fill, Rect::new_i32(
            ENEMY_BARS.0,
            ENEMY_BARS.1 + y_offset,
            BAR_SIZE.0 * (enemy.health / enemy.max_health) as i32,
            BAR_SIZE.1,
        ))?;
        y_offset += BAR_SIZE.1;
    }
    Ok(())
}
