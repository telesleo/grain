use std::{any::{Any, TypeId}, collections::HashMap};

use crate::{ComponentSet, Entity, EntityId, EntityVersion, Global};

trait ErasedComponentSet: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove(&mut self, entity_id: EntityId);
}

impl<T: 'static> ErasedComponentSet for ComponentSet<T> {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn remove(&mut self, entity_id: EntityId) { self.remove(entity_id); }
}

pub struct Storage {
    next_entity_id: EntityId,
    available_entity_ids: Vec<EntityId>,
    entity_versions: Vec<EntityVersion>,
    component_sets: HashMap<TypeId, Box<dyn ErasedComponentSet>>,
    pub global: Global,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            next_entity_id: EntityId::MIN,
            available_entity_ids: Vec::new(),
            entity_versions: Vec::new(),
            component_sets: HashMap::new(),
            global: Global::new(),
        }
    }

    pub fn valid(&self, entity: Entity) -> bool {
        let id = entity.id();
        let version = entity.version();

        if id as usize >= self.entity_versions.len() {
            return false;
        }

        version == self.entity_versions[id as usize]
    }

    pub fn create(&mut self) -> Entity {
        if let Some(id) = self.available_entity_ids.pop() {
            let version = self.entity_versions[id as usize];

            Entity::new(id, version)
        } else {
            let id = self.next_entity_id;
            let version = EntityVersion::MIN;

            self.next_entity_id += 1;
            self.entity_versions.push(version);

            Entity::new(id, version)
        }
    }

    pub fn destroy(&mut self, entity: Entity) {
        let id = entity.id();

        for set in self.component_sets.values_mut() {
            set.remove(id);
        }

        self.available_entity_ids.push(id);
        self.entity_versions[id as usize] = self.entity_versions[id as usize].wrapping_add(1);
    }

    pub fn with<T: 'static>(&self) -> impl Iterator<Item = Entity> + '_ {
        let entities = self
            .get_set::<T>()
            .map(|set| set.entities())
            .unwrap_or(&[]);

        entities.iter().map(|&id| {
            let version = self.entity_versions[id as usize];
            Entity::new(id, version)
        })
    }

    pub fn contains<T: 'static>(&self, entity: Entity) -> bool {
        self.get_set::<T>()
            .map_or(false, |set| set.contains(entity.id()))
    }

    pub fn get<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.get_set::<T>()?.get(entity.id())
    }

    pub fn get_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.get_set_mut::<T>()?.get_mut(entity.id())
    }

    pub fn insert<T: 'static>(&mut self, entity: Entity, component: T) {
        self.get_or_insert_set_mut::<T>()
            .insert(entity.id(), component);
    }

    pub fn remove<T: 'static>(&mut self, entity: Entity) {
        if let Some(set) = self.get_set_mut::<T>() {
            set.remove(entity.id());
        }
    }

    fn get_set<T: 'static>(&self) -> Option<&ComponentSet<T>> {
        self.component_sets
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref()
    }

    fn get_set_mut<T: 'static>(&mut self) -> Option<&mut ComponentSet<T>> {
        self.component_sets
            .get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut()
    }

    fn get_or_insert_set_mut<T: 'static>(&mut self) -> &mut ComponentSet<T> {
        self.component_sets
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(ComponentSet::<T>::new()))
            .as_any_mut()
            .downcast_mut()
            .unwrap()
    }
}
