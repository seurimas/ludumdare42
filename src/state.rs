use ggez::*;
use specs::*;
use std::collections::HashMap;
use input::Direction;

pub const SCREEN_SIZE: (u32, u32) = (632, 368);
pub const TILE_SIZE: u32 = 64;
pub const CHAR_SIZE: u32 = 56;

#[derive(Debug, Clone)]
pub enum MoveType {
    DamageMany(u32),
    DamageOne(u32),
    Heal(u32),
    Defend(u32),
}

#[derive(Debug, Clone)]
pub struct Move {
    pub name: String,
    pub effect: MoveType,
}

#[derive(PartialEq, Debug, Clone)]
pub enum SpiritType {
    Fire(u32),
    Water(u32),
    Slime(u32),
    Light(u32),
    Dark(u32),
}

#[derive(Component, Debug, Clone)]
pub struct Spirit {
    pub element: SpiritType,
    pub max_health: u32,
    pub health: u32,
    pub moves: [Move; 4],
}

impl Spirit {
    pub fn level(&self) -> u32 {
        match self.element {
            SpiritType::Fire(level) => level,
            SpiritType::Water(level) => level,
            SpiritType::Slime(level) => level,
            SpiritType::Light(level) => level,
            SpiritType::Dark(level) => level,
        }
    }
}

#[derive(Component, Debug)]
pub struct PlayerSpirit {
    pub active: bool,
}

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub spirits: Vec<Spirit>,
}

#[derive(Default, Debug)]
pub struct BattleState {
    pub in_combat: bool,
    pub retreating: bool,
    pub activate: bool,
    pub combat_move: Option<usize>,
    pub active_entity: Option<Entity>,
    pub encounter_entity: Option<Entity>,
}

impl BattleState {
    pub fn new() -> Self {
        BattleState {
            in_combat: true,
            retreating: false,
            activate: false,
            combat_move: None,
            active_entity: None,
            encounter_entity: None,
        }
    }
    pub fn want_attack(&mut self) {
        self.activate = true;
    }
    pub fn finish_attack(&mut self) {
        self.activate = false;
    }
    pub fn get_move<'a>(&self, spirits: &WriteStorage<'a, Spirit>) -> Option<Move> {
        match (self.active_entity, self.combat_move) {
            (Some(entity), Some(index)) => {
                match spirits.get(entity) {
                    Some(spirit) => {
                        spirit.moves.get(index).map(|v| v.clone())
                    },
                    None => None
                }
            },
            _ => None,
        }
    }
}

#[derive(Debug, Component, Clone)]
pub struct Encounter {
    pub spirits: Vec<Spirit>,
}

#[derive(Default)]
pub struct Camera {
    pub x_offset: u32,
    pub y_offset: u32,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Camera {
            x_offset: 0,
            y_offset: 0,
            width: screen_width / TILE_SIZE + 2,
            height: screen_height / TILE_SIZE + 2,
        }
    }
}

#[derive(Debug, Component)]
pub struct WorldEntity {
    pub location: (u32, u32),
}

pub struct Tile {
    pub active: bool,
}

pub struct Level {
    pub tiles: HashMap<(u32, u32), Tile>,
}

impl Level {
    pub fn new() -> Self {
        let mut tiles = HashMap::new();

        tiles.insert((0, 0), Tile { active: true });
        tiles.insert((1, 0), Tile { active: true });
        tiles.insert((0, 1), Tile { active: true });
        tiles.insert((1, 1), Tile { active: true });
        for x in 0..10 {
            for y in 0..10 {
                tiles.insert((x, y), Tile { active: true });
            }
        }
        for x in 5..15 {
            for y in 5..15 {
                tiles.insert((x, y), Tile { active: true });
            }
        }
        for x in 10..20 {
            for y in 10..20 {
                tiles.insert((x, y), Tile { active: true });
            }
        }
        Level {
            tiles,
        }
    }

    pub fn has_tile(&self, loc: (u32, u32)) -> bool {
        match self.tiles.get(&loc) {
            Some(tile) => tile.active,
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum PlayState {
    InWorld,
    InBattle,
    Combining,
    GameOver,
    Looting(Vec<SpiritType>),
    MainMenu,
}

#[derive(Clone)]
pub enum InputState {
    Rest,
    Move(Direction),
    Select,
    Escape,
}

#[derive(Clone)]
pub struct InventoryState {
    pub index: usize,
}

impl InventoryState {
    pub fn new() -> Self {
        InventoryState {
            index: 0,
        }
    }
}

pub struct GameState<'a, 'b> {
    pub dispatcher: Dispatcher<'a, 'b>,
    pub world: World,
}
