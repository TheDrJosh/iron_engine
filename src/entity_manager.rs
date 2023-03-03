use thiserror::Error;

use crate::Entity;

#[derive(Error, Debug)]
pub enum EntityManagementError {
    #[error("tryed to remove entity {0} which does not exist.")]
    RemovalError(Entity)
}


pub struct EntityManager {
    entitys: Vec<Entity>,
    next_entity: Entity,
}
impl EntityManager {
    pub fn new() -> Self {
        Self {
            entitys: Vec::new(),
            next_entity: 0,
        }
    }
    pub fn new_entity(&mut self) -> Entity {
        self.entitys.push(self.next_entity);
        self.next_entity += 1;
        return self.next_entity - 1;
    }

    pub fn remove_entity(&mut self, entity_id: Entity) -> Result<(), EntityManagementError>{
        //wrong need to find diffent method
        let index = match self.entitys.iter().find(|&&entity| entity == entity_id) {
            Some(i) => *i,
            None => {
                Err(EntityManagementError::RemovalError(entity_id))?
            },
        };
        self.entitys.remove(index);
        Ok(())
    }

    pub fn get_entitys(&self) -> Vec<Entity> {
        self.entitys.clone()
    }

    pub fn is_alive(&self, entity_id: Entity) -> bool {
        self.entitys.contains(&entity_id)
    }


}
