use crate::EntityId;

pub struct ComponentSet<T> {
    sparse: Vec<EntityId>,
    dense: Vec<EntityId>,
    data: Vec<T>,
}

impl<T> ComponentSet<T> {
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn entities(&self) -> &[EntityId] {
        &self.dense
    }

    pub fn contains(&self, entity_id: EntityId) -> bool {
        let sparse_index = entity_id as usize;

        if sparse_index >= self.sparse.len() {
            return false;
        }

        self.sparse[sparse_index] != EntityId::MAX
    }

    pub fn get(&self, entity_id: EntityId) -> Option<&T> {
        let sparse_index = entity_id as usize;

        if sparse_index >= self.sparse.len() {
            return None;
        }

        let sparse_value = self.sparse[sparse_index];

        if sparse_value == EntityId::MAX {
            return None;
        }

        let dense_index = sparse_value as usize;

        self.data.get(dense_index)
    }

    pub fn get_mut(&mut self, entity_id: EntityId) -> Option<&mut T> {
        let sparse_index = entity_id as usize;

        if sparse_index >= self.sparse.len() {
            return None;
        }

        let sparse_value = self.sparse[sparse_index];

        if sparse_value == EntityId::MAX {
            return None;
        }

        let dense_index = sparse_value as usize;

        self.data.get_mut(dense_index)
    }

    pub fn insert(&mut self, entity_id: EntityId, component: T) {
        let sparse_index = entity_id as usize;

        if sparse_index >= self.sparse.len() {
            self.sparse.resize(sparse_index + 1, EntityId::MAX);
        }

        let sparse_value = self.sparse[sparse_index];

        if sparse_value == EntityId::MAX {
            self.sparse[sparse_index] = self.dense.len() as EntityId;
            self.dense.push(entity_id);
            self.data.push(component);
            return;
        }

        let dense_index = sparse_value as usize;
        self.data[dense_index] = component;
    }

    pub fn remove(&mut self, entity_id: EntityId) {
        let sparse_index = entity_id as usize;

        if sparse_index >= self.sparse.len() {
            return;
        }

        let sparse_value = self.sparse[sparse_index];

        if sparse_value == EntityId::MAX {
            return;
        }

        let dense_index = sparse_value as usize;

        let other_dense_index = self.dense.len() - 1;
        let other_dense_value = self.dense[other_dense_index];

        if dense_index != other_dense_index {
            self.sparse[other_dense_value as usize] = dense_index as EntityId;
        }

        self.sparse[sparse_index] = EntityId::MAX;

        self.dense.swap(dense_index, other_dense_index);
        self.data.swap(dense_index, other_dense_index);

        self.dense.pop();
        self.data.pop();
    }
}
