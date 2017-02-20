use std::collections::BTreeMap;
use sfml::window::Key;

pub type KeyManager = BTreeMap<Key, Action>;

pub enum Action {
    SpawnShip(usize)
}