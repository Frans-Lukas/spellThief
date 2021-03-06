use specs::saveload::{MarkedBuilder, SimpleMarker};
use specs::{Builder, Entity, World, WorldExt};

use super::KnownSpell;
use super::{
    AreaOfEffect, Bounces, DestroysWalls, InflictsDamage, Name, Pierces, Ranged, RecastOnKill,
    SerializeMe, Spell,
};
use std::borrow::BorrowMut;

pub enum SpellComponent {
    SCInflictsDamage { damage: i32 },
    AreaOfEffect { range: i32 },
    DestroysWalls,
    Pierces,
    RecastsOnKill,
}

pub fn add_spell_component(ecs: &mut World, spell_entity: &mut Entity, spell_component: SpellComponent) {
}

pub fn fireball(ecs: &mut World) -> KnownSpell {
    ecs.create_entity()
        .with(InflictsDamage { damage: 20 })
        .with(AreaOfEffect { radius: 3 })
        .with(DestroysWalls {})
        .with(Spell {
            name: "fireball".to_string(),
            range: 6,
            mana_cost: 1,
        })
        .with(Name {
            name: "fireball".to_string(),
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
    KnownSpell {
        name: "fireball".to_string(),
        components: vec![
            "ranged".to_string(),
            "deals damage".to_string(),
            "destroys walls".to_string(),
            "area of effect".to_string(),
        ],
    }
}

//
// pub fn icicle(ecs: &mut World) -> KnownSpell {
//     let spell_entity = ecs
//         .create_entity()
//         .with(Ranged { range: 6 })
//         .with(InflictsDamage { damage: 20 })
//         .with(Pierces {})
//         .with(Spell {})
//         .marked::<SimpleMarker<SerializeMe>>()
//         .build();
//     KnownSpell {
//         display_name: "icicle".to_string(),
//         mana_cost: 1,
//         components: vec![
//             "ranged".to_string(),
//             "deals damage".to_string(),
//             "pierces".to_string(),
//         ],
//     }
// }
//
// pub fn ghost_bolt(ecs: &mut World) -> KnownSpell {
//     let spell_entity = ecs
//         .create_entity()
//         .with(Ranged { range: 6 })
//         .with(InflictsDamage { damage: 20 })
//         .with(RecastOnKill {})
//         .with(Spell {})
//         .marked::<SimpleMarker<SerializeMe>>()
//         .build();
//     KnownSpell {
//         display_name: "ghost bolt".to_string(),
//         mana_cost: 1,
//         components: vec![
//             "ranged".to_string(),
//             "deals damage".to_string(),
//             "recast on kill".to_string(),
//         ],
//         spell: spell_entity,
//     }
// }
//
// pub fn chain_lightning(ecs: &mut World) -> KnownSpell {
//     let spell_entity = ecs
//         .create_entity()
//         .with(Ranged { range: 6 })
//         .with(InflictsDamage { damage: 20 })
//         .with(Bounces { bounce_range: 4 })
//         .with(Spell {})
//         .marked::<SimpleMarker<SerializeMe>>()
//         .build();
//     KnownSpell {
//         display_name: "chain lightning".to_string(),
//         mana_cost: 1,
//         components: vec![
//             "ranged".to_string(),
//             "deals damage".to_string(),
//             "bounces".to_string(),
//         ],
//         spell: spell_entity,
//     }
// }
