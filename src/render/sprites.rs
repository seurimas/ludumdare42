use ggez::*;
use specs::*;
use ggez::graphics::*;
use state::*;

const INVENTORY_SPRITE_SIZE: (f32, f32) = (32.0, 32.0);
const SPRITESHEET_SIZE: (f32, f32) = (512.0, 512.0);

fn sprite_src(pu: f32, pv: f32, pw: f32, ph: f32) -> Rect {
    Rect::new(pu / SPRITESHEET_SIZE.0, pv / SPRITESHEET_SIZE.1, pw / SPRITESHEET_SIZE.0, ph / SPRITESHEET_SIZE.1)
}

fn sprite_scale(pw: f32, ph: f32, width: i32, height: i32) -> Point2 {
    Point2::new(width as f32 / pw, height as f32 / ph)
}

pub fn inventory_sprite(element: &SpiritType, x: i32, y: i32, width: i32, height: i32) -> DrawParam {
    let (pu, pv) = match element {
        SpiritType::Fire(0) => (0.0, 32.0),
        _ => (448.0, 448.0),
    };
    let src = sprite_src(pu, pv, INVENTORY_SPRITE_SIZE.0, INVENTORY_SPRITE_SIZE.1);
    let dest = Point2::new(x as f32, y as f32);
    let rotation = 0.0;
    let scale = Point2::new(width as f32 / INVENTORY_SPRITE_SIZE.0, height as f32 / INVENTORY_SPRITE_SIZE.1);
    DrawParam {
        src,
        dest,
        rotation,
        scale,
        offset: Point2::new(0.0, 0.0),
        shear: Point2::new(0.0, 0.0),
        color: None,
    }
}

pub fn enemy_bar_sprite(x: i32, y: i32, width: i32, height: i32) -> DrawParam {
    DrawParam {
        src: sprite_src(0.0, 0.0, 64.0, 16.0),
        dest: Point2::new(x as f32, y as f32),
        rotation: 0.0,
        scale: sprite_scale(64.0, 16.0, width, height),
        offset: Point2::new(0.0, 0.0),
        shear: Point2::new(0.0, 0.0),
        color: None,
    }
}

pub fn ally_bar_sprite(x: i32, y: i32, width: i32, height: i32) -> DrawParam {
    DrawParam {
        src: sprite_src(0.0, 16.0, 64.0, 16.0),
        dest: Point2::new(x as f32, y as f32),
        rotation: 0.0,
        scale: sprite_scale(64.0, 16.0, width, height),
        offset: Point2::new(0.0, 0.0),
        shear: Point2::new(0.0, 0.0),
        color: None,
    }
}
