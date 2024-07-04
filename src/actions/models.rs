use bevy::prelude::*;

use crate::board::components::Wall;
use crate::board::{components::Position, CurrentBoard};
use crate::pieces::components::{Gold, Health, Occupier, Piece, Portal};
use crate::player::Player;
use crate::vectors::Vector2Int;

use super::{Action, GameOverEvent, NextLevelEvent};

pub struct DamageAction(pub Entity, pub u32);
impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let Some(mut health) = world.get_mut::<Health>(self.0) else {
            return Err(());
        };
        health.value = health.value.saturating_sub(self.1);
        if health.value == 0 {
            // the unit is killed
            // if its a player send the gameover event
            if world.get::<Player>(self.0).is_some() {
                world.send_event(GameOverEvent);
            }
            despawn_children(world, self.0);
            world.despawn(self.0);
        }
        Ok(Vec::new())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
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

/// despawns an entity and all its children
pub fn despawn_recursive(world: &mut World, entity: Entity) {
    despawn_children(world, entity);
    world.despawn(entity);
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
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
            return Err(());
        };

        let mut position = world.get_mut::<Position>(self.0).ok_or(())?;
        position.v = self.1;

        let pickup_action = Box::new(PickupAction(self.0, position.v));
        let next_level_action = Box::new(NextLevelAction(self.0, position.v));
        Ok(vec![pickup_action, next_level_action])
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct DigAction(pub Entity, pub Vector2Int);
impl Action for DigAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let wall_entity = world
            .query_filtered::<(Entity, &Position, &Occupier), With<Wall>>()
            .iter(world)
            .find_map(|(entity, position, _)| {
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct PickupAction(pub Entity, pub Vector2Int);
impl Action for PickupAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let target_gold = world
            .query_filtered::<(Entity, &Gold, &Position), (With<Piece>, Without<Player>)>()
            .iter(world)
            .filter(|(_, _, p)| p.v == self.1)
            .map(|(e, g, _)| (e, g.value))
            .collect::<Vec<_>>();

        if target_gold.is_empty() {
            return Err(());
        }
        let mut player_gold = world.get_mut::<Gold>(self.0).ok_or(())?;
        player_gold.value += target_gold[0].1;
        despawn_recursive(world, target_gold[0].0);
        Ok(Vec::new())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct NextLevelAction(pub Entity, pub Vector2Int);
impl Action for NextLevelAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let target_portal = world
            .query_filtered::<(Entity, &Position), With<Portal>>()
            .iter(world)
            .filter(|(_, p)| p.v == self.1)
            .map(|(e, _)| e)
            .collect::<Vec<_>>();

        if target_portal.is_empty() {
            return Err(());
        }

        world.send_event(NextLevelEvent);
        Ok(Vec::new())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
