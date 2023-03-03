use std::{rc::{Rc, Weak}, cell::RefCell};

use thiserror::Error;

use crate::{component::Component, Entity};


#[derive(Error, Debug)]
pub enum ComponentManagementError {
    #[error("tryed to remove component {0} on entity {1} which does not exist.")]
    RemovalError(ComponentId, Entity),
    #[error("Tried to use a dropped component")]
    InvalidComponent
}


pub type ComponentId = usize;

pub struct ComponentContainer {
    id: ComponentId,
    parent: Entity,
    pub component: Box<dyn Component>,
}



struct ComponentManager {
    components: Vec<Rc<RefCell<ComponentContainer>>>,
    next_component: ComponentId,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            next_component: 0,
        }
    }
    pub fn add_component(&mut self, entity: Entity, component: Box<dyn Component>) -> ComponentId {
        let id = self.next_component;
        let cc = Rc::new(RefCell::new(ComponentContainer {
            id,
            parent: entity,
            component,
        }));
        self.components.push(cc.clone());
        self.next_component += 1;
        return id;
    }

    pub fn get_component(&mut self, entity: Entity, component: ComponentId) -> Option<Weak<RefCell<ComponentContainer>>> {
        
        None
    }

    pub fn remove_component(&mut self, entity: Entity, component: Weak<RefCell<ComponentContainer>>) -> Result<(), ComponentManagementError> {
        self.remove_component_by_id(entity, match component.upgrade() {
            Some(component) => Ok(component.borrow().id),
            None => Err(ComponentManagementError::InvalidComponent),
        }?)
    }
    pub fn remove_component_by_id(&mut self, entity: Entity, component_id: ComponentId) -> Result<(), ComponentManagementError> {

        let index = self.components.iter()(|&component| {
            component.borrow().id == component_id
        });

        self.components.remove(index)

        return Err(ComponentManagementError::RemovalError(component_id, entity));

        Ok(())
    }




}
