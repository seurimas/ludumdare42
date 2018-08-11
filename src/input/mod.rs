mod world;
mod combat;
mod inventory;

use state::PlayState;
use ggez::*;
use specs::*;
pub use self::world::HandleMove;
pub use self::combat::HandleBattleMenu;
pub use self::inventory::HandleInventory;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
