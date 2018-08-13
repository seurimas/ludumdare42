use ggez::*;
use specs::*;
use ggez::graphics::*;
use ggez::graphics::spritebatch::*;
use state::*;
use render::*;

const OFFSET: u32 = (TILE_SIZE - CHAR_SIZE) / 2;

pub fn render_in_world(ctx: &mut Context, world: &World) -> GameResult<()> {
    let camera = world.read_resource::<Camera>();
    let level = world.read_resource::<Level>();
    let mut spritebatch = world.write_resource::<SpriteBatch>();
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
    let players = world.read_storage::<Player>();
    let stairs = world.read_storage::<Stair>();
    let encounters = world.read_storage::<Encounter>();
    for (encounter, position) in (&encounters, &positions).join() {
        if position.location.0 >= camera.x_offset
            && position.location.1 >= camera.y_offset
            && position.location.0 < camera.x_offset + camera.width
            && position.location.1 < camera.y_offset + camera.height {
            let (pos_x, pos_y) = (
                position.location.0 - camera.x_offset,
                position.location.1 - camera.y_offset,
            );
            if let Some(head) = encounter.spirits.first() {
                spritebatch.add(spirit_sprite(&head.element,
                    (pos_x * TILE_SIZE + OFFSET) as i32,
                    (pos_y * TILE_SIZE + OFFSET) as i32,
                    (CHAR_SIZE) as i32,
                    (CHAR_SIZE) as i32,
                    None,
                ));
            }
        }
    }
    for (player, position) in (&players, &positions).join() {
        if position.location.0 >= camera.x_offset
            && position.location.1 >= camera.y_offset
            && position.location.0 < camera.x_offset + camera.width
            && position.location.1 < camera.y_offset + camera.height {
            let (pos_x, pos_y) = (
                position.location.0 - camera.x_offset,
                position.location.1 - camera.y_offset,
            );
            if let Some(head) = player.spirits.first() {
                spritebatch.add(spirit_sprite(&head.element,
                    (pos_x * TILE_SIZE + OFFSET) as i32,
                    (pos_y * TILE_SIZE + OFFSET) as i32,
                    (CHAR_SIZE) as i32,
                    (CHAR_SIZE) as i32,
                    None,
                ));
            }
        }
    }
    for (_stair, position) in (&stairs, &positions).join() {
        if position.location.0 >= camera.x_offset
            && position.location.1 >= camera.y_offset
            && position.location.0 < camera.x_offset + camera.width
            && position.location.1 < camera.y_offset + camera.height {
            let (pos_x, pos_y) = (
                position.location.0 - camera.x_offset,
                position.location.1 - camera.y_offset,
            );
            spritebatch.add(spirit_sprite(&SpiritType::Fire(4),
                (pos_x * TILE_SIZE + OFFSET) as i32,
                (pos_y * TILE_SIZE + OFFSET) as i32,
                (CHAR_SIZE) as i32,
                (CHAR_SIZE) as i32,
                None,
            ));
        }
    }
    Ok(())
}
