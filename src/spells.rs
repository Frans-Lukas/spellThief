use super::KnownSpell;

pub fn fireball() -> KnownSpell{
    KnownSpell{
        display_name: "fireball".to_string(),
        mana_cost: 1
    }
}

pub fn icicle() -> KnownSpell{
    KnownSpell{
        display_name: "icicle".to_string(),
        mana_cost: 1
    }
}

pub fn ghost_bolt() -> KnownSpell{
    KnownSpell{
        display_name: "ghost bolt".to_string(),
        mana_cost: 1
    }
}

pub fn chain_lightning() -> KnownSpell{
    KnownSpell{
        display_name: "chain lightning".to_string(),
        mana_cost: 1
    }
}