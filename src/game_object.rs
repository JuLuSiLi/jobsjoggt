use crate::components::*;

pub struct GameObject<'a> {
    pub name: String,
    components: Vec<Box<dyn 'a + Component>>,
}

impl<'a> GameObject<'a> {
    pub fn new(name: &str) -> GameObject {
        GameObject {
            name: name.to_string(),
            components: Vec::new(),
        }
    }

    pub fn add_component<T: 'a + Component>(&mut self, mut component: T) {
        component.init();
        self.components.push(Box::new(component));
    }

    pub fn update(&mut self) {
        for component in self.components.iter_mut() {
            component.update();
        }
    }
}