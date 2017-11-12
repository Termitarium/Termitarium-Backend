#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    // TODO: make fields private?
    pub entities: Vec<Entity>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    // TODO: make fields private?
    pub id: EntityId,
    pub name: EntityName
}

impl Entity {
    fn new(id: EntityId, name: EntityName) -> Self {
        Entity {id, name}
    }
}

pub type EntityId = String;
pub type EntityName = String;