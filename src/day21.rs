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

    println!("part1: {}", lowest_winning_spend(player, boss));
}

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

fn lowest_winning_spend(player: Character, boss: Character) -> i32 {
    let weapons = vec![
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
    let armors = vec![
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
    let rings = vec![
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

    let mut loadouts = vec![];
    for weapon in &weapons {
        for armor in &armors {
            for right_ring in &rings {
                for left_ring in &rings {
                    if left_ring != right_ring {
                        loadouts.push([weapon, armor, right_ring, left_ring]);
                    }
                }
            }
        }
    }

    loadouts
        .iter()
        .filter(|loadout| {
            let mut buffed = player.clone();
            buffed.dmg += loadout[0].dmg + loadout[1].dmg + loadout[2].dmg + loadout[3].dmg;
            buffed.ac += loadout[0].ac + loadout[1].ac + loadout[2].ac + loadout[3].ac;
            let battle_result = simulate_battle(&buffed, &boss);
            battle_result.0 > 0
        })
        .map(|loadout| loadout[0].cost + loadout[1].cost + loadout[2].cost + loadout[3].cost)
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
