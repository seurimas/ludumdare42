use ggez::*;
use ggez::audio::Source;
use specs::*;
use std::collections::HashMap;
use input::Direction;
use rand::*;
use rand::distributions::{Normal, Distribution};
use std::time::Duration;
use std::cmp;

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

impl MoveType {
    pub fn actual_amount(&self, attacker: &Spirit, defender: &Spirit) -> u32 {
        let mut rng = thread_rng();
        let attack = attacker.attack * (attacker.element.level() + 1);
        let defense = defender.defense;
        let stamina = attacker.stamina;
        let type_advantage = attacker.element.type_advantage(&defender.element);
        let amount = match self {
            MoveType::DamageMany(x) => {
                let calculated = *x as f32
                    + (*x as f32 * (attack as f32 - 8.0) as f32 / 45.0); // Base attack.
                let calculated = calculated + (rng.gen::<f32>() * calculated) / 10.0;
                let calculated = calculated * (1.0 + 0.5 * type_advantage as f32);
                let calculated = calculated - (defense * defense) as f32;
                cmp::max(1, calculated as i32) as u32
            },
            MoveType::DamageOne(x) => {
                let calculated = *x as f32
                    + (*x as f32 * (attack as f32 - 8.0) as f32 / 45.0); // Base attack.
                let calculated = calculated + (rng.gen::<f32>() * calculated) / 10.0;
                let calculated = calculated * (1.0 + 0.5 * type_advantage as f32);
                let calculated = calculated - (defense * defense / 4) as f32;
                cmp::max(1, calculated as i32) as u32
            },
            MoveType::Heal(x) => {
                cmp::max(1, *x as i32 + stamina as i32) as u32
            },
            MoveType::Defend(x) => *x,
        };
        amount
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    pub name: String,
    pub effect: MoveType,
}

fn enemy_moves() -> [Move; 8] {
    [
        Move {
            name: "(Fight)".to_string(),
            effect: MoveType::DamageOne(2),
        },
        Move {
            name: "(Advance)".to_string(),
            effect: MoveType::DamageOne(4),
        },
        Move {
            name: "(Thrash)".to_string(),
            effect: MoveType::DamageMany(3),
        },
        Move {
            name: "(Revive)".to_string(),
            effect: MoveType::Heal(4),
        },
        Move {
            name: "(Thrash)".to_string(),
            effect: MoveType::DamageMany(3),
        },
        Move {
            name: "(Thrash)".to_string(),
            effect: MoveType::DamageMany(3),
        },
        Move {
            name: "(Revive)".to_string(),
            effect: MoveType::Heal(4),
        },
        Move {
            name: "(Defend))".to_string(),
            effect: MoveType::Defend(2),
        },
    ]
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

fn water_moves() -> [Move; 8] {
    [
        Move {
            name: "Squirt".to_string(),
            effect: MoveType::DamageOne(4),
        },
        Move {
            name: "Jet".to_string(),
            effect: MoveType::DamageOne(7),
        },
        Move {
            name: "Wave".to_string(),
            effect: MoveType::DamageOne(10),
        },
        Move {
            name: "Deluge".to_string(),
            effect: MoveType::DamageMany(3),
        },
        Move {
            name: "Tsunami".to_string(),
            effect: MoveType::DamageMany(6),
        },
        Move {
            name: "Bubble".to_string(),
            effect: MoveType::Heal(8),
        },
        Move {
            name: "Refill".to_string(),
            effect: MoveType::Heal(16),
        },
        Move {
            name: "Raise Tides".to_string(),
            effect: MoveType::Defend(2),
        },
    ]
}

fn slime_moves() -> [Move; 8] {
    [
        Move {
            name: "Nibble".to_string(),
            effect: MoveType::DamageOne(4),
        },
        Move {
            name: "Chomp".to_string(),
            effect: MoveType::DamageOne(7),
        },
        Move {
            name: "Consume".to_string(),
            effect: MoveType::DamageOne(10),
        },
        Move {
            name: "Pummel".to_string(),
            effect: MoveType::DamageMany(3),
        },
        Move {
            name: "Explode".to_string(),
            effect: MoveType::DamageMany(6),
        },
        Move {
            name: "Reform".to_string(),
            effect: MoveType::Heal(8),
        },
        Move {
            name: "Eat".to_string(),
            effect: MoveType::Heal(16),
        },
        Move {
            name: "Harden".to_string(),
            effect: MoveType::Defend(2),
        },
    ]
}

fn dark_moves() -> [Move; 8] {
    [
        Move {
            name: "Scold".to_string(),
            effect: MoveType::DamageOne(4),
        },
        Move {
            name: "Punish".to_string(),
            effect: MoveType::DamageOne(7),
        },
        Move {
            name: "Eviscerate".to_string(),
            effect: MoveType::DamageOne(10),
        },
        Move {
            name: "Dominate".to_string(),
            effect: MoveType::DamageMany(3),
        },
        Move {
            name: "Destroy".to_string(),
            effect: MoveType::DamageMany(6),
        },
        Move {
            name: "Unholy Health".to_string(),
            effect: MoveType::Heal(8),
        },
        Move {
            name: "Unholy Greed".to_string(),
            effect: MoveType::Heal(16),
        },
        Move {
            name: "Unholy Armor".to_string(),
            effect: MoveType::Defend(2),
        },
    ]
}

fn light_moves() -> [Move; 8] {
    [
        Move {
            name: "Slash".to_string(),
            effect: MoveType::DamageOne(4),
        },
        Move {
            name: "Bash".to_string(),
            effect: MoveType::DamageOne(7),
        },
        Move {
            name: "Avenge".to_string(),
            effect: MoveType::DamageOne(10),
        },
        Move {
            name: "Radiant".to_string(),
            effect: MoveType::DamageMany(3),
        },
        Move {
            name: "Great Light".to_string(),
            effect: MoveType::DamageMany(6),
        },
        Move {
            name: "Heal".to_string(),
            effect: MoveType::Heal(8),
        },
        Move {
            name: "Resurrect".to_string(),
            effect: MoveType::Heal(16),
        },
        Move {
            name: "Protect".to_string(),
            effect: MoveType::Defend(2),
        },
    ]
}

#[derive(Debug, Clone)]
pub enum CombatEffect {
    Damage(u32),
    Heal(u32),
    Defense(u32),
    ShedDefense(u32),
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
                    if spirit.defense > 6 {
                        spirit.defense += 1;
                        if *amount > 1 {
                            new_effects.push(CombatEffect::Defense(amount - 1));
                        }
                    }
                },
                CombatEffect::ShedDefense(amount) => {
                    if spirit.defense > spirit.base_defense {
                        spirit.defense -= 1;
                        if *amount > 1 {
                            new_effects.push(CombatEffect::ShedDefense(amount - 1));
                        }
                    }
                }
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
    pub fn base_level(&self) -> SpiritType {
        match self {
            SpiritType::Fire(level) => SpiritType::Fire(0),
            SpiritType::Water(level) => SpiritType::Water(0),
            SpiritType::Slime(level) => SpiritType::Slime(0),
            SpiritType::Light(level) => SpiritType::Light(0),
            SpiritType::Dark(level) => SpiritType::Dark(0),
        }
    }
    pub fn type_advantage(&self, other: &SpiritType) -> i32 {
        match (self, other) {
            (SpiritType::Fire(level), SpiritType::Slime(_)) => 1,
            (SpiritType::Fire(level), SpiritType::Water(_)) => -1,
            (SpiritType::Water(level), SpiritType::Fire(_)) => 1,
            (SpiritType::Water(level), SpiritType::Slime(_)) => -1,
            (SpiritType::Slime(level), SpiritType::Water(_)) => 1,
            (SpiritType::Slime(level), SpiritType::Fire(_)) => -1,

            (SpiritType::Light(level), SpiritType::Dark(_)) => *level as i32,
            (SpiritType::Light(level), SpiritType::Light(_)) => -1,
            (SpiritType::Dark(level), SpiritType::Light(_)) => *level as i32,
            (SpiritType::Dark(level), SpiritType::Dark(_)) => -1,
            _ => 0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Spirit {
    pub element: SpiritType,
    pub max_health: u32,
    pub health: u32,
    pub defense: i32,
    pub base_defense: i32,
    pub attack: u32,
    pub stamina: u32,
    pub moves: [Move; 4],
}
fn next_spirit(spirit: Spirit) -> Spirit {
    match spirit.element {
        SpiritType::Fire(level) => {
            Spirit {
                element: SpiritType::Fire(level + 1),
                max_health: spirit.max_health * 4,
                health: spirit.max_health * 4,
                moves: spirit.moves,
                attack: spirit.attack,
                base_defense: spirit.base_defense,
                stamina: spirit.stamina,
                defense: spirit.base_defense,
            }
        },
        SpiritType::Water(level) => {
            Spirit {
                element: SpiritType::Water(level + 1),
                max_health: spirit.max_health * 4,
                health: spirit.max_health * 4,
                moves: spirit.moves,
                attack: spirit.attack,
                base_defense: spirit.base_defense,
                stamina: spirit.stamina,
                defense: spirit.base_defense,
            }
        },
        SpiritType::Slime(level) => {
            Spirit {
                element: SpiritType::Slime(level + 1),
                max_health: spirit.max_health * 4,
                health: spirit.max_health * 4,
                moves: spirit.moves,
                attack: spirit.attack,
                base_defense: spirit.base_defense,
                stamina: spirit.stamina,
                defense: spirit.base_defense,
            }
        },
        SpiritType::Light(level) => {
            Spirit {
                element: SpiritType::Light(level + 1),
                max_health: spirit.max_health * 4,
                health: spirit.max_health * 4,
                moves: spirit.moves,
                attack: spirit.attack,
                base_defense: spirit.base_defense,
                stamina: spirit.stamina,
                defense: spirit.base_defense,
            }
        },
        SpiritType::Dark(level) => {
            Spirit {
                element: SpiritType::Dark(level + 1),
                max_health: spirit.max_health * 4,
                health: spirit.max_health * 4,
                moves: spirit.moves,
                attack: spirit.attack,
                base_defense: spirit.base_defense,
                stamina: spirit.stamina,
                defense: spirit.base_defense,
            }
        },
    }
}

impl Spirit {
    pub fn new(element: SpiritType, is_player: bool) -> Self {
        let mut rng = thread_rng();
        let mut moves = match (is_player, &element) {
            (false, _) => {
                enemy_moves()
            },
            (true, SpiritType::Fire(_)) => {
                fire_moves()
            },
            (true, SpiritType::Water(_)) => {
                water_moves()
            },
            (true, SpiritType::Slime(_)) => {
                slime_moves()
            },
            (true, SpiritType::Dark(_)) => {
                dark_moves()
            },
            (true, SpiritType::Light(_)) => {
                light_moves()
            },
        };
        let (max_health, base_defense, attack, stamina) = match is_player {
            true => {
                (
                    (10.0 + (rng.gen::<f32>() + rng.gen::<f32>()) * 5.0) as u32,
                    (rng.gen::<f32>() * 5.0) as i32,
                    (rng.gen::<f32>() * 15.0) as u32,
                    (rng.gen::<f32>() * 15.0) as u32,
                )
            },
            false => {
                (((1 + element.level()) * 10) +
                    ((rng.gen::<f32>() + rng.gen::<f32>()) * 5.0) as u32,
                2, 8, 8)
            }
        };
        rng.shuffle(&mut moves);
        let mut spirit = Spirit {
            element: match is_player {
                true => element.base_level(),
                false => element.clone(),
            },
            max_health,
            health: max_health,
            defense: 0,
            base_defense,
            attack,
            stamina,
            moves: [
                moves[0].clone(),
                moves[1].clone(),
                moves[2].clone(),
                moves[3].clone(),
            ],
        };
        if is_player {
            for _ in 0..(element.level()) {
                spirit = next_spirit(spirit);
            }
        }
        spirit
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

#[derive(Default, Clone, Debug)]
pub struct BattleState {
    pub in_combat: bool,
    pub retreating: bool,
    pub enemy_attacking: Option<u32>,
    pub activate: bool,
    pub animating: bool,
    pub combat_move: Option<usize>,
    pub active_entity: Option<Entity>,
    pub encounter_entity: Option<Entity>,
    pub notification: Option<String>,
}

impl BattleState {
    pub fn new() -> Self {
        BattleState {
            in_combat: true,
            retreating: false,
            enemy_attacking: None,
            activate: false,
            animating: false,
            combat_move: None,
            active_entity: None,
            encounter_entity: None,
            notification: None,
        }
    }
    pub fn notifying(&self) -> bool {
        self.notification != None
    }
    pub fn notify(&mut self, notification: String) {
        self.notification = Some(notification);
    }
    pub fn clear_notification(&mut self) {
        self.notification = None;
    }
    pub fn animating(&self) -> bool {
        self.animating || self.notifying()
    }
    pub fn set_animating(&mut self, animating: bool) {
        self.animating = animating;
    }
    pub fn want_attack(&mut self) {
        if self.enemy_attacking == None && !self.animating() {
            self.activate = true;
        }
    }
    pub fn finish_attack(&mut self) {
        self.activate = false;
        self.enemy_attacking = Some(2);
        self.animating = true;
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
    pub period: u32,
    pub duration: Duration,
}

impl Encounter {
    fn encounter_size(level: u32) -> Vec<(u32, u32)> {
        let mut rng = thread_rng();
        let mut spirit_level = (rng.gen::<f32>() * (level as f32 + 1.0)) as u32;
        if spirit_level > 2 {
            spirit_level = 2;
        }
        let mut remaining = 11;
        let mut spirit_counts = Vec::new();
        for spirit_level in (0..(spirit_level + 1)).rev() {
            let count = (level - spirit_level + 1) as f32 * rng.gen::<f32>() * 5.0;
            let count = cmp::min(remaining, cmp::max(1, count as u32));
            spirit_counts.push((
                spirit_level,
                count,
            ));
            remaining -= count;
        }
        spirit_counts
    }
    fn random_encounter(level: u32) -> Vec<Spirit> {
        let mut spirits = Vec::new();
        let sizes = Encounter::encounter_size(level);
        match (thread_rng().gen::<f32>() * 5.0) as u32 {
            0 => {
                for (spirit_level, count) in sizes.iter() {
                    for _ in 0..*count {
                        spirits.push(Spirit::new(
                            SpiritType::Fire(*spirit_level),
                            false,
                        ));
                    }
                }
            },
            1 => {
                for (spirit_level, count) in sizes.iter() {
                    for _ in 0..*count {
                        spirits.push(Spirit::new(
                            SpiritType::Water(*spirit_level),
                            false,
                        ));
                    }
                }
            },
            2 => {
                for (spirit_level, count) in sizes.iter() {
                    for _ in 0..*count {
                        spirits.push(Spirit::new(
                            SpiritType::Slime(*spirit_level),
                            false,
                        ));
                    }
                }
            },
            3 => {
                for (spirit_level, count) in sizes.iter() {
                    for _ in 0..*count {
                        spirits.push(Spirit::new(
                            SpiritType::Light(*spirit_level),
                            false,
                        ));
                    }
                }
            },
            _ => {
                for (spirit_level, count) in sizes.iter() {
                    for _ in 0..*count {
                        spirits.push(Spirit::new(
                            SpiritType::Dark(*spirit_level),
                            false,
                        ));
                    }
                }
            },
        }
        spirits
    }
    pub fn new(level: u32) -> Self {
        let mut rng = thread_rng();
        let spirits = Encounter::random_encounter(level);
        let period = 250000000 * (1.0 + rng.gen::<f32>() * 4.0) as u32;
        Encounter {
            spirits,
            period,
            duration: Duration::new(0, period),
        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        match self.duration.checked_sub(delta) {
            Some(next) => {
                self.duration = next;
                false
            },
            None => {
                self.duration = Duration::new(0, self.period);
                true
            }
        }
    }
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

#[derive(Clone, Debug)]
pub struct Room {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
    visited: bool,
}

pub struct Level {
    pub depth: u32,
    pub tiles: HashMap<(u32, u32), Tile>,
    pub rooms: HashMap<(u32, u32), Room>,
    pub entrance: (u32, u32),
    pub exit: (u32, u32),
}

fn unvisited_neighbors(location: &(u32, u32), rooms: &HashMap<(u32, u32), Room>) -> Vec<(u32, u32)> {
    let mut unvisited = Vec::new();
    let (x, y) = *location;
    if x == 0 {}
    else if let Some(room) = rooms.get(&(x - 1, y)) {
        if !room.visited {
            unvisited.push((x - 1, y));
        }
    }
    if y == 0 {}
    else
    if let Some(room) = rooms.get(&(x, y - 1)) {
        if !room.visited {
            unvisited.push((x, y - 1));
        }
    }
    if let Some(room) = rooms.get(&(x + 1, y)) {
        if !room.visited {
            unvisited.push((x + 1, y));
        }
    }
    if let Some(room) = rooms.get(&(x, y + 1)) {
        if !room.visited {
            unvisited.push((x, y + 1));
        }
    }
    return unvisited;
}

fn gen_maze(size: (u32, u32)) -> HashMap<(u32, u32), Room> {
    let mut rng = thread_rng();
    let mut rooms = HashMap::new();
    for y in 0..size.1 {
        for x in 0..size.0 {
            rooms.insert((x, y), Room {
                north: false,
                east: false,
                south: false,
                west: false,
                visited: false,
            });
        }
    }
    let mut trace = Vec::new();
    trace.push((0, 0));
    while let Some(&head) = trace.last() {
        let head_room = rooms[&head].clone();
        let neighbors = unvisited_neighbors(&head, &rooms);
        if let Some(next) = rng.choose(&neighbors) {
            let next_room = rooms[next].clone();
            let north_south = head.1 > next.1 || rng.gen_bool(0.2);
            let south_north = next.1 > head.1 || rng.gen_bool(0.2);
            let east_west = next.0 > head.0 || rng.gen_bool(0.2);
            let west_east = head.0 > next.0 || rng.gen_bool(0.2);
            trace.push(*next);
            rooms.insert(head, Room {
                north: head_room.north || north_south,
                east: head_room.east || east_west,
                south: head_room.south || south_north,
                west: head_room.west || west_east,
                visited: true,
            });
            rooms.insert(*next, Room {
                north: next_room.north || south_north,
                east: next_room.east || west_east,
                south: next_room.south || north_south,
                west: next_room.west || east_west,
                visited: true,
            });
        } else {
            trace.pop();
        }
    }
    rooms
}

fn tiles_for_rooms(room_size: u32, rooms: &HashMap<(u32, u32), Room>) -> HashMap<(u32, u32), Tile> {
    let mut tiles = HashMap::new();
    for ((room_x, room_y), room) in rooms.iter() {
        let left = room_x * room_size;
        let top = room_y * room_size;
        let right = (room_x + 1) * room_size;
        let bottom = (room_y + 1) * room_size;
        let center_x = (left + right) / 2;
        let center_y = (top + bottom) / 2;
        if room.west {
            for x in left..center_x {
                tiles.insert((x, center_y + 1), Tile { active: true });
                tiles.insert((x, center_y), Tile { active: true });
                tiles.insert((x, center_y - 1), Tile { active: true });
            }
        }
        if room.east {
            for x in center_x..right {
                tiles.insert((x, center_y + 1), Tile { active: true });
                tiles.insert((x, center_y), Tile { active: true });
                tiles.insert((x, center_y - 1), Tile { active: true });
            }
        }
        if room.north {
            for y in top..center_y {
                tiles.insert((center_x - 1, y), Tile { active: true });
                tiles.insert((center_x, y), Tile { active: true });
                tiles.insert((center_x + 1, y), Tile { active: true });
            }
        }
        if room.south {
            for y in center_y..bottom {
                tiles.insert((center_x - 1, y), Tile { active: true });
                tiles.insert((center_x, y), Tile { active: true });
                tiles.insert((center_x + 1, y), Tile { active: true });
            }
        }
        for y in (top + 1)..(bottom - 1) {
            for x in (left + 1)..(right - 1) {
                tiles.insert((x, y), Tile { active: true });
            }
        }
    }
    tiles
}

#[derive(Component)]
pub struct Stair {
    pub depth: u32,
}

const ROOM_SIZE: u32 = 5;
impl Level {
    pub fn new(depth: u32) -> Self {
        let mut rng = thread_rng();
        let entrance = (0, 0);
        let size = (5, 5);
        let exit =(
            size.0 - (rng.gen::<f32>() * 2.0) as u32 - 1,
            size.1 - (rng.gen::<f32>() * 2.0) as u32 - 1,
        );
        let rooms = gen_maze(size);
        println!("{:?}", rooms);
        let tiles = tiles_for_rooms(ROOM_SIZE, &rooms);
        Level {
            depth,
            tiles,
            rooms,
            entrance,
            exit,
        }
    }

    pub fn spawn_encounters(&self, world: &mut World) {
        type ClearData<'a> = (
            Entities<'a>,
            WriteStorage<'a, WorldEntity>,
            WriteStorage<'a, Player>,
            WriteStorage<'a, Stair>,
            ReadStorage<'a, Spirit>,
        );
        world.exec(|(entities, mut world_entities, mut player, mut stairs, spirits): ClearData| {
            for (entity, world_entity, ()) in (&*entities, &world_entities, !&player).join() {
                (*entities).delete(entity);
            }
            for (entity, _spirit) in (&*entities, &spirits).join() {
                (*entities).delete(entity);
            }
            if self.depth == 0 {
                for (entity, world_entity, _player) in (&*entities, &world_entities, &player).join() {
                    (*entities).delete(entity);
                }
                let mut spirits = Vec::new();
                spirits.push(Spirit::new(SpiritType::Fire(0), true));
                spirits.push(Spirit::new(SpiritType::Water(0), true));
                spirits.push(Spirit::new(SpiritType::Slime(0), true));
                entities.build_entity()
                    .with(WorldEntity { location: (2, 2) }, &mut world_entities)
                    .with(Player { spirits: spirits }, &mut player)
                    .build();
            } else {
                for (entity, world_entity, _player) in (&*entities, &mut world_entities, &player).join() {
                    let ex = self.entrance.0 * ROOM_SIZE + (ROOM_SIZE / 2);
                    let ey = self.entrance.1 * ROOM_SIZE + (ROOM_SIZE / 2);
                    world_entity.location = (ex, ey);
                }
            }
            let sx = self.exit.0 * ROOM_SIZE + (ROOM_SIZE / 2);
            let sy = self.exit.1 * ROOM_SIZE + (ROOM_SIZE / 2);
            entities.build_entity()
                .with(WorldEntity {
                    location: (sx, sy),
                }, &mut world_entities)
                .with(Stair { depth: self.depth + 1 }, &mut stairs)
                .build();
        });
        let mut rng = thread_rng();
        for ((x, y), _room) in self.rooms.iter() {
            if (*x, *y) != self.entrance && (*x, *y) != self.exit {
                let odds = cmp::min(8, self.depth + 4);
                if rng.gen_bool(odds as f64 / 10.0) {
                    let tx = x * ROOM_SIZE + (ROOM_SIZE / 2);
                    let ty = y * ROOM_SIZE + (ROOM_SIZE / 2);
                    world.create_entity()
                        .with(WorldEntity {
                            location: (tx, ty),
                        })
                        .with(Encounter::new(self.depth))
                        .build();
                }
            }
        }
    }

    pub fn has_tile(&self, loc: (u32, u32)) -> bool {
        match self.tiles.get(&loc) {
            Some(tile) => tile.active,
            _ => false,
        }
    }
}

fn move_by(loc: (u32, u32), direction: &Direction) -> Option<(u32, u32)> {
    match direction {
        Direction::Up => {
            if loc.1 > 0 {
                Some((loc.0, loc.1 - 1))
            } else {
                None
            }
        },
        Direction::Down => {
            Some((loc.0, loc.1 + 1))
        },
        Direction::Left => {
            if loc.0 > 0 {
                Some((loc.0 - 1, loc.1))
            } else {
                None
            }
        },
        Direction::Right => {
            Some((loc.0 + 1, loc.1))
        },
    }
}

pub fn move_in_level(loc: (u32, u32), direction: &Direction, level: &Level) -> Option<(u32, u32)> {
    match move_by(loc, direction) {
        Some(next) => {
            if level.has_tile(next) {
                Some(next)
            } else {
                None
            }
        },
        None => None,
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
    Stairs(u32),
    MainMenu(u32),
}

#[derive(Clone, PartialEq)]
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

pub struct Sounds {
    pub fire: Source,
    pub water: Source,
    pub slime: Source,
    pub light: Source,
    pub dark: Source,
    pub blip: Source,
    pub collide: Source,
    pub confirm: Source,
    pub cancel: Source,
    pub encounter: Source,
    pub lose: Source,
    pub pending: Vec<Source>,
}

impl Sounds {
    pub fn sound_for_attack(&self, spirit: &Spirit) {
        self.play(match spirit.element {
            SpiritType::Fire(_) => &self.fire,
            SpiritType::Water(_) => &self.water,
            SpiritType::Slime(_) => &self.slime,
            SpiritType::Light(_) => &self.light,
            SpiritType::Dark(_) => &self.dark,
        });
    }
    pub fn play(&self, sound: &Source) {
        if !sound.playing() {
            sound.play();
        }
    }
}

pub struct GameState<'a, 'b> {
    pub dispatcher: Dispatcher<'a, 'b>,
    pub world: World,
}
