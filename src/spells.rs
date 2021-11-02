use specs::saveload::SimpleMarker;
use specs::Entity;

use {AreaOfEffect, InflictsDamage};
use {Bounces, Spell};
use {Pierces, RecastOnKill};
use {Ranged, SerializeMe};

use super::KnownSpell;

pub fn fireball(spell_entity: Entity) -> KnownSpell {
    ecs.create_entity()
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 20 })
        .with(AreaOfEffect { radius: 3 })
        .with(Spell {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
    KnownSpell {
        display_name: "fireball".to_string(),
        mana_cost: 1,
        components: vec![
            "ranged".to_string(),
            "deals damage".to_string(),
            "area of effect".to_string(),
        ],
        spell: spell_entity,
    }
}

pub fn icicle(spell_entity: Entity) -> KnownSpell {
    ecs.create_entity()
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 20 })
        .with(Pierces {})
        .with(Spell {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
    KnownSpell {
        display_name: "icicle".to_string(),
        mana_cost: 1,
        components: vec![
            "ranged".to_string(),
            "deals damage".to_string(),
            "pierces".to_string(),
        ],
        spell: spell_entity,
    }
}

pub fn ghost_bolt(spell_entity: Entity) -> KnownSpell {
    ecs.create_entity()
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 20 })
        .with(RecastOnKill {})
        .with(Spell {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
    KnownSpell {
        display_name: "ghost bolt".to_string(),
        mana_cost: 1,
        components: vec![
            "ranged".to_string(),
            "deals damage".to_string(),
            "recast on kill".to_string(),
        ],
        spell: spell_entity,
    }
}

pub fn chain_lightning() -> KnownSpell {
    ecs.create_entity()
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 20 })
        .with(Bounces { bounce_range: 4 })
        .with(Spell {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
    KnownSpell {
        display_name: "chain lightning".to_string(),
        mana_cost: 1,
        components: vec![
            "ranged".to_string(),
            "deals damage".to_string(),
            "bounces".to_string(),
        ],
        spell: spell_entity,
    }
}
