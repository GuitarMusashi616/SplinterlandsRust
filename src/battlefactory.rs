use crate::registry::Registry;
use crate::battleproxy::BattleProxy;
use std::rc::Rc;

#[derive(Debug)]
pub struct BattleFactory {
    registry: Rc<Registry>,
}

impl BattleFactory {
    pub fn new(registry: &Rc<Registry>) -> Self {
        Self {
            registry: Rc::clone(registry),
        }
    }

    pub fn from(filename: &str) -> Self {
        Self::new(&Registry::from(filename))
    }

    pub fn create<'a>(&'a self, home: &'a[&'a str], oppo: &'a[&'a str]) -> BattleProxy {
        BattleProxy::new(&self.registry, home, oppo)
    }
}