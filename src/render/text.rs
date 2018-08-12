use state::*;
use ggez::*;
use ggez::graphics::*;
use input::*;

pub fn spirit_name(element: &SpiritType) -> String {
    match element {
        SpiritType::Fire(0) => "Ember",
        SpiritType::Fire(1) => "Fire Elemental",
        SpiritType::Fire(2) => "Efreet",

        SpiritType::Water(0) => "Nymph",
        SpiritType::Water(1) => "Water Maiden",
        SpiritType::Water(2) => "Leviathan",

        SpiritType::Slime(0) => "Lesser Slime",
        SpiritType::Slime(1) => "Slime",
        SpiritType::Slime(2) => "Greater Slime",

        SpiritType::Dark(0) => "Imp",
        SpiritType::Dark(1) => "Fiend",
        SpiritType::Dark(2) => "Dominator",

        SpiritType::Light(0) => "Wisp",
        SpiritType::Light(1) => "Guardian",
        SpiritType::Light(2) => "Angel",

        _ => "Unknown",
    }.to_string()
}

pub fn spirit_level_text(element: &SpiritType) -> String {
    match element {
        SpiritType::Fire(level) => format!("Level {} Fire Elemental", level),
        SpiritType::Water(level) => format!("Level {} Water Elemental", level),
        SpiritType::Slime(level) => format!("Level {} Slime", level),
        SpiritType::Light(level) => format!("Level {} Spirit of Light", level),
        SpiritType::Dark(level) => format!("Level {} Spirit of Darkness", level),
    }
}

pub fn collide_text(element: &SpiritType) -> String {
    if !can_upgrade(element) {
        format!("Cannot exceed its current power")
    } else {
        match element {
            SpiritType::Fire(level) => format!("Can combust with {} other {} to become a {}",
                required_spirits(element),
                spirit_name(element),
                spirit_name(&SpiritType::Fire(level + 1)),
            ),
            SpiritType::Water(level) => format!("Can mingle with {} other {} to become a {}",
                required_spirits(element),
                spirit_name(element),
                spirit_name(&SpiritType::Water(level + 1)),
            ),
            SpiritType::Slime(level) => format!("Can absorb {} other {} to become a {}",
                required_spirits(element),
                spirit_name(element),
                spirit_name(&SpiritType::Fire(level + 1)),
            ),
            SpiritType::Light(level) => format!("Can accept the sacrifies of {} other {} to become a {}",
                required_spirits(element),
                spirit_name(element),
                spirit_name(&SpiritType::Fire(level + 1)),
            ),
            SpiritType::Dark(level) => format!("Can consume {} other {} to become a {}",
                required_spirits(element),
                spirit_name(element),
                spirit_name(&SpiritType::Fire(level + 1)),
            ),
        }
    }
}

pub fn health(spirit: &Spirit) -> String {
    format!("{} / {}", spirit.health, spirit.max_health)
}

pub fn text_in_box(ctx: &mut Context, text: &String, region: (i32, i32, i32)) -> GameResult<()> {
    let font = Font::default_font()?;
    let (size, lines) = font.get_wrap(text, region.2 as usize);
    for (idx, line) in lines.iter().enumerate() {
        let text = Text::new(
            ctx,
            &line,
            &font,
        )?;
        draw(ctx, &text, Point2::new(
            region.0 as f32,
            region.1 as f32 + idx as f32 * 16.0,
        ), 0.0)?;
    }
    Ok(())
}
