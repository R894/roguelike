use bevy::prelude::*;

use crate::board::components::Wall;
use crate::board::{components::Position, CurrentBoard};
use crate::pieces::components::{Health, Melee, Occupier};
use crate::vectors::Vector2Int;

use super::Action;

pub struct DamageAction(pub Entity, pub u32);
impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let Some(mut health) = world.get_mut::<Health>(self.0) else {
            return Err(());
        };
        health.value = health.value.saturating_sub(self.1);
        if health.value == 0 {
            // the unit is killed
            despawn_children(world, self.0);
            world.despawn(self.0);
        }
        Ok(Vec::new())
    }
}

/// despawns all children of an entity
///
/// does not despawn the entity itself
fn despawn_children(world: &mut World, entity: Entity) {
    let children_to_despawn: Vec<Entity> = if let Some(children) = world.get::<Children>(entity) {
        children.iter().cloned().collect()
    } else {
        Vec::new()
    };

    for child in children_to_despawn {
        world.despawn(child);
    }
}

pub struct MeleeHitAction {
    pub attacker: Entity,
    pub target: Vector2Int,
    pub damage: u32,
}
impl Action for MeleeHitAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let attacker_position = world.get::<Position>(self.attacker).ok_or(())?;
        if attacker_position.v.manhattan(self.target) > 1 {
            return Err(());
        };
        let target_entities = world
            .query_filtered::<(Entity, &Position), With<Health>>()
            .iter(world)
            .filter(|(_, p)| p.v == self.target)
            .collect::<Vec<_>>();
        if target_entities.is_empty() {
            return Err(());
        };
        let result = target_entities
            .iter()
            .map(|e| Box::new(DamageAction(e.0, self.damage)) as Box<dyn Action>)
            .collect::<Vec<_>>();
        Ok(result)
    }
}

pub struct WalkAction(pub Entity, pub Vector2Int);
impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let board = world.get_resource::<CurrentBoard>().ok_or(())?;
        if !board.tiles.contains_key(&self.1) {
            return Err(());
        };
        if world
            .query_filtered::<&Position, With<Occupier>>()
            .iter(world)
            .any(|p| p.v == self.1)
        {
            let Some(melee) = world.get::<Melee>(self.0) else {
                return Err(());
            };
            return Ok(vec![Box::new(MeleeHitAction {
                attacker: self.0,
                target: self.1,
                damage: melee.damage,
            })]);
        };
        let mut position = world.get_mut::<Position>(self.0).ok_or(())?;
        position.v = self.1;
        Ok(Vec::new())
    }
}

pub struct DigAction(pub Entity, pub Vector2Int);
impl Action for DigAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let wall_entity = world
            .query_filtered::<(Entity, &Position), With<Wall>>()
            .iter(world)
            .find_map(|(entity, position)| {
                if position.v == self.1 {
                    Some(entity)
                } else {
                    None
                }
            });
        if let Some(wall_entity) = wall_entity {
            despawn_children(world, wall_entity);
            world.despawn(wall_entity);
        } else {
            return Err(());
        }

        Ok(Vec::new())
    }
}
