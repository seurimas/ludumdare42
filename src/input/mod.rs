mod world;
mod combat;
mod inventory;
mod looting;

use state::PlayState;
use ggez::*;
use specs::*;
pub use self::world::HandleMove;
pub use self::combat::HandleBattleMenu;
pub use self::inventory::HandleInventory;
pub use self::looting::HandleLootMenu;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
