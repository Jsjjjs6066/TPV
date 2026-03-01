use crate::btmd;

use std::{
    marker::PhantomData,
    sync::{Arc, RwLock},
};

use btmd::element::Element;

#[derive(Clone, Debug, Default)]
pub struct AddingElements;
#[derive(Clone, Debug, Default)]
pub struct Finished;

#[derive(Clone, Debug, Default)]
pub struct HoveredVec<State = Finished> {
    vec: Vec<Arc<RwLock<Element>>>,
    marker: PhantomData<State>,
}

impl HoveredVec {
    pub fn new() -> HoveredVec<AddingElements> {
        HoveredVec {
            vec: Vec::new(),
            marker: PhantomData::<AddingElements>,
        }
    }

    pub fn get_vec(&self) -> &Vec<Arc<RwLock<Element>>> {
        &self.vec
    }
}

impl HoveredVec<AddingElements> {
    pub fn add_element(&mut self, element: Arc<RwLock<Element>>) {
        self.vec.push(element);
    }

    pub fn finish(&self) -> HoveredVec<Finished> {
        HoveredVec {
            vec: self.vec.to_owned(),
            marker: PhantomData::<Finished>,
        }
    }
}
