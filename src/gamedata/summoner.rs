use crate::cardparse::carddata::CardData;

#[derive(Debug)]
pub struct Summoner<'a> {
    type_object: &'a CardData,
}

impl<'a> Summoner<'a> {
    pub fn new(type_object: &'a CardData) -> Self {
        Self {
            type_object,
        }
    }
}