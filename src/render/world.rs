use ggez::*;
use specs::*;
use ggez::graphics::*;
use state::*;

pub fn render_in_world(ctx: &mut Context, world: &World) -> GameResult<()> {
    let camera = world.read_resource::<Camera>();
    let level = world.read_resource::<Level>();
    set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;
    for x in 0..camera.width{
        for y in 0..camera.height {
            if level.has_tile((camera.x_offset + x, camera.y_offset + y)) {
                rectangle(ctx, DrawMode::Fill, Rect::new_i32(
                    (x * TILE_SIZE) as i32,
                    (y * TILE_SIZE) as i32,
                    TILE_SIZE as i32,
                    TILE_SIZE as i32,
                ))?;
            }
        }
    }
    let positions = world.read_storage::<WorldEntity>();
    set_color(ctx, [1.0, 0.0, 0.0, 1.0].into())?;
    for position in (&positions).join() {
        let OFFSET = (TILE_SIZE - CHAR_SIZE) / 2;
        let (pos_x, pos_y) = (position.location.0 - camera.x_offset, position.location.1 - camera.y_offset);
        rectangle(ctx, DrawMode::Fill, Rect::new_i32(
            (pos_x * TILE_SIZE + OFFSET) as i32,
            (pos_y * TILE_SIZE + OFFSET) as i32,
            (CHAR_SIZE) as i32,
            (CHAR_SIZE) as i32,
        ))?;
    }
    Ok(())
}
