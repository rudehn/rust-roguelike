use specs::prelude::*;
use crate::{rng::roll_dice, Burning, DamageOverTime, Duration, EquipmentChanged, Initiative,
    MyTurn, Position, RunState, StatusEffect, DEFAULT_ACTION_COST, MOVE_ACTION_COST, ATTACK_ACTION_COST, Name};

pub struct InitiativeSystem {}

impl<'a> System<'a> for InitiativeSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteStorage<'a, Initiative>,
                        ReadStorage<'a, Position>,
                        WriteStorage<'a, MyTurn>,
                        Entities<'a>,
                        WriteExpect<'a, RunState>,
                        ReadExpect<'a, Entity>,
                        ReadExpect<'a, rltk::Point>,
                        WriteStorage<'a, Duration>,
                        WriteStorage<'a, EquipmentChanged>,
                        ReadStorage<'a, StatusEffect>,
                        ReadStorage<'a, DamageOverTime>,
                        ReadStorage<'a, Burning>,
                        ReadStorage<'a, Name>
                    );

    fn run(&mut self, data : Self::SystemData) {
        let (mut initiatives, positions, mut turns, entities,
            mut runstate, player, player_pos, mut durations, mut dirty,
            statuses, dots, burning, names) = data;

        if *runstate != RunState::Ticking { return; }

        // Clear any remaining MyTurn we left by mistkae
        turns.clear();

        // Roll initiative
        for (entity, initiative, pos) in (&entities, &mut initiatives, &positions).join() {
            initiative.current += initiative.energy_gain;
            let entity_name = names.get(entity);
            if let Some(entity_name) = entity_name {
                println!("{} has {} energy", entity_name.name, initiative.current);
            }
            else {
                println!("unknown has {} energy", initiative.current);
            }
            // Several options here
            // 1. It's the players turn
            // 2. It's the enemies turn, but they are too far away, so don't process them
            // 3. It's the enemies turn and they are nearby
            if initiative.current >= 0 {
                let mut myturn = true;

                // If its the player, we want to go to an AwaitingInput state
                if entity == *player {
                    // Give control to the player
                    *runstate = RunState::AwaitingInput;
                }
                //  else {
                //     let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_pos, rltk::Point::new(pos.x, pos.y));
                //     if distance > 20.0 {
                //         myturn = false;
                //     }
                // }

                // It's my turn!
                if myturn {
                    turns.insert(entity, MyTurn{}).expect("Unable to insert turn");
                }

            }
        }

        // Handle durations
        if *runstate == RunState::AwaitingInput {
            use crate::effects::*;
            for (effect_entity, duration, status) in (&entities, &mut durations, (&statuses).maybe()).join() {
                // Status exists
                if let Some(status) = status{
                    if entities.is_alive(status.target) {
                        duration.turns -= 1;
                        if let Some(dot) = dots.get(effect_entity) {
                            add_effect(
                                None, 
                                EffectType::Damage{ amount : dot.damage }, 
                                Targets::Single{ target : status.target 
                                }
                            );
                        }
                        if let Some(_burn) = burning.get(effect_entity) {
                            // Roll for burn damage
                            let burn_damage = roll_dice(1, 3);
                            add_effect(
                                None, 
                                EffectType::Damage{ amount : burn_damage }, 
                                Targets::Single{ target : status.target 
                                }
                            );
                        }
                        if duration.turns < 1 {
                            dirty.insert(status.target, EquipmentChanged{}).expect("Unable to insert");
                            entities.delete(effect_entity).expect("Unable to delete");
                        }
                    }
                } else {
                    // Currently the only other flow to get here is fire effects on the map that have a duration
                    duration.turns -= 1;
                    if duration.turns < 1 {
                        entities.delete(effect_entity).expect("Unable to delete");
                    }
                }

            }
        }
    }
}

pub fn apply_move_action_cost(initiative: &mut Initiative) {
    println!("move before: {}", initiative.current);
    initiative.current -= (MOVE_ACTION_COST as f32 * initiative.move_action_mult).round() as i32;
    println!("move after: {}", initiative.current);
}


pub fn apply_attack_action_cost(initiative: &mut Initiative) {
    println!("attack before: {}", initiative.current);
    initiative.current -= (ATTACK_ACTION_COST as f32 * initiative.attack_action_mult).round() as i32;
    println!("attack after: {}", initiative.current);
}

pub fn apply_generic_action_cost(initiative: &mut Initiative) {
    println!("generic before: {}", initiative.current);
    initiative.current -= DEFAULT_ACTION_COST;
    println!("generic after: {}", initiative.current);
}