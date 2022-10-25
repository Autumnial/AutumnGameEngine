use engine_logger::*;
use std::cell::{RefCell, RefMut};

//todo better way of querying entities
//todo deleting entities
//todo removing components



pub struct World{
    entities: usize,
    free_ids: Vec<usize>,
    components: Vec<Box<dyn ComponentVec>>
}

impl World {
    pub fn new() -> Self{
        Self { entities: 0, free_ids: Vec::new(),  components: Vec::new() }
    }

    pub fn update(&mut self){
        //do stuff
    }

    pub fn add_entity(&mut self) -> usize{
        let id;
        if !self.free_ids.is_empty() {
            id = self.free_ids.pop().unwrap();
        } else {
            id = self.entities;
            self.entities += 1;

            for component_vec in self.components.iter_mut(){
                component_vec.push_none()
            };
        }
        id
    }

    pub fn destroy_entity(&mut self, entity: usize){
        let component_arrays = &mut self.components;

        for array in component_arrays{
            array.set_none(entity);
        }

        self.free_ids.push(entity);
    }

    pub fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType
    ){
        for component_vec in self.components.iter_mut(){
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
                {
                    component_vec.get_mut()[entity] = Some(component);
                    return;
            }
        }

        let mut new_component_vec: Vec<Option<ComponentType>> = Vec::with_capacity(self.entities);

        for _ in 0..self.entities{
            new_component_vec.push(None)
        }

        new_component_vec[entity] = Some(component);
        self.components.push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn remove_component_from_entity<ComponentType: 'static>(
        &mut self,
        entity: usize 
    ){
        for component_vec in self.components.iter_mut(){
            if let Some(component_vec) = component_vec
            .as_any_mut()
            .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>(){
                component_vec.get_mut()[entity] = None;
                return;
            }
        }

        warning!("entity does not have component {}", std::any::type_name::<ComponentType>());
    }

    pub fn borrow_component_vec<ComponentType: 'static>(&self) -> RefMut<Vec<Option<ComponentType>>>{

        for component_vec in self.components.iter(){
            if let Some(component_vec) = component_vec
            .as_any()
            .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>(){
                return component_vec.borrow_mut();
            }
        }
        error!("Component vector does not exist");

    }


}

trait ComponentVec{
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
    fn set_none(&mut self, index: usize);
}


impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>>{

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None);
    }

    fn set_none(&mut self, index: usize) {
        self.get_mut()[index] = None;
    }
}

