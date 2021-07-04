#[derive(Debug)]
pub struct Attack {
    pub name: String,
    pub damage: i32,
    pub power: i32,
    pub self_damage: i32,
}

#[derive(Debug)]
pub struct Pokemon {
    pub name: String,
    pub health: i32,
    pub accuracy: i32,
    pub combat_power: i32,
    pub attacks: [Attack; 4],
}

impl Pokemon {
    pub fn attack(self, attack_index: usize) -> i32 {
        let attack = &self.attacks[attack_index];
        let damage: i32 = (attack.damage * self.combat_power / 100) as i32;
        return damage;
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health = self.health - damage;
    }
}

pub fn thunderbolt() -> Attack {
    Attack {
        name: String::from("Thunderbolt"),
        damage: 60,
        power: 90,
        self_damage: 1,
    }
}

pub fn quick_attack() -> Attack {
    Attack {
        name: String::from("Quick Attack"),
        damage: 30,
        power: 70,
        self_damage: 15,
    }
}

pub fn iron_tail() -> Attack {
    Attack {
        name: String::from("Iron Tail"),
        damage: 100,
        power: 100,
        self_damage: 60,
    }
}

pub fn mega_punch() -> Attack {
    Attack {
        name: String::from("Mega Punch"),
        damage: 80,
        power: 85,
        self_damage: 30,
    }
}
