use ggez::*;
use specs::*;
use ggez::graphics::*;
use ggez::graphics::spritebatch::*;
use state::*;
use render::*;

const SPIRIT_SIZE: (i32, i32) = (64, 64);
const SPIRIT_LOCATION: (i32, i32) = (8, 8);
const SPIRIT_BUFFER: (i32, i32) = (8, 8);
const INVENTORY_LAYOUT: (i32, i32) = (5, 5);
const DESCRIPTION_AREA: (i32, i32) = (368, 8);
const DESCRIPTION_SIZE: (i32, i32) = (256, 280);
const NAME_OFFSET: (f32, f32) = (8.0, 8.0);
const ELEMENT_OFFSET: (f32, f32) = (8.0, 24.0);
const HEALTH_OFFSET: (f32, f32) = (8.0, 40.0);
const COLLIDE_OFFSET: (f32, f32) = (8.0, 72.0);
const MOVES_OFFSETS: [(f32, f32); 4] = [
    (8.0, 136.0),
    (148.0, 136.0),
    (8.0, 200.0),
    (148.0, 200.0),
];

pub fn render_combining(ctx: &mut Context, world: &mut World) -> GameResult<()> {
    type SystemData<'a> = (
        ReadStorage<'a, Player>,
        ReadExpect<'a, InventoryState>,
        WriteExpect<'a, SpriteBatch>,
    );
    world.exec(|(players, inventory_state, mut spritebatch): SystemData| -> GameResult<()> {
        let font = Font::default_font()?;
        for player in (&players).join() {
            for y in 0..INVENTORY_LAYOUT.0 {
                for x in 0..INVENTORY_LAYOUT.1 {
                    let index = x + y * INVENTORY_LAYOUT.0;
                    if index == inventory_state.index as i32 {
                        set_color(ctx, [0.0, 0.0, 1.0, 1.0].into())?;
                    } else {
                        set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;
                    }
                    rectangle(ctx, DrawMode::Line(4.0), Rect::new_i32(
                        SPIRIT_LOCATION.0 + (SPIRIT_SIZE.0 + SPIRIT_BUFFER.0) * x,
                        SPIRIT_LOCATION.1 + (SPIRIT_SIZE.1 + SPIRIT_BUFFER.1) * y,
                        SPIRIT_SIZE.0,
                        SPIRIT_SIZE.1,
                    ))?;
                    if let Some(spirit) = player.spirits.get(index as usize) {
                        set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;
                        spritebatch.add(spirit_sprite(&spirit.element,
                            SPIRIT_LOCATION.0 + (SPIRIT_SIZE.0 + SPIRIT_BUFFER.0) * x,
                            SPIRIT_LOCATION.1 + (SPIRIT_SIZE.1 + SPIRIT_BUFFER.1) * y,
                            SPIRIT_SIZE.0,
                            SPIRIT_SIZE.1,
                            None,
                        ));
                        set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;
                        rectangle(ctx, DrawMode::Line(4.0), Rect::new_i32(
                            DESCRIPTION_AREA.0,
                            DESCRIPTION_AREA.1,
                            DESCRIPTION_SIZE.0,
                            DESCRIPTION_SIZE.1,
                        ))?;
                        if index == inventory_state.index as i32 {
                            set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;
                            let text = Text::new(
                                ctx,
                                &spirit_name(&spirit.element),
                                &font,
                            )?;
                            draw(ctx, &text, Point2::new(
                                DESCRIPTION_AREA.0 as f32 + NAME_OFFSET.0,
                                DESCRIPTION_AREA.1 as f32 + NAME_OFFSET.1,
                            ), 0.0)?;
                            let text = Text::new(
                                ctx,
                                &spirit_level_text(&spirit.element),
                                &font,
                            )?;
                            draw(ctx, &text, Point2::new(
                                DESCRIPTION_AREA.0 as f32 + ELEMENT_OFFSET.0,
                                DESCRIPTION_AREA.1 as f32 + ELEMENT_OFFSET.1,
                            ), 0.0)?;
                            let text = Text::new(
                                ctx,
                                &format!("Health: {}", health(&spirit)),
                                &font,
                            )?;
                            draw(ctx, &text, Point2::new(
                                DESCRIPTION_AREA.0 as f32 + HEALTH_OFFSET.0,
                                DESCRIPTION_AREA.1 as f32 + HEALTH_OFFSET.1,
                            ), 0.0)?;
                            text_in_box(ctx, &collide_text(&spirit.element), (
                                DESCRIPTION_AREA.0 + COLLIDE_OFFSET.0 as i32,
                                DESCRIPTION_AREA.1 + COLLIDE_OFFSET.1 as i32,
                                DESCRIPTION_SIZE.0,
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    })?;
    Ok(())
}
