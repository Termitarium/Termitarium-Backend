use super::{EntityId, EntityName, EntityType};

pub enum Operation {
    EntityOperation(EntityOperation)
}

pub enum EntityOperation {
    NewEntity{id: EntityId, name: EntityName, entity_type: EntityType}
}