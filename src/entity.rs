use std::fmt;

pub type EntityId = u32;

pub type EntityVersion = u16;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Entity {
    id: EntityId,
    version: EntityVersion,
}

impl Entity {
    pub fn new(id: EntityId, version: EntityVersion) -> Self {
        Self {
            id,
            version,
        }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    pub fn version(&self) -> EntityVersion {
        self.version
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}v{}", self.id, self.version)
    }
}
