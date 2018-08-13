extern crate specs;
extern crate ggez;
#[macro_use]
extern crate specs_derive;
extern crate rand;

mod render;
mod state;
mod input;
mod systems;
use specs::*;
use ggez::*;
use ggez::audio::*;
use ggez::graphics::*;
use ggez::graphics::spritebatch::*;
use ggez::event::*;
use render::render_world;
use state::*;
use input::*;
use systems::*;
use std::path;
use std::path::Path;
use std::collections::*;
use std::time::Duration;

pub struct CameraSystem;
impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        Write<'a, Camera>,
        ReadStorage<'a, WorldEntity>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut camera, entities, players): Self::SystemData) {
        for (entity, player) in (&entities, &players).join() {
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

impl<'a, 'b> GameState<'a, 'b> {
    fn new(ctx: &mut Context) -> Self {
        let mut world = World::new();
        world.register::<WorldEntity>();
        world.register::<Encounter>();
        world.register::<Spirit>();
        world.register::<PlayerSpirit>();
        world.register::<Player>();
        world.register::<CombatEffects>();
        world.register::<Stair>();
        world.add_resource(Camera::new(SCREEN_SIZE.0, SCREEN_SIZE.1));
        world.add_resource(BattleState::new());
        world.add_resource(PlayState::MainMenu(0));
        world.add_resource(InputState::Rest);
        world.add_resource(InventoryState::new());
        world.add_resource(Duration::new(0, 0));
        world.add_resource(Level::new(0));

        let dispatcher = DispatcherBuilder::new()
            .with(HandleMove, "move", &[])
            .with(HandleBattleMenu, "battle_menu", &[])
            .with(HandleInventory, "inventory", &[])
            .with(HandleLootMenu, "looting", &[])
            .with(HandleMainMenu, "main_menu", &[])
            .with(CameraSystem, "camera", &[])
            .with(FindEncounters, "find", &[])
            .with(WanderEncounters, "wander", &[])
            .with(WatchAttack, "attack", &[])
            .with(WatchSpirits, "spirits", &[])
            .with(TickEffects, "tick_combat", &["attack"])
            .with(EnemyCombat, "enemy_attack", &["tick_combat"])
            .build();

        GameState {
            dispatcher,
            world,
        }
    }

    fn sound<P: AsRef<path::Path>>(context: &mut Context, path: P) -> GameResult<Source> {
        let mut sound = Source::new(context, path)?;
        sound.set_repeat(false);
        sound.set_volume(100.0);
        Ok(sound)
    }

    fn init(&mut self, ctx: &mut Context) -> GameResult<()> {
        let image = Image::new(ctx, &"/Sprites.png")?;
        let fire = GameState::sound(ctx, &"/fire_attack.wav")?;
        let mut water = GameState::sound(ctx, &"/water_attack.wav")?;
        let mut slime = GameState::sound(ctx, &"/slime_attack.wav")?;
        let mut light = GameState::sound(ctx, &"/light_attack.wav")?;
        let mut dark = GameState::sound(ctx, &"/dark_attack.wav")?;
        let mut blip = GameState::sound(ctx, &"/blip.wav")?;
        let mut cancel = GameState::sound(ctx, &"/cancel.wav")?;
        let mut collide = GameState::sound(ctx, &"/collide.wav")?;
        let mut confirm = GameState::sound(ctx, &"/confirm.wav")?;
        let mut encounter = GameState::sound(ctx, &"/encounter.wav")?;
        let mut lose = GameState::sound(ctx, &"/lose.wav")?;
        self.world.add_resource(SpriteBatch::new(image));
        self.world.add_resource(Sounds {
            fire,
            water,
            slime,
            light,
            dark,
            blip,
            cancel,
            collide,
            confirm,
            encounter,
            lose,
            pending: Vec::new(),
        });
        Ok(())
    }
    fn wants_level(&self) -> Option<u32> {
        if let PlayState::Stairs(depth) = *self.world.read_resource::<PlayState>() {
            Some(depth)
        } else {
            None
        }
    }
}

impl<'a, 'b> EventHandler for GameState<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.world.add_resource(ggez::timer::get_delta(&ctx));
        self.dispatcher.dispatch(&mut self.world.res);
        if let Some(depth) = self.wants_level() {
            let level = Level::new(depth);
            level.spawn_encounters(&mut self.world);
            self.world.add_resource(level);
            self.world.add_resource(PlayState::InWorld);
        }
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.world.write_resource::<SpriteBatch>().clear();
        render_world(ctx, &mut self.world)?;
        graphics::set_color(ctx, [1.0, 1.0, 1.0, 1.0].into())?;
        graphics::draw(
            ctx,
            &*self.world.read_resource::<SpriteBatch>(),
            Point2::new(0.0, 0.0),
            0.0,
        )?;
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
                    self.world.add_resource(InputState::Move(Direction::Up));
                },
                Keycode::A => {
                    self.world.add_resource(InputState::Move(Direction::Left));
                },
                Keycode::S => {
                    self.world.add_resource(InputState::Move(Direction::Down));
                },
                Keycode::D => {
                    self.world.add_resource(InputState::Move(Direction::Right));
                },
                Keycode::Space => {
                    self.world.add_resource(InputState::Select);
                },
                Keycode::Backspace => {
                    self.world.add_resource(InputState::Escape);
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
    graphics::set_default_filter(ctx, FilterMode::Nearest);

    let state = &mut GameState::new(ctx);
    state.init(ctx).expect("Failed to load resources");

    match event::run(ctx, state) {
        Err(e) => println!("Error encountered running game: {}", e),
        Ok(_) => {},
    }
}
