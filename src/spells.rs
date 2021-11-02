use super::KnownSpell;

pub fn fireball() -> KnownSpell {
    KnownSpell {
        display_name: "fireball".to_string(),
        mana_cost: 1,
        components: vec!["Deals Damage".to_string(), "Area of effect".to_string()],
    }
}

pub fn icicle() -> KnownSpell {
    KnownSpell {
        display_name: "icicle".to_string(),
        mana_cost: 1,
        components: vec!["Deals Damage".to_string(), "Pierces".to_string()],
    }
}

pub fn ghost_bolt() -> KnownSpell {
    KnownSpell {
        display_name: "ghost bolt".to_string(),
        mana_cost: 1,
        components: vec!["Deals Damage".to_string(), "Re-trigger on kill".to_string()],
    }
}

pub fn chain_lightning() -> KnownSpell {
    KnownSpell {
        display_name: "chain lightning".to_string(),
        mana_cost: 1,
        components: vec!["Deals Damage".to_string(), "Bounces".to_string()],
    }
}
