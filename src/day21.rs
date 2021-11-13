pub fn run() {
    let player = Character {
        hp: 100,
        dmg: 0,
        ac: 0,
    };
    let boss = Character {
        hp: 109,
        dmg: 8,
        ac: 2,
    };

    println!("part1: {}", lowest_winning_spend(&player, &boss));
    println!("part2: {}", biggest_losing_spend(&player, &boss));
}

const WEAPONS: [Item; 5] = [
    Item {
        cost: 8,
        dmg: 4,
        ac: 0,
    },
    Item {
        cost: 10,
        dmg: 5,
        ac: 0,
    },
    Item {
        cost: 25,
        dmg: 6,
        ac: 0,
    },
    Item {
        cost: 40,
        dmg: 7,
        ac: 0,
    },
    Item {
        cost: 74,
        dmg: 8,
        ac: 0,
    },
];

const ARMOR: [Item; 6] = [
    Item {
        cost: 0,
        dmg: 0,
        ac: 0,
    },
    Item {
        cost: 13,
        dmg: 0,
        ac: 1,
    },
    Item {
        cost: 31,
        dmg: 0,
        ac: 2,
    },
    Item {
        cost: 53,
        dmg: 0,
        ac: 3,
    },
    Item {
        cost: 75,
        dmg: 0,
        ac: 4,
    },
    Item {
        cost: 102,
        dmg: 0,
        ac: 5,
    },
];

const RINGS: [Item; 7] = [
    Item {
        cost: 0,
        dmg: 0,
        ac: 0,
    },
    Item {
        cost: 25,
        dmg: 1,
        ac: 0,
    },
    Item {
        cost: 50,
        dmg: 2,
        ac: 0,
    },
    Item {
        cost: 100,
        dmg: 3,
        ac: 0,
    },
    Item {
        cost: 20,
        dmg: 0,
        ac: 1,
    },
    Item {
        cost: 40,
        dmg: 0,
        ac: 2,
    },
    Item {
        cost: 80,
        dmg: 0,
        ac: 3,
    },
];

#[derive(Clone, Debug)]
struct Character {
    hp: i32,
    dmg: i32,
    ac: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Item {
    cost: i32,
    dmg: i32,
    ac: i32,
}

struct Loadout<'a> {
    weapon: &'a Item,
    armor: &'a Item,
    left_ring: &'a Item,
    right_ring: &'a Item,
}

impl<'a> Loadout<'a> {
    fn cost(&self) -> i32 {
        self.weapon.cost + self.armor.cost + self.left_ring.cost + self.right_ring.cost
    }

    fn dmg(&self) -> i32 {
        self.weapon.dmg + self.armor.dmg + self.left_ring.dmg + self.right_ring.dmg
    }

    fn ac(&self) -> i32 {
        self.weapon.ac + self.armor.ac + self.left_ring.ac + self.right_ring.ac
    }
}

fn get_loadouts<'a>() -> Vec<Loadout<'a>> {
    let mut result = vec![];
    for weapon in &WEAPONS {
        for armor in &ARMOR {
            for right_ring in &RINGS {
                for left_ring in &RINGS {
                    if left_ring != right_ring {
                        result.push(Loadout {
                            weapon,
                            armor,
                            left_ring,
                            right_ring,
                        });
                    }
                }
            }
        }
    }
    result
}

fn get_battle_result<'a>(
    loadout: &'a Loadout,
    player: &'a Character,
    boss: &'a Character,
) -> (i32, i32) {
    let mut buffed = player.clone();
    buffed.dmg += loadout.dmg();
    buffed.ac += loadout.ac();
    simulate_battle(&buffed, boss)
}

fn biggest_losing_spend(player: &Character, boss: &Character) -> i32 {
    get_loadouts()
        .iter()
        .filter(|&loadout| get_battle_result(loadout, player, boss).1 > 0)
        .map(|loadout| loadout.cost())
        .max()
        .unwrap()
}

fn lowest_winning_spend(player: &Character, boss: &Character) -> i32 {
    get_loadouts()
        .iter()
        .filter(|&loadout| get_battle_result(loadout, player, boss).0 > 0)
        .map(|loadout| loadout.cost())
        .min()
        .unwrap()
}

fn simulate_battle(char1: &Character, char2: &Character) -> (i32, i32) {
    let mut result = (char1.hp, char2.hp);

    let mut player1_turn = true;
    while result.0 > 0 && result.1 > 0 {
        if player1_turn {
            result.1 -= bounded_atk(char1.dmg - char2.ac);
            player1_turn = false;
        } else {
            result.0 -= bounded_atk(char2.dmg - char1.ac);
            player1_turn = true;
        }
    }

    result
}

fn bounded_atk(atk: i32) -> i32 {
    if atk < 1 {
        1
    } else {
        atk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let player = Character {
            hp: 8,
            dmg: 5,
            ac: 5,
        };

        let boss = Character {
            hp: 12,
            dmg: 7,
            ac: 2,
        };

        assert_eq!(simulate_battle(&player, &boss), (2, 0));
    }
}
