use crate::carddata::CardData;
use std::rc::Rc;

#[derive(Debug)]
pub struct Summoner {
    type_object: Rc<CardData>,
}

impl Summoner {
    pub fn new(type_object: &Rc<CardData>) -> Self {
        Self {
            type_object: Rc::clone(type_object),
        }
    }
}