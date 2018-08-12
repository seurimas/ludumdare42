use ggez::*;
use specs::*;
use std::collections::HashMap;
use input::Direction;
use rand::*;
use rand::distributions::{Normal, Distribution};
use std::time::Duration;

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

fn fire_moves() -> [Move; 8] {
    [
        Move {
            name: "Blast".to_string(),
            effect: MoveType::DamageOne(4),
        },
        Move {
            name: "Scorch".to_string(),
            effect: MoveType::DamageOne(7),
        },
        Move {
            name: "Incinerate".to_string(),
            effect: MoveType::DamageOne(10),
        },
        Move {
            name: "Inferno".to_string(),
            effect: MoveType::DamageMany(3),
        },
        Move {
            name: "Decimate".to_string(),
            effect: MoveType::DamageMany(6),
        },
        Move {
            name: "Recombust".to_string(),
            effect: MoveType::Heal(8),
        },
        Move {
            name: "Resurge".to_string(),
            effect: MoveType::Heal(16),
        },
        Move {
            name: "Heat".to_string(),
            effect: MoveType::Defend(2),
        },
    ]
}

#[derive(Debug, Clone)]
pub enum CombatEffect {
    Damage(u32),
    Heal(u32),
    Defense(u32),
}

#[derive(Component, Debug, Clone)]
pub struct CombatEffects {
    pub effects: Vec<CombatEffect>,
    pub duration: Duration,
}

impl CombatEffects {
    pub fn new(effects: Vec<CombatEffect>) -> Self {
        CombatEffects {
            effects,
            duration: Duration::from_millis(100),
        }
    }
    pub fn active(&self) -> bool {
        self.effects.len() > 0
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        match self.duration.checked_sub(delta) {
            Some(next) => {
                self.duration = next;
                false
            },
            None => {
                self.duration = Duration::from_millis(100);
                true
            }
        }
    }
    pub fn apply_tick(&mut self, spirit: &mut Spirit) {
        let mut new_effects = Vec::new();
        for effect in self.effects.iter() {
            match effect {
                CombatEffect::Damage(amount) => {
                    if spirit.health > 0 {
                        spirit.health -= 1;
                        if *amount > 1 {
                            new_effects.push(CombatEffect::Damage(amount - 1));
                        }
                    }
                },
                CombatEffect::Heal(amount) => {
                    if spirit.health < spirit.max_health {
                        spirit.health += 1;
                        if *amount > 1 {
                            new_effects.push(CombatEffect::Heal(amount - 1));
                        }
                    }
                },
                CombatEffect::Defense(amount) => {
                    spirit.defense += 1;
                    if *amount > 1 {
                        new_effects.push(CombatEffect::Defense(amount - 1));
                    }
                },
            }
        }
        self.effects = new_effects;
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum SpiritType {
    Fire(u32),
    Water(u32),
    Slime(u32),
    Light(u32),
    Dark(u32),
}

impl SpiritType {
    pub fn level(&self) -> u32 {
        match self {
            SpiritType::Fire(level) => *level,
            SpiritType::Water(level) => *level,
            SpiritType::Slime(level) => *level,
            SpiritType::Light(level) => *level,
            SpiritType::Dark(level) => *level,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Spirit {
    pub element: SpiritType,
    pub max_health: u32,
    pub health: u32,
    pub defense: u32,
    pub moves: [Move; 4],
}

impl Spirit {
    pub fn new(element: SpiritType) -> Self {
        let mut rng = thread_rng();
        let max_health = ((1 + element.level()) * 10) +
            ((rng.gen::<f32>() + rng.gen::<f32>()) * 5.0) as u32;
        let mut moves = fire_moves();
        rng.shuffle(&mut moves);
        Spirit {
            element,
            max_health,
            health: max_health,
            defense: 0,
            moves: [
                moves[0].clone(),
                moves[1].clone(),
                moves[2].clone(),
                moves[3].clone(),
            ],
        }
    }
    pub fn level(&self) -> u32 {
        self.element.level()
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

#[derive(Default, Debug, Clone)]
pub struct BattleState {
    pub in_combat: bool,
    pub retreating: bool,
    pub enemy_attacking: bool,
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
            enemy_attacking: false,
            activate: false,
            combat_move: None,
            active_entity: None,
            encounter_entity: None,
        }
    }
    pub fn want_attack(&mut self) {
        if !self.enemy_attacking {
            self.activate = true;
        }
    }
    pub fn finish_attack(&mut self) {
        self.activate = false;
        self.enemy_attacking = true;
    }
    pub fn retreat(&mut self) {
        self.active_entity = None;
        self.retreating = true;
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
    Looting {
        captured: Vec<SpiritType>,
        lost: Vec<SpiritType>,
    },
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
pub fn can_upgrade(element: &SpiritType) -> bool {
    match element {
        SpiritType::Fire(2) => false,
        SpiritType::Water(2) => false,
        SpiritType::Slime(2) => false,
        SpiritType::Light(2) => false,
        SpiritType::Dark(2) => false,
        _ => true,
    }
}
pub fn required_spirits(element: &SpiritType) -> u32 {
    match element {
        SpiritType::Fire(level) => {
            4 + level * 2
        },
        SpiritType::Water(level) => {
            4 + level * 2
        },
        SpiritType::Slime(level) => {
            2 + level * 1
        },
        SpiritType::Light(level) => {
            3 + level * 4
        },
        SpiritType::Dark(level) => {
            3 + level * 4
        }
    }
}

pub struct GameState<'a, 'b> {
    pub dispatcher: Dispatcher<'a, 'b>,
    pub world: World,
}
