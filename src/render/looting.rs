use ggez::*;
use specs::*;
use ggez::graphics::*;
use ggez::graphics::spritebatch::*;
use state::*;
use render::*;

const SPRITE_SIZE: (i32, i32) = (
    64, 64,
);

const LOOTED_AREA: (f32, f32, f32, f32) = (
    0.0, 0.0, SCREEN_SIZE.0 as f32, ((SCREEN_SIZE.1 as f32) - 32.0) / 2.0,
);
const LOOTED_HEADER: (f32, f32) = (
    8.0, 8.0,
);
const LOOTED_REGION: (f32, f32, f32, f32) = (
    8.0, 32.0,
    LOOTED_AREA.2 - 16.0, LOOTED_AREA.3 - 40.0,
);

const LOST_AREA: (f32, f32, f32, f32) = (
    0.0, LOOTED_AREA.3, SCREEN_SIZE.0 as f32, ((SCREEN_SIZE.1 as f32) - 32.0) / 2.0,
);
const LOST_HEADER: (f32, f32) = (
    8.0, LOST_AREA.1 + 8.0,
);
const LOST_REGION: (f32, f32, f32, f32) = (
    8.0, LOST_AREA.1 + 32.0,
    LOOTED_REGION.2, LOOTED_REGION.3,
);

pub fn render_spread(spritebatch: &mut SpriteBatch, spirits: &Vec<SpiritType>, region: (f32, f32, f32, f32)) -> GameResult<()> {
    let mut x = region.0;
    let mut y = region.1;
    for spirit in spirits.iter() {
        spritebatch.add(spirit_sprite(&spirit,
            x as i32,
            y as i32,
            SPRITE_SIZE.0,
            SPRITE_SIZE.1,
            None,
        ));
        x += (SPRITE_SIZE.0 + 16) as f32;
        if x + (SPRITE_SIZE.0 + 16) as f32 > region.0 + region.2 {
            y += (SPRITE_SIZE.1 + 16) as f32;
            x = region.0;
        }
    }
    Ok(())
}

pub fn render_looting(ctx: &mut Context, world: &mut World, captured: &Vec<SpiritType>, lost: &Vec<SpiritType>) -> GameResult<()> {
    type SystemData<'a> = (
        Entities<'a>,
        WriteExpect<'a, SpriteBatch>,
    );
    let font = Font::default_font()?;
    world.exec(|(entities, mut spritebatch): SystemData| -> GameResult<()> {
        set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;
        let text = Text::new(
            ctx,
            &"Captured Spirits",
            &font,
        )?;
        draw(ctx, &text, Point2::new(
            LOOTED_HEADER.0,
            LOOTED_HEADER.1,
        ), 0.0)?;
        render_spread(&mut spritebatch, captured, LOOTED_REGION)?;
        let text = Text::new(
            ctx,
            &"Lost Spirits",
            &font,
        )?;
        draw(ctx, &text, Point2::new(
            LOST_HEADER.0,
            LOST_HEADER.1,
        ), 0.0)?;
        render_spread(&mut spritebatch, lost, LOST_REGION)?;
        Ok(())
    })
}
