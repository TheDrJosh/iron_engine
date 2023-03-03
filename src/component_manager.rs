use std::{rc::{Rc, Weak}, cell::RefCell};

use crate::{component::Component, Entity};

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
    pub fn add_component(&mut self, entity: Entity, component: Box<dyn Component>) -> Weak<RefCell<ComponentContainer>> {
        let id = self.next_component;
        let cc = Rc::new(RefCell::new(ComponentContainer {
            id,
            parent: entity,
            component,
        }));
        self.components.push(cc.clone());
        self.next_component += 1;
        return Rc::downgrade(&cc);
    }

    pub fn remove_component(&mut self, entity: Entity, component: Weak<RefCell<ComponentContainer>>) {

    }
    pub fn remove_component_by_id(&mut self, entity: Entity, component: ComponentIds) {

    }


}
