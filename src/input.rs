use ggez::*;
use specs::*;
use state::*;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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

fn move_in_level(loc: (u32, u32), direction: &Direction, level: &Level) -> Option<(u32, u32)> {
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

pub fn handle_arrow(ctx: &mut Context, world: &mut World, play_state: &PlayState, direction: Direction) {
    match play_state {
        PlayState::InWorld => {
            println!("{:?}", direction);
            move_player(ctx, world, &direction);
        },
        _ => {

        }
    }
}

fn move_player(ctx: &mut Context, world: &mut World, direction: &Direction) {
    let level = world.read_resource::<Level>();
    let mut entities = world.write_storage::<WorldEntity>();
    for mut ent in (&mut entities).join() {
        if ent.entity_type == 0 {
            match move_in_level(ent.location, &direction, &level) {
                Some(next) => { ent.location = next; },
                _ => {}
            }
        }
    }
}
