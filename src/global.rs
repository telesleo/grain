use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct Global {
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl Global {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn contains<T: 'static>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }

    pub fn insert<T: 'static>(&mut self, component: T) {
        self.components.insert(TypeId::of::<T>(), Box::new(component));
    }

    pub fn remove<T: 'static>(&mut self) {
        self.components.remove(&TypeId::of::<T>());
    }
}
