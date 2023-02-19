use itertools::Itertools;
use crate::util;
use rayon::prelude::*;

pub fn main() {
    let boss = Character {
        hit_points: 100,
        damage: 8,
        armor: 2,
    };
    let player = Character {
        hit_points: 100,
        damage: 0,
        armor: 0,
    };


    println!("Day 21 Part 1: {:?}", minimal_cost(&player, &boss));
    println!("Day 21 Part 2: {:?}", maximal_cost_for_loss(&player, &boss));
}

fn minimal_cost(player: &Character, boss: &Character) -> i32 {
    let loadouts = generate_loadouts();
    loadouts.iter().filter(|loadout| {
        let mut player = player.clone();
        for item in *loadout {
            player.equip(item);
        }
        player_wins(player, boss.clone())
    })
        .map(|loadout| loadout.iter().map(|l| l.cost).sum())
        .min().unwrap()
}

fn maximal_cost_for_loss(player: &Character, boss: &Character) -> i32 {
    let loadouts = generate_loadouts();
    loadouts.iter().filter(|loadout| {
        let mut player = player.clone();
        for item in *loadout {
            player.equip(item);
        }
        !player_wins(player, boss.clone())
    })
        .map(|loadout| loadout.iter().map(|l| l.cost).sum())
        .max().unwrap()
}

fn generate_loadouts() -> Vec<Vec<Item>> {
    let weapons = get_weapons();
    let armor = get_armor();
    let rings = get_rings();
    let mut loadouts = vec![];
    for weapons_selection in select_items_combinations(&weapons, 1, 1) {
        for armor_selection in select_items_combinations(&armor, 0, 1) {
            for ring_selection in select_items_combinations(&rings, 0, 2) {
                let mut loadout = vec![];
                loadout.extend(weapons_selection.iter().cloned());
                loadout.extend(armor_selection.iter().cloned());
                loadout.extend(ring_selection.iter().cloned());
                loadouts.push(loadout)
            }
        }
    }
    loadouts
}

fn select_items_combinations(items: &Vec<Item>, min: usize, max: usize) -> Vec<Vec<Item>> {
    (min..=max)
        .flat_map(|size| items.iter().cloned().combinations(size))
        .collect()
}

#[derive(Clone)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

fn get_weapons() -> Vec<Item> {
    vec![
        Item { name: String::from("Dagger"), cost: 8, damage: 4, armor: 0 },
        Item { name: String::from("Shortsword"), cost: 10, damage: 5, armor: 0 },
        Item { name: String::from("Warhammer"), cost: 25, damage: 6, armor: 0 },
        Item { name: String::from("Longsword"), cost: 40, damage: 7, armor: 0 },
        Item { name: String::from("Greataxe"), cost: 74, damage: 8, armor: 0 },
    ]
}

fn get_armor() -> Vec<Item> {
    vec![
        Item { name: String::from("Leather"), cost: 13, damage: 0, armor: 1 },
        Item { name: String::from("Chainmail"), cost: 31, damage: 0, armor: 2 },
        Item { name: String::from("Splintmail"), cost: 53, damage: 0, armor: 3 },
        Item { name: String::from("Bandedmail"), cost: 75, damage: 0, armor: 4 },
        Item { name: String::from("Platemail"), cost: 102, damage: 0, armor: 5 },
    ]
}

fn get_rings() -> Vec<Item> {
    vec![
        Item { name: String::from("Damage +1"), cost: 25, damage: 1, armor: 0 },
        Item { name: String::from("Damage +2"), cost: 50, damage: 2, armor: 0 },
        Item { name: String::from("Damage +3"), cost: 100, damage: 3, armor: 0 },
        Item { name: String::from("Defense +1"), cost: 20, damage: 0, armor: 1 },
        Item { name: String::from("Defense +2"), cost: 40, damage: 0, armor: 2 },
        Item { name: String::from("Defense +3"), cost: 80, damage: 0, armor: 3 },
    ]
}

#[derive(Clone)]
struct Character {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

impl Character {
    fn equip(&mut self, item: &Item) {
        self.damage += item.damage;
        self.armor += item.armor;
    }

    fn calculate_damage(&self, other: &Character) -> i32 {
        (self.damage - other.armor).max(0)
    }
}

fn player_wins(mut player: Character, mut boss: Character) -> bool {
    loop {
        // Player's turn
        let damage = player.calculate_damage(&boss);
        if damage == 0 {
            return false;
        }
        boss.hit_points -= damage;
        if boss.hit_points <= 0 {
            // Player wins
            return true;
        }

        // Boss's turn
        let boss_damage = boss.calculate_damage(&player);
        player.hit_points -= boss_damage;
        if player.hit_points <= 0 {
            // Boss wins
            return false;
        }
    }
}
