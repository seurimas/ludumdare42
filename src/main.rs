extern crate specs;
extern crate ggez;
#[macro_use]
extern crate specs_derive;

mod render;
mod state;
mod input;
mod systems;
use specs::*;
use ggez::*;
use ggez::event::*;
use render::render_world;
use state::*;
use input::*;
use systems::*;
use std::collections::*;

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
        world.add_resource(Level::new());
        world.add_resource(Camera::new(SCREEN_SIZE.0, SCREEN_SIZE.1));
        world.add_resource(BattleState::new());
        world.add_resource(PlayState::Combining);
        world.add_resource(InputState::Rest);
        world.add_resource(InventoryState::new());

        let mut spirits = Vec::new();
        let moves = [
            Move {
                name: "Attack".to_string(),
                effect: MoveType::DamageOne(5),
            },
            Move {
                name: "Defend".to_string(),
                effect: MoveType::Defend(5),
            },
            Move {
                name: "Special".to_string(),
                effect: MoveType::DamageMany(3),
            },
            Move {
                name: "Special Defense".to_string(),
                effect: MoveType::Heal(10),
            },
        ];
        spirits.push(Spirit {
            element: SpiritType::Fire(0),
            health: 10,
            max_health: 10,
            moves: moves.clone(),
        });
        spirits.push(Spirit {
            element: SpiritType::Fire(0),
            health: 10,
            max_health: 10,
            moves: moves.clone(),
        });
        spirits.push(Spirit {
            element: SpiritType::Fire(0),
            health: 10,
            max_health: 10,
            moves: moves.clone(),
        });
        spirits.push(Spirit {
            element: SpiritType::Fire(0),
            health: 10,
            max_health: 10,
            moves: moves.clone(),
        });
        spirits.push(Spirit {
            element: SpiritType::Fire(0),
            health: 10,
            max_health: 10,
            moves: moves.clone(),
        });
        spirits.push(Spirit {
            element: SpiritType::Fire(0),
            health: 10,
            max_health: 10,
            moves: moves.clone(),
        });
        spirits.push(Spirit {
            element: SpiritType::Light(0),
            health: 10,
            max_health: 10,
            moves: moves.clone(),
        });

        world.create_entity()
            .with(WorldEntity { location: (1, 1) })
            .with(Player { spirits: spirits.clone() })
            .build();
        world.create_entity()
            .with(WorldEntity {
                location: (2, 2),
            })
            .with(Encounter {
                spirits,
            })
            .build();

        let dispatcher = DispatcherBuilder::new()
            .with(HandleMove, "move", &[])
            .with(HandleBattleMenu, "battle_menu", &[])
            .with(HandleInventory, "inventory", &[])
            .with(CameraSystem, "camera", &[])
            .with(FindEncounters, "find", &[])
            .with(WatchAttack, "attack", &[])
            .with(WatchSpirits, "spirits", &[])
            .build();

        GameState {
            dispatcher,
            world,
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
        render_world(ctx, &mut self.world)?;
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

    let state = &mut GameState::new(ctx);
    state.init(ctx).expect("Failed to load resources");

    match event::run(ctx, state) {
        Err(e) => println!("Error encountered running game: {}", e),
        Ok(_) => {},
    }
}
