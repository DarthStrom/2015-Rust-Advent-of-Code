use std::cmp::max;

pub fn run() {
    let player = Player {
        hp: 50,
        armor: 0,
        mana: 500,
    };
    let boss = Boss { hp: 51, dmg: 9 };

    // println!("part1: {}", lowest_winning_spend(&player, &boss));
    // println!("part2: {}", biggest_losing_spend(&player, &boss));
}

enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Player {
    hp: i32,
    armor: i32,
    mana: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Boss {
    hp: i32,
    dmg: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Effects {
    shield: i32,
    poison: i32,
    recharge: i32,
}

impl Effects {
    fn apply(self, spell: Spell) -> Effects {
        match spell {
            Spell::Shield => Effects { shield: 6, ..self },
            Spell::Poison => Effects { poison: 6, ..self },
            Spell::Recharge => Effects {
                recharge: 5,
                ..self
            },
            _ => self,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Game {
    player: Player,
    boss: Boss,
    effects: Effects,
}

impl Game {
    fn new(player: Player, boss: Boss) -> Game {
        Game {
            player,
            boss,
            effects: Effects::default(),
        }
    }
}

fn player_turn(game: Game, spell: Spell) -> Game {
    let mut player = game.player;
    let mut boss = game.boss;
    let mut effects = game.effects;

    player.armor = 0;
    match spell {
        Spell::MagicMissile => {
            player.mana -= 53;
            boss.hp -= 4;
        }
        Spell::Drain => {
            player.mana -= 73;
            boss.hp -= 2;
            player.hp += 2;
        }
        Spell::Shield => {
            player.mana -= 113;
            player.armor = 7;
        }
        Spell::Poison => {
            player.mana -= 173;
        }
        Spell::Recharge => {
            player.mana -= 229;
        }
    }

    apply_effects(&mut player, &mut boss, &mut effects);

    effects = effects.apply(spell);

    Game {
        player,
        boss,
        effects,
    }
}

fn boss_turn(game: Game) -> Game {
    let mut player = game.player;
    let mut boss = game.boss;
    let mut effects = game.effects;

    apply_effects(&mut player, &mut boss, &mut effects);

    if boss.hp > 0 {
        player.hp -= max(0, boss.dmg - player.armor);
    }

    Game {
        player,
        boss,
        effects,
    }
}

fn apply_effects(player: &mut Player, boss: &mut Boss, effects: &mut Effects) {
    if effects.shield > 0 {
        effects.shield -= 1;
        if effects.shield == 0 {
            player.armor = 0;
        } else {
            player.armor = 7;
        }
    }
    if effects.poison > 0 {
        effects.poison -= 1;
        boss.hp -= 3;
    }
    if effects.recharge > 0 {
        effects.recharge -= 1;
        player.mana += 101;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let player = Player {
            hp: 10,
            armor: 0,
            mana: 250,
        };
        let boss = Boss { hp: 13, dmg: 8 };
        let mut game = Game::new(player, boss);

        game = player_turn(game, Spell::Poison);
        assert_eq!(
            game,
            Game {
                player: Player {
                    hp: 10,
                    armor: 0,
                    mana: 77
                },
                boss: Boss { hp: 13, dmg: 8 },
                effects: Effects {
                    poison: 6,
                    ..Effects::default()
                },
            }
        );

        game = boss_turn(game);
        assert_eq!(
            game,
            Game {
                player: Player {
                    hp: 2,
                    armor: 0,
                    mana: 77
                },
                boss: Boss { hp: 10, dmg: 8 },
                effects: Effects {
                    poison: 5,
                    ..Effects::default()
                }
            }
        );

        game = player_turn(game, Spell::MagicMissile);
        assert_eq!(
            game,
            Game {
                player: Player {
                    hp: 2,
                    armor: 0,
                    mana: 24
                },
                boss: Boss { hp: 3, dmg: 8 },
                effects: Effects {
                    poison: 4,
                    ..Effects::default()
                }
            }
        );

        game = boss_turn(game);
        assert_eq!(
            game,
            Game {
                player: Player {
                    hp: 2,
                    armor: 0,
                    mana: 24
                },
                boss: Boss { hp: 0, dmg: 8 },
                effects: Effects {
                    poison: 3,
                    ..Effects::default()
                }
            }
        );
    }

    #[test]
    fn example2() {
        let player = Player {
            hp: 10,
            armor: 0,
            mana: 250,
        };
        let boss = Boss { hp: 14, dmg: 8 };
        let mut game = Game::new(player, boss);

        game = player_turn(game, Spell::Recharge);
        assert_eq!(
            game,
            Game {
                player: Player {
                    hp: 10,
                    armor: 0,
                    mana: 21
                },
                boss: Boss { hp: 14, dmg: 8 },
                effects: Effects {
                    recharge: 5,
                    ..Effects::default()
                },
            }
        );

        game = boss_turn(game);
        assert_eq!(
            game,
            Game {
                player: Player {
                    hp: 2,
                    armor: 0,
                    mana: 122
                },
                boss: Boss { hp: 14, dmg: 8 },
                effects: Effects {
                    recharge: 4,
                    ..Effects::default()
                }
            }
        );

        game = player_turn(game, Spell::Shield);
        assert_eq!(
            game,
            Game {
                player: Player {
                    hp: 2,
                    armor: 7,
                    mana: 110
                },
                boss: Boss { hp: 14, dmg: 8 },
                effects: Effects {
                    recharge: 3,
                    shield: 6,
                    ..Effects::default()
                }
            }
        );

        game = boss_turn(game);
        assert_eq!(
            game,
            Game {
                player: Player {
                    hp: 1,
                    armor: 7,
                    mana: 211
                },
                boss: Boss { hp: 14, dmg: 8 },
                effects: Effects {
                    recharge: 2,
                    shield: 5,
                    ..Effects::default()
                }
            }
        );

        game = player_turn(game, Spell::Drain);
        assert_eq!(
            game,
            Game {
                player: Player {
                    hp: 3,
                    armor: 7,
                    mana: 239
                },
                boss: Boss { hp: 12, dmg: 8 },
                effects: Effects {
                    recharge: 1,
                    shield: 4,
                    ..Effects::default()
                }
            }
        );

        game = boss_turn(game);
        assert_eq!(
            game,
            Game {
                player: Player {
                    hp: 2,
                    armor: 7,
                    mana: 340
                },
                boss: Boss { hp: 12, dmg: 8 },
                effects: Effects {
                    shield: 3,
                    ..Effects::default()
                }
            }
        );

        game = player_turn(game, Spell::Poison);
        assert_eq!(
            game,
            Game {
                player: Player {
                    hp: 2,
                    armor: 7,
                    mana: 167
                },
                boss: Boss { hp: 12, dmg: 8 },
                effects: Effects {
                    shield: 2,
                    poison: 6,
                    ..Effects::default()
                }
            }
        );

        game = boss_turn(game);
        assert_eq!(
            game,
            Game {
                player: Player {
                    hp: 1,
                    armor: 7,
                    mana: 167
                },
                boss: Boss { hp: 9, dmg: 8 },
                effects: Effects {
                    shield: 1,
                    poison: 5,
                    ..Effects::default()
                }
            }
        );

        game = player_turn(game, Spell::MagicMissile);
        assert_eq!(
            game,
            Game {
                player: Player {
                    hp: 1,
                    armor: 0,
                    mana: 114
                },
                boss: Boss { hp: 2, dmg: 8 },
                effects: Effects {
                    poison: 4,
                    ..Effects::default()
                }
            }
        );

        game = boss_turn(game);
        assert_eq!(
            game,
            Game {
                player: Player {
                    hp: 1,
                    armor: 0,
                    mana: 114
                },
                boss: Boss { hp: -1, dmg: 8 },
                effects: Effects {
                    poison: 3,
                    ..Effects::default()
                }
            }
        );
    }
}
