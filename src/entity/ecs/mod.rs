pub mod types;

use {
    crate::{
        entity::{
            self,
            component::{self, health, physics, Component},
            Entity,
        },
        error::ZapataError,
        physics::{hitbox, vec3::Vec3},
        scene::Scene,
    },
    std::{
        fmt::{Debug, Display, Formatter},
        marker::PhantomData,
        sync::Mutex,
    },
};

pub const DEFAULT_MAX_ENTITY: usize = 1000;

struct OccupiedComponent<T>
where
    T: Component,
{
    component: T,
    generation: entity::Generation,
}

enum ComponentEntry<T>
where
    T: Component,
{
    Occupied(OccupiedComponent<T>),
    Free,
}

impl<T> ComponentEntry<T>
where
    T: Component,
{
    fn update(&mut self, self_entity: Entity) -> Result<(), ZapataError> {
        match self {
            ComponentEntry::Occupied(occupied) if occupied.generation == self_entity.generation => {
                occupied.component.update(self_entity)
            }
            _ => Ok(()),
        }
    }
}

impl<T> Debug for ComponentEntry<T>
where
    T: Component + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentEntry::Occupied(occupied) => f
                .write_fmt(format_args!(
                    "\n@{}: {:?}",
                    occupied.generation, occupied.component
                ))
                .expect("ComponentEntry debug string failed"),
            ComponentEntry::Free => f
                .write_str("None")
                .expect("ComponentEntry is free but couldn't write debug string"),
        }

        f.write_str("")
    }
}

struct ComponentArray<T>
where
    T: Component,
{
    collection: Vec<ComponentEntry<T>>,
}

impl<T> ComponentArray<T>
where
    T: Component,
{
    pub fn get(&self, entity: &Entity) -> Option<&T> {
        if let Some(entry) = self.collection.get(entity.index) {
            match entry {
                ComponentEntry::Occupied(occupied) if occupied.generation == entity.generation => {
                    return Some(&occupied.component)
                }
                _ => (),
            }
        }
        None
    }

    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        if let Some(entry) = self.collection.get_mut(entity.index) {
            match entry {
                ComponentEntry::Occupied(occupied) if occupied.generation == entity.generation => {
                    return Some(&mut occupied.component);
                }
                _ => (),
            }
        }
        None
    }

    pub fn fill_new_entity(
        &mut self,
        entity: &Entity,
        component: T,
    ) -> Result<ComponentEntry<T>, ZapataError> {
        if let Some(entry) = self.collection.get(entity.index) {
            match entry {
                ComponentEntry::Occupied(occupied) if occupied.generation > entity.generation => return Err(ZapataError::RuntimeError(format!("[{}]Component @ {} being replaced by {:?} is now filled with higher generation entry.", component.name(), entity.index, entity))),
                _ => (),
            }
        }
        Ok(std::mem::replace(
            &mut self.collection[entity.index],
            ComponentEntry::Occupied(OccupiedComponent {
                generation: entity.generation,
                component,
            }),
        ))
    }

    pub fn push(&mut self, entry: ComponentEntry<T>) {
        self.collection.push(entry)
    }

    pub fn new(capacity: usize) -> Self {
        Self {
            collection: Vec::with_capacity(capacity),
        }
    }
}

// 4/13/23 TODO: I think in the future this should use a ComponentRegistry type system.
//          My thinking is that with a registry system, all of the components wouldn't need to be allocated/present.
//          For making one 'game' that might work out alright, but hypothetically if this engine were to ever grow &
//          make multiple kinds of games/systems, then it might hurt having every type of Component that exists
//          loaded/present during *every* runtime. However, the Registry needs to be essentially seamless for intertwining with the ECS.
//          Ideally, all of the create_* fn's in ecs/types.rs return the same expected behavior as before. - Justin

pub struct ECS {
    max: entity::Index,
    current_entity: entity::Index,
    entities: Vec<Entity>,

    physics: ComponentArray<component::Physics>,
    collider: ComponentArray<component::Collider>,
    health: ComponentArray<component::Health>,
}

impl ECS {
    fn get_next_entity(&mut self) -> Result<Entity, ZapataError> {
        if self.current_entity == self.max {
            return Err(ZapataError::RuntimeError(format!(
                "ECS is at max entities. Couldn't get next entity."
            )));
        }
        /* TODO: In the future, we'll want to be able to periodically go through each ComponentEntry list &
         *  find the Free entries. Adding them to a "free_queue" of some kind is a decent way to do it that I can
         *  think of right now. */
        let e = self.current_entity;

        self.physics.push(ComponentEntry::Free);
        self.collider.push(ComponentEntry::Free);
        self.health.push(ComponentEntry::Free);

        self.current_entity += 1;
        Ok(Entity {
            index: e,
            generation: 0,
        })
    }

    pub fn do_updates(&mut self) -> Result<(), ZapataError> {
        for entity in &self.entities {
            if let Some(physx) = self.physics.get_mut(entity) {
                if let Err(e) = physx.update(entity.clone()) {
                    return Err(e);
                }
            }

            if let Some(collider) = self.collider.get_mut(entity) {
                if let Err(e) = collider.update(entity.clone()) {
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}

impl ECS {
    pub fn new(max_entities: usize) -> Self {
        Self {
            max: max_entities,
            current_entity: 0,
            entities: Vec::new(),
            physics: ComponentArray::new(max_entities),
            collider: ComponentArray::new(max_entities),
            health: ComponentArray::new(max_entities),
        }
    }
}

impl Debug for ECS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for entity in &self.entities {
            f.write_str("\n")?;
            f.write_fmt(format_args!("E#{}@{}", entity.index, entity.generation))
                .expect("ECS debug string failed");

            if let Some(physx) = self.physics.get(entity) {
                f.write_fmt(format_args!("\n{}: {:?}", physx.name(), physx))
                    .expect(format!("ECS failed to debug physx for {}", entity).as_str());
            }

            if let Some(collider) = self.collider.get(entity) {
                f.write_fmt(format_args!("\n{}: {:?}", collider.name(), collider))
                    .expect(format!("ECS failed to debug collider for {}", entity).as_str())
            }

            if let Some(health) = self.health.get(entity) {
                f.write_fmt(format_args!("\n{}: {:?}", health.name(), health))
                    .expect(format!("ECS failed to debug health for {}", entity).as_str())
            }

            f.write_str("\n")?
        }
        f.write_str("done")
    }
}
