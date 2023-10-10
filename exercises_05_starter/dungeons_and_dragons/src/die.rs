use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

/// Takes in a max value and returns a random number between 1 and max
// DO NOT MODIFY
fn get_random_value(max: u8) -> u8 {
    let mut rng = ChaCha20Rng::seed_from_u64(2);
    rng.gen_range(1..=max)
}

pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

pub struct Coin;

// MODIFY/ADD BELOW HERE ONLY

pub fn roll<T>(item: T) -> u8
where
    T: RollDice,
{
    item.get_value()
}

pub trait RollDice {
    fn get_value(&self) -> u8;
}

impl RollDice for Die {
    fn get_value(&self) -> u8 {
        match self {
            Die::D4 => get_random_value(4),
            Die::D6 => get_random_value(6),
            Die::D8 => get_random_value(8),
            Die::D10 => get_random_value(10),
            Die::D12 => get_random_value(12),
            Die::D20 => get_random_value(20),
        }
    }
}

impl RollDice for Coin {
    fn get_value(&self) -> u8 {
        get_random_value(2)
    }
}
