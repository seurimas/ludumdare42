use state::*;

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

pub fn health(spirit: &Spirit) -> String {
    format!("{} / {}", spirit.health, spirit.max_health)
}
