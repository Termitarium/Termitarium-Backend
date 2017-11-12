use super::{EntityId, EntityName, EntityType};

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
    EntityOperation(EntityOperation)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EntityOperation {
    NewEntity{id: EntityId, name: EntityName, entity_type: EntityType}
}