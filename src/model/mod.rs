#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    // TODO: make fields private?
    pub entities: Vec<Entity>,
    pub entity_types: Vec<EntityType>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    // TODO: make fields private?
    pub id: EntityId,
    pub entity_type: EntityType,
    pub name: EntityName
}

impl Entity {
    // TODO: builder pattern here
    fn new(id: EntityId, entity_type: EntityType, name: EntityName) -> Self {
        Entity {id, entity_type, name}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityType {
    pub id: EntityTypeId,
    pub name: EntityTypeName
}

pub type EntityId = String;
pub type EntityName = String;

pub type EntityTypeId = String;
pub type EntityTypeName = String;