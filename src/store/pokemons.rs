#[path = "./utils.rs"]
mod utils;

pub fn charmendar() -> utils::Pokemon {
    let mut entity = utils::Pokemon {
        name: String::from("Charmendar"),
        health: 120,
        accuracy: 90,
        combat_power: 95,
        attacks: [
            utils::thunderbolt(),
            utils::quick_attack(),
            utils::iron_tail(),
            utils::mega_punch(),
        ],
    };

    return entity;
}
pub fn pikachu() -> utils::Pokemon {
    let mut entity = utils::Pokemon {
        name: String::from("Pikachu"),
        health: 150,
        accuracy: 90,
        combat_power: 95,
        attacks: [
            utils::thunderbolt(),
            utils::quick_attack(),
            utils::iron_tail(),
            utils::mega_punch(),
        ],
    };

    return entity;
}
