use crate::Entity;

pub trait Component {
    fn start(&mut self, parent: &Entity/*, &mut GameState*/);
    fn update(&mut self, parent: &Entity/*, &mut GameState*/);
}