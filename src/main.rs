extern crate specs;
extern crate ggez;
#[macro_use]
extern crate specs_derive;

mod render;
mod state;
mod input;
use specs::*;
use ggez::*;
use ggez::event::*;
use render::render_world;
use state::*;
use input::*;

const SCREEN_SIZE: (u32, u32) = (800, 600);

pub struct CameraSystem;
impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        Write<'a, Camera>,
        ReadStorage<'a, WorldEntity>,
    );

    fn run(&mut self, (mut camera, entities): Self::SystemData) {
        for entity in (&entities).join() {
            if entity.entity_type == 0 {
                if entity.location.0 >= camera.width / 2 {
                    let x_offset = entity.location.0 - camera.width / 2;
                    camera.x_offset = x_offset;
                } else {
                    camera.x_offset = 0;
                }
                if entity.location.1 >= camera.height / 2 {
                    let y_offset = entity.location.1 - camera.height / 2;
                    camera.y_offset = y_offset;
                } else {
                    camera.y_offset = 0;
                }
            }
        }
    }
}

impl<'a, 'b> GameState<'a, 'b> {
    fn new(ctx: &mut Context) -> Self {
        let mut world = World::new();
        world.register::<WorldEntity>();
        world.add_resource(Level::new());
        world.add_resource(Camera::new(SCREEN_SIZE.0, SCREEN_SIZE.1));

        world.create_entity()
            .with(WorldEntity { location: (1, 1), entity_type: 0 })
            .build();

        let dispatcher = DispatcherBuilder::new()
            .with(CameraSystem, "camera", &[])
            .build();

        let play_state = PlayState::InWorld;

        GameState {
            dispatcher,
            world,
            play_state,
        }
    }

    fn init(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}

impl<'a, 'b> EventHandler for GameState<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        render_world(ctx, &self.world, &self.play_state)?;
        graphics::present(ctx);
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: Keycode,
        _keymod: event::Mod,
        repeat: bool,
    ) {
        if !repeat {
            match keycode {
                Keycode::W => {
                    handle_arrow(ctx, &mut self.world, &self.play_state, Direction::Up);
                },
                Keycode::A => {
                    handle_arrow(ctx, &mut self.world, &self.play_state, Direction::Left);
                },
                Keycode::S => {
                    handle_arrow(ctx, &mut self.world, &self.play_state, Direction::Down);
                },
                Keycode::D => {
                    handle_arrow(ctx, &mut self.world, &self.play_state, Direction::Right);
                },
                _ => {

                }
            }
        }
    }
}

fn main() {
    let ctx = &mut ggez::ContextBuilder::new("Spirits of Semb", "Seurimas")
        .window_setup(ggez::conf::WindowSetup::default().title("Spirits of Semb"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build().expect("Failed to build ggez context");

    graphics::set_background_color(ctx, [0.0, 0.0, 0.0, 1.0].into());

    let state = &mut GameState::new(ctx);
    state.init(ctx).expect("Failed to load resources");

    match event::run(ctx, state) {
        Err(e) => println!("Error encountered running game: {}", e),
        Ok(_) => {},
    }
}
