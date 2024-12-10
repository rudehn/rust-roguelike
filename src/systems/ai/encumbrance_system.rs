use specs::prelude::*;
use crate::{EquipmentChanged, Item, InBackpack, Equipped, Pools, Attributes, AttributeBonus,
    gamesystem::attr_bonus, StatusEffect, Slow};
use std::collections::HashMap;

pub struct EncumbranceSystem {}

impl<'a> System<'a> for EncumbranceSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, EquipmentChanged>,
        Entities<'a>,
        ReadStorage<'a, Item>,
        ReadStorage<'a, InBackpack>,
        ReadStorage<'a, Equipped>,
        WriteStorage<'a, Pools>,
        WriteStorage<'a, Attributes>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, AttributeBonus>,
        ReadStorage<'a, StatusEffect>,
        ReadStorage<'a, Slow>
    );

    fn run(&mut self, data : Self::SystemData) {
        let (mut equip_dirty, entities, items, backpacks, wielded,
            mut pools, mut attributes, player, attrbonus, statuses, slowed) = data;

        if equip_dirty.is_empty() { return; }

        struct ItemUpdate {
            weight : f32,
            initiative : f32,
            strength : i32,
            constitution : i32,
            dexterity : i32,
            intelligence : i32
        }

        // Build the map of who needs updating
        let mut to_update : HashMap<Entity, ItemUpdate> = HashMap::new(); // (weight, intiative)
        for (entity, _dirty) in (&entities, &equip_dirty).join() {
            to_update.insert(entity, ItemUpdate{ weight: 0.0, initiative: 0.0, strength: 0, constitution: 0, dexterity: 0, intelligence: 0 });
        }

        // Remove all dirty statements
        equip_dirty.clear();

        // Total up equipped items

        for (item, equipped, entity) in (&items, &wielded, &entities).join() {
            if to_update.contains_key(&equipped.owner) {
                let totals = to_update.get_mut(&equipped.owner).unwrap();
                totals.weight += item.weight_lbs;
                totals.initiative += item.initiative_penalty;
                if let Some(attr) = attrbonus.get(entity) {
                    totals.strength += attr.strength.unwrap_or(0);
                    totals.constitution += attr.constitution.unwrap_or(0);
                    totals.dexterity += attr.dexterity.unwrap_or(0);
                    totals.intelligence += attr.intelligence.unwrap_or(0);
                }
            }
        }

        // Total up carried items
        for (item, carried) in (&items, &backpacks).join() {
            if to_update.contains_key(&carried.owner) {
                let totals = to_update.get_mut(&carried.owner).unwrap();
                totals.weight += item.weight_lbs;
                totals.initiative += item.initiative_penalty;
            }
        }

        // Total up status effect modifiers
        for (status, attr) in (&statuses, &attrbonus).join() {
            if to_update.contains_key(&status.target) {
                let totals = to_update.get_mut(&status.target).unwrap();
                totals.strength += attr.strength.unwrap_or(0);
                totals.constitution += attr.constitution.unwrap_or(0);
                totals.dexterity += attr.dexterity.unwrap_or(0);
                totals.intelligence += attr.intelligence.unwrap_or(0);
            }
        }

        // Total up haste/slow
        for (status, slow) in (&statuses, &slowed).join() {
            if to_update.contains_key(&status.target) {
                let totals = to_update.get_mut(&status.target).unwrap();
                totals.initiative += slow.initiative_penalty;
            }
        }

        // Apply the data to Pools
        for (entity, item) in to_update.iter() {
            if let Some(pool) = pools.get_mut(*entity) {
                pool.total_weight = item.weight;
                pool.total_initiative_penalty = item.initiative;

                if let Some(attr) = attributes.get_mut(*entity) {
                    attr.strength.modifiers = item.strength;
                    attr.constitution.modifiers = item.constitution;
                    attr.dexterity.modifiers = item.dexterity;
                    attr.intelligence.modifiers = item.intelligence;
                    attr.strength.bonus = attr_bonus(attr.strength.base + attr.strength.modifiers);
                    attr.constitution.bonus = attr_bonus(attr.constitution.base + attr.constitution.modifiers);
                    attr.dexterity.bonus = attr_bonus(attr.dexterity.base + attr.dexterity.modifiers);
                    attr.intelligence.bonus = attr_bonus(attr.intelligence.base + attr.intelligence.modifiers);

                    let carry_capacity_lbs = (attr.strength.base + attr.strength.modifiers) * 15;
                    if pool.total_weight as i32 > carry_capacity_lbs {
                        // Overburdened
                        pool.total_initiative_penalty += 4.0;
                        if *entity == *player {
                            crate::gamelog::Logger::new()
                                .color(rltk::ORANGE)
                                .append("You are overburdened, and suffering an initiative penalty.")
                                .log();
                        }
                    }
                }
            }
        }
    }
}
