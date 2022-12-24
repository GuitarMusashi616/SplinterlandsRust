// unique key for a monster
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum MonsterKey {
    Home(u8),
    Oppo(u8),
}
