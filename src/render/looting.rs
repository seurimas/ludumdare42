use ggez::*;
use specs::*;
use ggez::graphics::*;
use state::*;
use render::*;

pub fn render_looting(ctx: &mut Context, world: &mut World, spirits: &Vec<SpiritType>) -> GameResult<()> {
    type SystemData<'a> = (
    );
    world.exec(|(): SystemData| -> GameResult<()> {
        Ok(())
    })
}
